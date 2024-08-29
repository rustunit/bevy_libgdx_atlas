use bevy::{
    asset::{AssetLoader, AsyncReadExt},
    prelude::*,
    utils::HashMap,
};

use crate::{assetformat::AssetFile, LibGdxAtlasAsset, LibGdxAtlasAssetError};

#[derive(Default)]
pub struct LibGdxAtlasAssetLoader;
impl AssetLoader for LibGdxAtlasAssetLoader {
    type Asset = LibGdxAtlasAsset;
    type Settings = ();
    type Error = LibGdxAtlasAssetError;

    fn extensions(&self) -> &[&str] {
        &["libgdx.atlas"]
    }

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut file = String::new();
            reader.read_to_string(&mut file).await?;

            let asset = AssetFile::new(file)?;

            let image: Image = load_context
                .load_direct(
                    load_context
                        .asset_path()
                        .path()
                        .parent()
                        .unwrap()
                        .join(asset.file),
                )
                .await?
                .take()
                .expect("expected image asset");

            let mut layout = TextureAtlasLayout::new_empty(asset.size.as_vec2());
            let mut files = HashMap::new();

            for frame in asset.files {
                let id = layout.add_texture(frame.bounds);
                files.insert(frame.filename, id);
            }

            let atlas = load_context.add_labeled_asset("atlas_layout".into(), layout);
            let image = load_context.add_labeled_asset("atlas_texture".into(), image);

            Ok(LibGdxAtlasAsset {
                image,
                atlas,
                files,
            })
        })
    }
}
