mod assetformat;
mod error;
mod loader;

use bevy::{prelude::*, utils::HashMap};
pub use error::LibGdxAtlasAssetError;
use loader::LibGdxAtlasAssetLoader;

pub struct LibGdxAssetPlugin;
impl Plugin for LibGdxAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LibGdxAtlasAsset>();
        app.init_asset_loader::<LibGdxAtlasAssetLoader>();
    }
}

/// Asset bundling together the texture atlas with its layout information and file mapping loading from a libgdx atlas file
#[derive(Asset, TypePath, Debug)]
pub struct LibGdxAtlasAsset {
    /// texture atlas image
    pub image: Handle<Image>,
    /// the atlas layout
    pub atlas: Handle<TextureAtlasLayout>,
    /// the original individual files mapped to their layout index
    pub files: HashMap<String, usize>,
}
