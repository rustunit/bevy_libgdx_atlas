//! # Bevy Libgdx Atlas
//!
//! `bevy_libgdx_atlas` adds an asset loader for libGDX's atlas format with the exetension `.libgdx.atlas` to allow use of it as a [`LibGdxAtlasAsset`].
//!
//! ## Usage
//!
//! To use, you would add the [`LibGdxAssetPlugin`] to your app:
//!
//! ```
//! use bevy::prelude::*;
//! use bevy_libgdx_atlas::*;
//!
//! let mut app = App::new();
//! app.add_plugins(MinimalPlugins);
//! app.add_plugins(AssetPlugin::default());
//! app.add_plugins(LibGdxAssetPlugin);
//! ```
//!
//! Now when you load files with the `.libgdx.atlas` extension through the asset server, or even `bevy_asset_loader`, they will load as a [`LibGdxAtlasAsset`] which you can then use.

mod assetformat;
mod error;
mod loader;

use bevy::{platform::collections::HashMap, prelude::*};
pub use error::LibGdxAtlasAssetError;
use loader::LibGdxAtlasAssetLoader;

/// This plugin initializes the [`LibGdxAtlasAsset`], and its private loader `LibGdxAtlasAssetLoader`, so that `.libgdx.atlas` files may be loaded as assets.
pub struct LibGdxAssetPlugin;
impl Plugin for LibGdxAssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<LibGdxAtlasAsset>();
        app.init_asset_loader::<LibGdxAtlasAssetLoader>();
    }
}

/// This is an asset containing the texture atlas image, the texture atlas layout, and a map of the original file names to their corresponding indices in the texture atlas.
#[derive(Asset, TypePath, Debug)]
pub struct LibGdxAtlasAsset {
    /// The texture atlas image.
    pub image: Handle<Image>,
    /// The texture atlas layout.
    pub atlas: Handle<TextureAtlasLayout>,
    /// The map of the original file names to indices of the texture atlas.
    pub files: HashMap<String, usize>,
}
