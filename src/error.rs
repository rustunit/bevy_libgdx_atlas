use thiserror::Error;

/// Errors that can occur during parsing of
/// `.libgdx.atlas` files, or during loading of
/// [`LibGdxAtlasAsset`](crate::LibGdxAtlasAsset)s.
// The errors have an inner `Box` because
// the size of the error type is
// as big as the largest variant,
// so when the error is bubbled up
// in the call stack it can allocate
// more memory than is needed on the
// stack, if say the error is
// actually a smaller variant.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LibGdxAtlasAssetError {
    /// An [IO](std::io) Error that occured
    /// during parsing of a `.libgdx.atlas` file.
    #[error("could not load asset: {0}")]
    Io(#[from] std::io::Error),

    /// A Bevy [`LoadDirectError`](bevy::asset::LoadDirectError) that occured
    /// while loading a [`LibGdxAtlasAsset::image`](crate::LibGdxAtlasAsset::image).
    #[error("could not load asset: {0}")]
    LoadDirect(Box<bevy::asset::LoadDirectError>),

    /// An error that occurs when parsing the
    /// content of a `.libgdx.atlas` file.
    #[error("parse error: {0}")]
    ParsingError(String),

    /// An error that can occur when
    /// parsing the size of a `.libgdx.atlas`'s
    /// texture atlas.
    #[error("parse Int error: {0}")]
    ParsingInt(#[from] std::num::ParseIntError),

    /// An error that can occur if there is
    /// trouble loading the image asset of
    /// an atlas.
    #[error("missing image asset: {0}")]
    LoadingImageAsset(String),
}

impl From<bevy::asset::LoadDirectError> for LibGdxAtlasAssetError {
    fn from(value: bevy::asset::LoadDirectError) -> Self {
        Self::LoadDirect(Box::new(value))
    }
}
