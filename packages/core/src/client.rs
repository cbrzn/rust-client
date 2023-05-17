use std::collections::HashMap;
use std::sync::{Arc};

use crate::error::Error;
use crate::invoker::Invoker;
use crate::resolvers::uri_resolution_context::UriResolutionContext;
use crate::uri::Uri;
use crate::interface_implementation::InterfaceImplementations;
use crate::resolvers::uri_resolver::{UriResolverHandler, UriResolver};
use crate::wrapper::Wrapper;

#[derive(Clone,Debug)]
pub struct UriRedirect {
  pub from: Uri,
  pub to: Uri,
}

impl UriRedirect {
  pub fn new(from: Uri, to: Uri) -> Self {
    Self { from, to }
  }
}

#[derive(Debug)]
pub struct ClientConfig {
  pub resolver: Arc<dyn UriResolver>,
  pub envs: Option<HashMap<String, Vec<u8>>>,
  pub interfaces: Option<InterfaceImplementations>
}

pub trait Client: Invoker + UriResolverHandler {
  fn get_env_by_uri(&self, uri: &Uri) -> Option<&[u8]>;
  fn load_wrapper(
    &self,
    uri: &Uri,
    resolution_context: Option<&mut UriResolutionContext>,
  ) -> Result<Arc<dyn Wrapper>, Error>;
  fn invoke_wrapper_raw(
    &self,
    wrapper: Arc<dyn Wrapper>,
    uri: &Uri,
    method: &str,
    args: Option<&[u8]>,
    env: Option<&[u8]>,
    resolution_context: Option<&mut UriResolutionContext>,
) -> Result<Vec<u8>, Error>;
}
