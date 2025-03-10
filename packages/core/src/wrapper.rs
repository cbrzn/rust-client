use std::sync::Arc;

use async_trait::async_trait;

use crate::{error::Error, invoke::{Invoker}, uri::Uri, resolvers::uri_resolution_context::UriResolutionContext, env::Env};
pub enum Encoding {
    Base64,
    UTF8,
}

pub struct GetFileOptions {
    pub path: String,
    pub encoding: Option<Encoding>,
}

#[async_trait]
pub trait Wrapper: Send + Sync {
    async fn invoke(
        &mut self,
        invoker: Arc<dyn Invoker>,
        uri: &Uri,
        method: &str,
        args: Option<&[u8]>,
        env: Option<Env>,
        resolution_context: Option<&mut UriResolutionContext>,
    ) -> Result<Vec<u8>, Error>;
    async fn get_file(&self, options: &GetFileOptions) -> Result<Vec<u8>, Error>;
}
