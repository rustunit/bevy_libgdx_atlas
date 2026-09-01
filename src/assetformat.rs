use std::{iter::Peekable, path::PathBuf};

use bevy_math::prelude::*;

use crate::LibGdxAtlasAssetError;

#[derive(Debug)]
pub struct AssetFileRegion {
    pub name: String,
    pub bounds: URect,
}

#[derive(Debug)]
pub struct AssetFile {
    pub file: PathBuf,
    pub size: UVec2,
    pub regions: Vec<AssetFileRegion>,
}

impl AssetFile {
    pub fn new(content: String) -> Result<Self, LibGdxAtlasAssetError> {
        // Whitespace is insignificant: libGDX indents region properties and writes
        // `size: 1, 2`, while other packers write neither.
        let mut lines = content.lines().map(str::trim).peekable();

        let file: PathBuf = lines
            .next()
            .filter(|line| !line.is_empty())
            .ok_or_else(|| parsing("not found: filename"))?
            .into();

        // Header. `format`, `filter`, `pma`, ... don't affect the layout.
        let mut size = None;
        while let Some(line) = lines.peek().copied() {
            let Some((key, value)) = property(line) else {
                break;
            };
            if key == "size" {
                size = Some(parse_uvec2(value)?);
            }
            lines.next();
        }
        let size = size.ok_or_else(|| parsing("not found: size"))?;

        let mut regions = Vec::new();
        while let Some(name) = lines.next() {
            if name.is_empty() {
                // A blank line starts the next page, and we can hold only one image.
                if lines.any(|line| !line.is_empty()) {
                    return Err(LibGdxAtlasAssetError::MultiplePages);
                }
                break;
            }

            regions.push(parse_region(name, &mut lines)?);
        }

        Ok(Self {
            file,
            size,
            regions,
        })
    }
}

fn parse_region<'a>(
    name: &str,
    lines: &mut Peekable<impl Iterator<Item = &'a str>>,
) -> Result<AssetFileRegion, LibGdxAtlasAssetError> {
    let (mut bounds, mut xy, mut size) = (None, None, None);

    while let Some(line) = lines.peek().copied() {
        let Some((key, value)) = property(line) else {
            break;
        };

        match key {
            "bounds" => bounds = Some(parse_urect(value)?),
            // libGDX before 1.9.11 wrote position and size separately.
            "xy" => xy = Some(parse_uvec2(value)?),
            "size" => size = Some(parse_uvec2(value)?),
            "rotate" if !matches!(value, "false" | "0") => {
                return Err(LibGdxAtlasAssetError::RotatedRegion(name.to_string()));
            }
            // `index`, `offsets`, `orig`, `split`, ... don't affect the layout.
            _ => {}
        }

        lines.next();
    }

    let bounds = match (bounds, xy, size) {
        (Some(bounds), _, _) => bounds,
        (None, Some(xy), Some(size)) => URect::from_corners(xy, xy.saturating_add(size)),
        _ => return Err(parsing(format!("not found: bounds of region '{name}'"))),
    };

    Ok(AssetFileRegion {
        name: name.to_string(),
        bounds,
    })
}

