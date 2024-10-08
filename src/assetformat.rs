use std::path::PathBuf;

use bevy::prelude::*;

use crate::LibGdxAtlasAssetError;

#[derive(Debug)]
pub struct AssetFileFrame {
    pub filename: String,
    pub bounds: Rect,
}

#[derive(Debug)]
pub struct AssetFile {
    pub file: PathBuf,
    pub size: IVec2,
    pub files: Vec<AssetFileFrame>,
}

impl AssetFile {
    pub fn new(content: String) -> Result<Self, LibGdxAtlasAssetError> {
        let mut lines = content.lines();
        let file: PathBuf = lines
            .next()
            .ok_or(LibGdxAtlasAssetError::ParsingError(
                "not found: filename".into(),
            ))?
            .into();
        let size: String = lines
            .next()
            .ok_or(LibGdxAtlasAssetError::ParsingError(
                "not found: size".into(),
            ))?
            .into();
        let _repeat: String = lines
            .next()
            .ok_or(LibGdxAtlasAssetError::ParsingError(
                "not found: repeat".into(),
            ))?
            .into();

        let size = parse_size(size)?;

        let mut files: Vec<AssetFileFrame> = Vec::new();

        while let Some(filename) = lines.next() {
            let bounds: String = lines
                .next()
                .ok_or(LibGdxAtlasAssetError::ParsingError(
                    "not found: bounds".into(),
                ))?
                .into();

            let bounds = parse_bounds(bounds)?;

            files.push(AssetFileFrame {
                filename: filename.into(),
                bounds,
            })
        }

        Ok(Self { file, size, files })
    }
}

fn parse_size(size: String) -> Result<IVec2, LibGdxAtlasAssetError> {
    if !size.starts_with("size:") {
        return Err(LibGdxAtlasAssetError::ParsingError(
            "expected: 'size:'".into(),
        ));
    }

    let colon = size.find(':').ok_or(LibGdxAtlasAssetError::ParsingError(
        "expected symbol: ':'".to_string(),
    ))?;

    let comma = size.find(',').ok_or(LibGdxAtlasAssetError::ParsingError(
        "expected symbol: 'x'".to_string(),
    ))?;

    let w = size[colon.saturating_add(1)..comma].parse::<u32>()?;
    let h = size[comma.saturating_add(1)..].parse::<u32>()?;

    Ok(IVec2::new(w as i32, h as i32))
}

fn parse_bounds(size: String) -> Result<Rect, LibGdxAtlasAssetError> {
    if !size.starts_with("bounds:") {
        return Err(LibGdxAtlasAssetError::ParsingError(
            "expected: 'bounds:'".to_string(),
        ));
    }

    let colon = size.find(':').ok_or(LibGdxAtlasAssetError::ParsingError(
        "expected symbol: ':'".to_string(),
    ))?;

    let mut values = size[colon.saturating_add(1)..].split(',');

    let x = values
        .next()
        .ok_or(LibGdxAtlasAssetError::ParsingError("expected x".into()))?
        .parse::<u32>()?;
    let y = values
        .next()
        .ok_or(LibGdxAtlasAssetError::ParsingError("expected y".into()))?
        .parse::<u32>()?;
    let x2 = x + values
        .next()
        .ok_or(LibGdxAtlasAssetError::ParsingError("expected w".into()))?
        .parse::<u32>()?;
    let y2 = y + values
        .next()
        .ok_or(LibGdxAtlasAssetError::ParsingError("expected h".into()))?
        .parse::<u32>()?;

    Ok(Rect::new(x as f32, y as f32, x2 as f32, y2 as f32))
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_size() {
        let size = parse_size("size:12,14".into()).unwrap();
        assert_eq!(size, IVec2::new(12, 14));
    }

    #[test]
    fn test_parse_bounds() {
        let size = parse_bounds("bounds:1,2,10,20".into()).unwrap();
        assert_eq!(size, Rect::new(1., 2., 11., 22.));
    }
}
