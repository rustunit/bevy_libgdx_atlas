use std::num::ParseIntError;

use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum LibGdxAtlasAssetError {
    /// An [IO](std::io) Error
    #[error("Could not load asset: {0}")]
    Io(#[from] std::io::Error),

    #[error("Could not load asset: {0}")]
    LoadDirect(#[from] bevy::asset::LoadDirectError),

    #[error("Parse error: {0}")]
    ParsingError(String),

    #[error("Parse Int error: {0}")]
    ParsingInt(#[from] ParseIntError),
}
