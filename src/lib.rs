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
//! Regions are addressed by name: see [`LibGdxAtlasAsset::sprite`] (default-on `sprite`
//! feature), [`LibGdxAtlasAsset::texture_atlas`] and [`LibGdxAtlasAsset::frames`].

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
    ///
    /// Indices are in packer order, not name order: use [`Self::frames`] for animations.
    pub files: HashMap<String, usize>,
}

impl LibGdxAtlasAsset {
    /// Index of the region `name` inside [`Self::atlas`].
    pub fn index(&self, name: &str) -> Option<usize> {
        self.files.get(name).copied()
    }

    /// [`TextureAtlas`] selecting the region `name`, for a `Sprite` or `ImageNode`.
    pub fn texture_atlas(&self, name: &str) -> Option<TextureAtlas> {
        Some(TextureAtlas {
            layout: self.atlas.clone(),
            index: self.index(name)?,
        })
    }

    /// Ready-to-spawn [`Sprite`] showing the region `name`.
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

    /// All region names, in unspecified order.
    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.files.keys().map(String::as_str)
    }

    /// Indices of the regions named `prefix*`, ordered by name: an animation's frames.
    ///
    /// `""` yields the whole atlas. Unlike [`Self::files`], the order is not packer order.
    pub fn frames(&self, prefix: &str) -> Vec<usize> {
        let mut regions: Vec<(&str, usize)> = self
            .files
            .iter()
            .filter(|(name, _)| name.starts_with(prefix))
            .map(|(name, index)| (name.as_str(), *index))
            .collect();

        regions.sort_by(|(a, _), (b, _)| frame_key(a).cmp(&frame_key(b)).then(a.cmp(b)));

        regions.into_iter().map(|(_, index)| index).collect()
    }
}

/// Compares the trailing frame number numerically: plain ordering puts `hero_10` first.
fn frame_key(name: &str) -> (&str, u64) {
    let head = name.trim_end_matches(|c: char| c.is_ascii_digit());
    (head, name[head.len()..].parse().unwrap_or_default())
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    /// Region names mapped to indices in packer order, i.e. deliberately not name order.
    fn atlas(regions: &[(&str, usize)]) -> LibGdxAtlasAsset {
        LibGdxAtlasAsset {
            image: Handle::default(),
            atlas: Handle::default(),
            files: regions
                .iter()
                .map(|(name, index)| ((*name).to_string(), *index))
                .collect(),
        }
    }

    #[test]
    fn test_frames_are_in_name_order_not_packer_order() {
        let atlas = atlas(&[("hero_10", 0), ("hero_1", 1), ("hero_2", 2)]);

        assert_eq!(atlas.frames("hero"), vec![1, 2, 0]);
    }

    #[test]
    fn test_frames_filters_by_prefix() {
        let atlas = atlas(&[("run_1", 0), ("idle_1", 1), ("run_2", 2)]);

        assert_eq!(atlas.frames("run"), vec![0, 2]);
        assert_eq!(atlas.frames(""), vec![1, 0, 2]);
        assert_eq!(atlas.frames("missing"), Vec::<usize>::new());
    }

    #[test]
    fn test_frames_sorts_padded_and_unpadded_numbers() {
        let atlas = atlas(&[
            ("tile010", 0),
            ("tile002", 1),
            ("hero_10", 2),
            ("hero_2", 3),
        ]);

        assert_eq!(atlas.frames("tile"), vec![1, 0]);
        assert_eq!(atlas.frames("hero"), vec![3, 2]);
    }
}
