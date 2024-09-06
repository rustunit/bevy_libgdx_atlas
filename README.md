# bevy_libgdx_atlas

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)
[![crates.io](https://img.shields.io/crates/v/bevy_libgdx_atlas)](https://crates.io/crates/bevy_libgdx_atlas)
[![docs.rs](https://docs.rs/bevy_libgdx_atlas/badge.svg)](https://docs.rs/bevy_libgdx_atlas)



Support loading `libgdx.atlas` files (used for sprite sheets and such) as Bevy assets.

## Usage

Add the `LibGdxAssetPlugin` to your app:

```rust
use bevy::prelude::*;
use bevy_libgdx_atlas::*;

let app = App::new();
app.add_plugins(MinimalPlugins);
app.add_plugins(AssetPlugin::default());
app.add_plugins(LibGdxAssetPlugin);
```

Now when you load files with the `.libgdx.atlas` extension through the asset server, or even `bevy_asset_loader`, they will load as a `LibGdxAtlasAsset` which you can then use.

## Compatible Bevy Versions

|bevy|bevy_libgdx_atlas|
|-|-|
|0.13|0.1|

## License

bevy_libgdx_atlas is dual-licensed under either [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.

## Contributing

[See our CONTRIBUTING.md](/CONTRIBUTING.md)
