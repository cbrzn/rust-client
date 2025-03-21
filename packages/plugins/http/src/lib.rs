use std::sync::Arc;

use async_trait::async_trait;
use mapping::{parse_request, parse_response};
use polywrap_core::{invoke::Invoker};
use polywrap_plugin::error::PluginError;
use polywrap_plugin_macro::{plugin_struct, plugin_impl};
use wrap::{module::{Module, ArgsGet, ArgsPost}, types::ResponseType};
pub mod mapping;
pub mod wrap;
use crate::wrap::wrap_info::get_manifest;

#[plugin_struct]
pub struct HttpPlugin {
}

#[plugin_impl]
#[async_trait]
impl Module for HttpPlugin {
    async fn get(
        &mut self,
        args: &ArgsGet,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, PluginError> {
        let response = parse_request(&args.url, args.request.clone(), mapping::RequestMethod::GET)
            .unwrap()
            .call()
            .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let parsed_response = parse_response(response, response_type).await?;

        Ok(Some(parsed_response))
    }

    async fn post(
        &mut self,
        args: &ArgsPost,
        _: Arc<dyn Invoker>,
    ) -> Result<Option<wrap::types::Response>, PluginError> {
        let response = parse_request(
            &args.url,
            args.request.clone(),
            mapping::RequestMethod::POST,
        )
        .unwrap()
        .call()
        .map_err(|e| PluginError::ModuleError(e.to_string()))?;

        let response_type = if let Some(r) = &args.request {
            r.response_type
        } else {
            ResponseType::TEXT
        };

        let parsed_response = parse_response(response, response_type).await?;

        Ok(Some(parsed_response))
    }
}
