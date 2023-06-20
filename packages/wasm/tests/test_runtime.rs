use polywrap_core::{
    error::Error, file_reader::SimpleFileReader,
    interface_implementation::InterfaceImplementations, invoker::Invoker,
    resolution::uri_resolution_context::UriResolutionContext, uri::Uri, wrapper::Wrapper,
};
use polywrap_wasm::wasm_wrapper::WasmWrapper;
use std::{collections::HashMap, path::Path, sync::Mutex};
use wrap_manifest_schemas::deserialize::deserialize_wrap_manifest;

use polywrap_msgpack::msgpack;
use polywrap_tests_utils::helpers::get_tests_path;
use std::fs;
use std::sync::Arc;
use polywrap_core_macros::uri;

#[derive(Clone)]
struct MockInvoker {
    wrapper: WasmWrapper,
}

impl MockInvoker {
    fn new(wrapper: WasmWrapper) -> Self {
        Self { wrapper }
    }

    fn invoke_wrapper_raw(
        &self,
        wrapper: Arc<dyn Wrapper>,
        _: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        _: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        wrapper.invoke(method, args, env, Arc::new(self.clone()), None)
    }
}

impl Invoker for MockInvoker {
    fn invoke_raw(
        &self,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<&[u8]>,
        resolution_context: Option<Arc<Mutex<UriResolutionContext>>>,
    ) -> Result<Vec<u8>, Error> {
        self.clone().invoke_wrapper_raw(
            Arc::new(self.wrapper.clone()),
            uri,
            method,
            args,
            env,
            resolution_context,
        )
    }

    fn get_implementations(&self, _uri: &Uri) -> Result<Vec<Uri>, Error> {
        Ok(vec![])
    }

    fn get_interfaces(&self) -> Option<InterfaceImplementations> {
        let i = HashMap::new();
        Some(i)
    }

    fn get_env_by_uri(&self, _: &Uri) -> Option<Vec<u8>> {
        None
    }
}

#[test]
fn invoke_test() {
    let test_path = get_tests_path().unwrap();
    let path = test_path.into_os_string().into_string().unwrap();

    let module_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.wasm");
    let manifest_path = format!("{path}/subinvoke/00-subinvoke/implementations/as/wrap.info");

    let module_bytes = fs::read(Path::new(&module_path)).unwrap();
    let manifest_bytes = fs::read(Path::new(&manifest_path)).unwrap();

    let _manifest = deserialize_wrap_manifest(&manifest_bytes, None).unwrap();
    let file_reader = SimpleFileReader::new();

    let wrapper = WasmWrapper::new(module_bytes, Arc::new(file_reader));

    let mock_invoker = MockInvoker::new(wrapper);
    let result = Arc::new(mock_invoker)
        .invoke_raw(
            &uri!("ens/wrapper.eth"),
            "add",
            Some(&msgpack!({ "a": 1, "b": 1})),
            None,
            None,
        )
        .unwrap();
    assert_eq!(result, [2])
}
