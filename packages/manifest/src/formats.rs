/* eslint-disable */
/**
 * This file was automatically generated by templates/each.mustache.
 * DO NOT MODIFY IT BY HAND. Instead, modify templates/each.mustache,
 * and run build script to regenerate this file.
 */

use serde_json::Value;
use serde::{Serialize, Deserialize};

schemafy::schemafy!(
  root: PolywrapManifest010
  "schemas/0.1.0.json"
);

schemafy::schemafy!(
  root: PolywrapManifest020
  "schemas/0.2.0.json"
);


#[derive(Clone)]
pub enum AnyManifest {
  PolywrapManifest010(PolywrapManifest010),
  PolywrapManifest020(PolywrapManifest020),
}

impl AnyManifest {
  pub fn format(&self) -> String {
    match self {
      AnyManifest::PolywrapManifest010(_) => "0.1.0".to_string(),
      AnyManifest::PolywrapManifest020(_) => "0.2.0".to_string(),
    }
  }

  pub fn get_latest(&self) -> Result<PolywrapManifest, polywrap_core::error::Error> {
        match self {
            AnyManifest::PolywrapManifest020(m) => Ok(m.clone()),
            _ => Err(polywrap_core::error::Error::ManifestError(
                "Invalid manifest format".to_string(),
            )),
        }
    }

    pub fn from_json_value(value: Value) -> Self {
        match value["format"].as_str().unwrap() {
            "0.1.0" => AnyManifest::PolywrapManifest010(serde_json::from_value(value).unwrap()),
            "0.2.0" => AnyManifest::PolywrapManifest020(serde_json::from_value(value).unwrap()),
            _ => panic!("Invalid manifest format"),
        }
    }

    pub fn to_json_value(&self) -> Value {
        match self {
            AnyManifest::PolywrapManifest010(manifest) => serde_json::to_value(manifest).unwrap(),
            AnyManifest::PolywrapManifest020(manifest) => serde_json::to_value(manifest).unwrap(),
        }
    }
}

pub const LATEST_MANIFEST_FORMAT: &str = "0.2.0";
pub type PolywrapManifest = PolywrapManifest020;