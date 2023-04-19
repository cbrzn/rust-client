use std::sync::{Arc, Mutex};

use polywrap_client::core::{resolvers::{uri_resolver_like::UriResolverLike}, client::UriRedirect, uri::Uri};
use polywrap_plugin::{package::PluginPackage, wrapper::PluginWrapper};
use polywrap_wasm::{wasm_package::WasmPackage, wasm_wrapper::WasmWrapper};

use crate::utils::{get_string_from_cstr_ptr, instantiate_from_ptr};

use super::uri_resolver::SafeUriResolversVariant;

#[repr(C)]
struct Redirect {
  from: *const std::ffi::c_char,
  to: *const std::ffi::c_char
}

#[repr(C)]
pub enum SafeUriResolverLikeType {
  Resolver,
  Redirect,
  WasmPackage,
  PluginPackage,
  WasmWrapper,
  PluginWrapper,
}

#[repr(C)]
pub struct SafeUriResolverLikeVariant {
  _type: SafeUriResolverLikeType,
  data: *mut std::ffi::c_void,
  uri: *const std::ffi::c_char
}

impl From<SafeUriResolverLikeVariant> for UriResolverLike {
    fn from(value: SafeUriResolverLikeVariant) -> Self {
      let data = instantiate_from_ptr(value.data as *mut SafeUriResolverLikeVariant);
      let uri: Uri = get_string_from_cstr_ptr(data.uri).try_into().unwrap();
  
      match data._type {
        SafeUriResolverLikeType::Resolver => {
          let uri_resolver_variant = instantiate_from_ptr(value.data as *mut SafeUriResolversVariant);
          UriResolverLike::Resolver(uri_resolver_variant.into())
        },
        SafeUriResolverLikeType::Redirect => {
          let redirect = instantiate_from_ptr(value.data as *mut Redirect);
          let from = get_string_from_cstr_ptr(redirect.from).try_into().unwrap();
          let to = get_string_from_cstr_ptr(redirect.to).try_into().unwrap();
          UriResolverLike::Redirect(UriRedirect::new(from, to))
        },
        SafeUriResolverLikeType::WasmPackage => {
          let package = instantiate_from_ptr(value.data as *mut WasmPackage);
          UriResolverLike::Package(uri, Arc::new(Mutex::new(Box::new(package))))
        },
        SafeUriResolverLikeType::PluginPackage => {
          let package = instantiate_from_ptr(value.data as *mut PluginPackage);
          UriResolverLike::Package(uri, Arc::new(Mutex::new(Box::new(package))))
        },
        SafeUriResolverLikeType::WasmWrapper => {
          let wrapper = instantiate_from_ptr(value.data as *mut WasmWrapper);
          UriResolverLike::Wrapper(uri, Arc::new(Mutex::new(Box::new(wrapper))))
        },
        SafeUriResolverLikeType::PluginWrapper => {
          let wrapper = instantiate_from_ptr(value.data as *mut PluginWrapper);
          UriResolverLike::Wrapper(uri, Arc::new(Mutex::new(Box::new(wrapper))))
        }
      }
    }
}
