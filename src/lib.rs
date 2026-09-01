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
//!
//! Regions are addressed by the name they carry in the atlas file:
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use bevy_libgdx_atlas::LibGdxAtlasAsset;
//! # #[cfg(feature = "sprite")]
//! # fn spawn(commands: &mut Commands, atlas: &LibGdxAtlasAsset) {
//! commands.spawn(atlas.sprite("tile007").unwrap());
//! # }
//! ```
//!
//! See [`LibGdxAtlasAsset::sprite`], [`LibGdxAtlasAsset::texture_atlas`] and
//! [`LibGdxAtlasAsset::index`]. `sprite` lives behind the default-on `sprite` feature,
//! which pulls in `bevy_sprite`; the others are always available.

mod assetformat;
mod error;
mod loader;

use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_image::prelude::*;
use bevy_platform::{collections::HashMap, prelude::*};
use bevy_reflect::prelude::*;
#[cfg(feature = "sprite")]
use bevy_sprite::prelude::*;
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

impl LibGdxAtlasAsset {
    /// Index of the region `name` inside [`Self::atlas`], or `None` if the atlas
    /// contains no such region.
    ///
    /// `name` is the region name as written in the `.libgdx.atlas` file, which is
    /// usually the packed file's name without its extension (`"tile007"`).
    pub fn index(&self, name: &str) -> Option<usize> {
        self.files.get(name).copied()
    }

    /// [`TextureAtlas`] selecting the region `name`, ready to assign to
    /// [`Sprite::texture_atlas`](https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html#structfield.texture_atlas)
    /// or [`ImageNode::texture_atlas`](https://docs.rs/bevy/latest/bevy/ui/widget/struct.ImageNode.html#structfield.texture_atlas).
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_libgdx_atlas::LibGdxAtlasAsset;
    /// # fn spawn(commands: &mut Commands, atlas: &LibGdxAtlasAsset) {
    /// commands.spawn(Sprite {
    ///     image: atlas.image.clone(),
    ///     texture_atlas: atlas.texture_atlas("tile007"),
    ///     ..default()
    /// });
    /// # }
    /// ```
    pub fn texture_atlas(&self, name: &str) -> Option<TextureAtlas> {
        Some(TextureAtlas {
            layout: self.atlas.clone(),
            index: self.index(name)?,
        })
    }

    /// A ready-to-spawn [`Sprite`] showing the region `name`.
    ///
    /// ```no_run
    /// # use bevy::prelude::*;
    /// # use bevy_libgdx_atlas::LibGdxAtlasAsset;
    /// # fn spawn(commands: &mut Commands, atlas: &LibGdxAtlasAsset) {
    /// commands.spawn(atlas.sprite("tile007").unwrap());
    /// # }
    /// ```
    #[cfg(feature = "sprite")]
    pub fn sprite(&self, name: &str) -> Option<Sprite> {
        Some(Sprite::from_atlas_image(
            self.image.clone(),
            self.texture_atlas(name)?,
        ))
    }

    /// The names of all regions in this atlas.
    ///
    /// The order is unspecified: [`Self::files`] is a hash map, and the indices behind
    /// it follow the order the packer wrote the regions in, not the region names.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(String::as_str)
    }
}
