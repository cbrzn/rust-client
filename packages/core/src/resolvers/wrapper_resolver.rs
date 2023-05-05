use core::fmt;
use std::sync::{Arc};

use crate::{uri::Uri, wrapper::Wrapper, invoke::Invoker};

use super::{
    resolver_with_history::ResolverWithHistory,
    uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
};

pub struct WrapperResolver {
    pub uri: Uri,
    pub wrapper: Arc<dyn Wrapper>,
}

impl WrapperResolver {}

impl ResolverWithHistory for WrapperResolver {
    fn get_step_description(&self, _: &crate::uri::Uri) -> String {
        format!("Wrapper ({})", self.uri)
    }

    fn _try_resolve_uri(
        &self,
        uri: &Uri,
        _: Arc<dyn Invoker>,
        _: &mut UriResolutionContext,
    ) -> Result<UriPackageOrWrapper, crate::error::Error> {
        if uri.to_string() != self.uri.to_string() {
            Ok(UriPackageOrWrapper::Uri(uri.clone()))
        } else {
            Ok(UriPackageOrWrapper::Wrapper(
                uri.clone(),
                self.wrapper.clone(),
            ))
        }
    }
}

impl fmt::Debug for WrapperResolver {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "WrapperResolver: {}", self.uri)
  }
}