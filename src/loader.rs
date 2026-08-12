use bevy_asset::{AssetLoader, AsyncReadExt};
use bevy_image::prelude::*;
use bevy_platform::{collections::HashMap, prelude::*};
use bevy_reflect::prelude::*;

use crate::{LibGdxAtlasAsset, LibGdxAtlasAssetError, assetformat::AssetFile};

#[derive(Default, TypePath)]
pub struct LibGdxAtlasAssetLoader;

impl AssetLoader for LibGdxAtlasAssetLoader {
    type Asset = LibGdxAtlasAsset;
    type Settings = ();
    type Error = LibGdxAtlasAssetError;

    fn extensions(&self) -> &[&str] {
        &["libgdx.atlas"]
    }

    async fn load(
        &self,
        reader: &mut dyn bevy_asset::io::Reader,
        _settings: &(),
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut file = String::new();
        reader.read_to_string(&mut file).await?;

        let asset = AssetFile::new(file)?;

        let path = load_context
            .path()
            .path()
            .parent()
            .ok_or(LibGdxAtlasAssetError::LoadingImageAsset(
                "can't find parent folder common to atlas and image asset".to_string(),
            ))?
            .join(asset.file);

        let image: Image = load_context
            .load_builder()
            .load_untyped_value(path)
            .await?
            .take()
            .ok_or(LibGdxAtlasAssetError::LoadingImageAsset(
                "failed to load image asset, does it exist".to_string(),
            ))?;

        let mut layout = TextureAtlasLayout::new_empty(asset.size.as_uvec2());
        let mut files = HashMap::new();

        for frame in asset.files {
            let id = layout.add_texture(frame.bounds.as_urect());
            files.insert(frame.filename, id);
        }

        let atlas = load_context.add_labeled_asset("atlas_layout", layout);
        let image = load_context.add_labeled_asset("atlas_texture", image);

        Ok(LibGdxAtlasAsset {
            image,
            atlas,
            files,
        })
    }
}
