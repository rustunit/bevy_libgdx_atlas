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
    #[error("Could not load asset: {0}")]
    Io(Box<std::io::Error>),

    /// A Bevy [`LoadDirectError`](bevy::asset::LoadDirectError) that occured
    /// while loading a [`LibGdxAtlasAsset::image`](crate::LibGdxAtlasAsset::image).
    #[error("Could not load asset: {0}")]
    LoadDirect(Box<bevy::asset::LoadDirectError>),

    /// An error that occurs when parsing the
    /// content of a `.libgdx.atlas` file.
    #[error("Parse error: {0}")]
    ParsingError(String),

    /// An error that can occur when
    /// parsing the size of a `.libgdx.atlas`'s
    /// texture atlas.
    #[error("Parse Int error: {0}")]
    ParsingInt(Box<std::num::ParseIntError>),
}

impl From<std::io::Error> for LibGdxAtlasAssetError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(Box::new(value))
    }
}

impl From<bevy::asset::LoadDirectError> for LibGdxAtlasAssetError {
    fn from(value: bevy::asset::LoadDirectError) -> Self {
        Self::LoadDirect(Box::new(value))
    }
}

impl From<std::num::ParseIntError> for LibGdxAtlasAssetError {
    fn from(value: std::num::ParseIntError) -> Self {
        Self::ParsingInt(Box::new(value))
    }
}
