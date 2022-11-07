use async_trait::async_trait;
use polywrap_manifest::{formats::WrapManifest};

use crate::{error::Error, wrapper::Wrapper};

pub struct GetManifestOptions {
    pub no_validate: bool,
}

pub struct SerializeManifestOptions {
    pub no_validate: bool,
}

#[async_trait]
pub trait WrapPackage: Send + Sync {
    async fn create_wrapper(
        &self,
    ) -> Result<Box<dyn Wrapper>, Error>;
    async fn get_manifest(
        &self,
        options: Option<GetManifestOptions>,
    ) -> Result<WrapManifest, Error>;
}
