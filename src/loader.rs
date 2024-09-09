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

    async fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader<'_>,
        _settings: &'a Self::Settings,
        load_context: &'a mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut file = String::new();
        reader.read_to_string(&mut file).await?;

        let asset = AssetFile::new(file)?;

        let path = load_context
                    .asset_path()
                    .path()
                    .parent()
                    .unwrap()
                    .join(asset.file);

        let image: Image = load_context
            .loader()
            .direct()
            .untyped()
            .load(path)
            .await?
            .take()
            .expect("expected image asset");

        let mut layout = TextureAtlasLayout::new_empty(asset.size.as_uvec2());
        let mut files = HashMap::new();

        for frame in asset.files {
            let id = layout.add_texture(frame.bounds.as_urect());
            files.insert(frame.filename, id);
        }

        let atlas = load_context.add_labeled_asset("atlas_layout".into(), layout);
        let image = load_context.add_labeled_asset("atlas_texture".into(), image);

        Ok(LibGdxAtlasAsset {
            image,
            atlas,
            files,
        })
    }
}
