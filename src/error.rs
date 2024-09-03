use std::num::ParseIntError;

use thiserror::Error;

/// Errors that can occur during parsing of
/// `.libgdx.atlas` files, or during loading of
/// [`LibGdxAtlasAsset`](crate::LibGdxAtlasAsset)s.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LibGdxAtlasAssetError {
    /// An [IO](std::io) Error that occured
    /// during parsing of a `.libgdx.atlas` file.
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A Bevy [`LoadDirectError`](bevy::asset::LoadDirectError) that occured
    /// while loading a [`LibGdxAtlasAsset::image`](crate::LibGdxAtlasAsset::image).
    #[error("Could not load asset: {0}")]
    LoadDirect(#[from] bevy::asset::LoadDirectError),

    /// An error that occurs when parsing the
    /// content of a `.libgdx.atlas` file.
    #[error("Parse error: {0}")]
    ParsingError(String),

    /// An error that can occur when
    /// parsing the size of a `.libgdx.atlas`'s
    /// texture atlas.
    #[error("Parse Int error: {0}")]
    ParsingInt(#[from] ParseIntError),
}