/// Splits a `key: value` line. Region names carry no colon, so this also tells a
/// property apart from the name starting the next region.
fn property(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

fn parse_uvec2(value: &str) -> Result<UVec2, LibGdxAtlasAssetError> {
    let [x, y] = parse_numbers(value)?;
    Ok(UVec2::new(x, y))
}

fn parse_urect(value: &str) -> Result<URect, LibGdxAtlasAssetError> {
    let [x, y, width, height] = parse_numbers(value)?;
    Ok(URect::new(
        x,
        y,
        x.saturating_add(width),
        y.saturating_add(height),
    ))
}

fn parse_numbers<const N: usize>(value: &str) -> Result<[u32; N], LibGdxAtlasAssetError> {
    let mut numbers = [0; N];
    let mut values = value.split(',');

    for number in &mut numbers {
        *number = values
            .next()
            .ok_or_else(|| parsing(format!("expected {N} numbers, got '{value}'")))?
            .trim()
            .parse()?;
    }

    Ok(numbers)
}

fn parsing(message: impl Into<String>) -> LibGdxAtlasAssetError {
    LibGdxAtlasAssetError::ParsingError(message.into())
}

#[allow(clippy::unwrap_used)]
#[cfg(test)]
mod test {
    use super::*;

    use pretty_assertions::assert_eq;

    /// What libGDX itself writes: spaced values, indented properties, extra keys.
    const LIBGDX: &str = "sheet.png
size: 128, 32
format: RGBA8888
filter: Nearest, Nearest
repeat: none
tile007
  rotate: false
  bounds: 92, 2, 16, 16
  index: -1
";

    /// What the gdx-texture-packer-gui writes.
    const COMPACT: &str = "sheet.png\nsize:128,32\nrepeat:none\ntile007\nbounds:92,2,16,16\n";

    /// libGDX before 1.9.11, where position and size were separate.
    const LEGACY: &str = "sheet.png
size: 128, 32
tile007
  xy: 92, 2
  size: 16, 16
  orig: 16, 16
  offset: 0, 0
";

    fn parse(content: &str) -> AssetFile {
        AssetFile::new(content.to_string()).unwrap()
    }

    #[test]
    fn test_parses_every_dialect_the_same() {
        for content in [LIBGDX, COMPACT, LEGACY] {
            let atlas = parse(content);

            assert_eq!(atlas.file, PathBuf::from("sheet.png"));
            assert_eq!(atlas.size, UVec2::new(128, 32));
            assert_eq!(atlas.regions.len(), 1);
            assert_eq!(atlas.regions[0].name, "tile007");
            assert_eq!(atlas.regions[0].bounds, URect::new(92, 2, 108, 18));
        }
    }

    #[test]
    fn test_parses_multiple_regions() {
        let atlas =
            parse("sheet.png\nsize:4,4\nrepeat:none\na\nbounds:0,0,1,1\nb\nbounds:1,0,1,1\n");

        assert_eq!(atlas.regions.len(), 2);
        assert_eq!(atlas.regions[1].name, "b");
    }

    #[test]
    fn test_ignores_trailing_blank_lines() {
        assert_eq!(parse(&format!("{COMPACT}\n\n")).regions.len(), 1);
    }

    #[test]
    fn test_rejects_rotated_regions() {
        let error = AssetFile::new("sheet.png\nsize:4,4\na\nrotate: true\nbounds:0,0,1,1\n".into());

        assert!(matches!(
            error,
            Err(LibGdxAtlasAssetError::RotatedRegion(name)) if name == "a"
        ));
    }

    #[test]
    fn test_rejects_multiple_pages() {
        let error = AssetFile::new(format!("{COMPACT}\n{COMPACT}"));

        assert!(matches!(error, Err(LibGdxAtlasAssetError::MultiplePages)));
    }

    #[test]
    fn test_reports_a_region_without_bounds() {
        let error = AssetFile::new("sheet.png\nsize:4,4\na\nindex:-1\n".into());

        assert!(matches!(
            error,
            Err(LibGdxAtlasAssetError::ParsingError(message)) if message.contains('a')
        ));
    }

    #[test]
    fn test_reports_a_missing_size() {
        let error = AssetFile::new("sheet.png\nrepeat:none\n".into());

        assert!(matches!(
            error,
            Err(LibGdxAtlasAssetError::ParsingError(message)) if message.contains("size")
        ));
    }

    #[test]
    fn test_parses_the_bundled_example_asset() {
        let atlas = parse(include_str!("../assets/animation_sheet.libgdx.atlas"));

        assert_eq!(atlas.regions.len(), 7);
    }

    #[test]
    fn test_parse_uvec2() {
        assert_eq!(parse_uvec2("12,14").unwrap(), UVec2::new(12, 14));
        assert_eq!(parse_uvec2(" 12 , 14 ").unwrap(), UVec2::new(12, 14));
        assert!(parse_uvec2("12").is_err());
    }

    #[test]
    fn test_parse_urect() {
        assert_eq!(parse_urect("1,2,10,20").unwrap(), URect::new(1, 2, 11, 22));
    }
}
