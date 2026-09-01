# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.8.0] - 2026-09-01

### Added
* `LibGdxAtlasAsset::index`, `::texture_atlas` and `::names` to look regions up by their atlas name
* `LibGdxAtlasAsset::sprite` returning a ready-to-spawn `Sprite`, behind the new default-on `sprite` feature
* `LibGdxAtlasAsset::frames` returning an animation's frame indices in name order, with trailing numbers sorted numerically (`hero_2` before `hero_10`)
* `LibGdxAtlasAsset::image_node` returning a ready-to-spawn `ImageNode`, behind the new default-on `ui` feature
* `LibGdxAtlasAsset::apply` pointing an existing `Sprite`'s or `ImageNode`'s `texture_atlas` at another region

### Changed
* renamed `LibGdxAtlasAsset::files` to `::regions`, which is what libGDX calls them
* `LibGdxAtlasAsset` derives `Reflect` and is registered by the plugin, so it shows up in inspectors
* dropped the unused `bevy_camera`, `bevy_ecs` and `bevy_internal` dependencies

### Fixed
* parse whitespace, extra keys and the pre-1.9.11 `xy:` + `size:` format, as written by libGDX itself
* report rotated regions and multi page atlases as errors instead of failing to parse

## [0.7.0] - 2026-08-18

### Changed
* upgrade to bevy `0.19`

## [0.6.0] - 2026-01-15

### Changed
* upgrade to bevy `0.18`
* switch to granular `bevy_*` dependencies

## [0.5.0] - 2025-11-15

### Changed
* upgrade to bevy `0.17`

## [0.4.0] - 2025-04-26

### Changed
* upgrade to bevy `0.16`

## [0.3.2] - 2024-12-12

### Changed
* better workaround to enable `bevy_image`

## [0.3.1] - 2024-12-08

### Changed
* fix building for `wasm`

## [0.3.0] - 2024-12-04

### Changed
* upgrade to bevy `0.15`
