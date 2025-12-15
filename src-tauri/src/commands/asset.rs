use serde::{Deserialize, Serialize};
use crate::commands::version::{Version, VersionManifest};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assets {
    pub objects: HashMap<String, Asset>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asset {
    pub hash: String,
    pub size: u32,
}

pub async fn get_assets(version: Version) -> Assets {
    // get the deserialized data form piston-meta
    let assets = reqwest::get(version.asset_index.url)
        .await
        .expect("Failed to get version manifest")
        .json::<Assets>()
        .await
        .expect("Failed to parse version manifest");

    // return the deserialized data
    assets
}