# bevy_libgdx_atlas

[![Following released Bevy versions](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://bevyengine.org/learn/quick-start/plugin-development/#main-branch-tracking)


Support loading `libgdx.atlas` files (used for sprite sheets and such) as Bevy assets.

## Usage

Add the `LibGdxAssetPlugin` to your app:

```rust
app.add_plugins(LibGdxAssetPlugin);
```

Now when you load files with the `.libgdx.atlas` extension through the asset server, or even `bevy_asset_loader`, they will load as a `LibGdxAtlasAsset` which you can then use.

## Compatible Bevy Versions

|bevy|bevy_libgdx_atlas|
|-|-|
|0.14|0.2|
|0.13|0.1|

## License

bevy_libgdx_atlas is dual-licensed under either [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0), at your option.
