// ! Example of loading libgdx atlas as an embedded asset.

use bevy::{asset::{embedded_asset, embedded_path}, prelude::*};
use bevy_libgdx_atlas::{LibGdxAssetPlugin, LibGdxAtlasAsset};


fn main(){
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()));
    app.add_plugins(LibGdxAssetPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Update, spawn_sprite);

    // .atlas and .png need to be embedded together.
    embedded_asset!(app, "examples", "bevy_logo.libgdx.atlas");
    embedded_asset!(app, "examples", "bevy_logo.libgdx.png");

    // The trimmed path of embedded asset is often confusing, we can check it using `embedded_path!()`.
    println!("Embedded path: {:?}", embedded_path!("examples", "bevy_logo.libgdx.atlas"));
    
    app.run();
}


#[derive(Resource, Default, Debug)]
struct LogHandle(Handle<LibGdxAtlasAsset>);

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    // Add "embedded://" prefix to declare asset source. 
    commands.insert_resource(LogHandle(asset_server.load("embedded://embedded_asset/bevy_logo.libgdx.atlas")));
    commands.spawn(Camera2d);
}

fn spawn_sprite(
    atlases: Res<Assets<LibGdxAtlasAsset>>,
    handle: Res<LogHandle>,
    mut commands: Commands,
    mut is_ran: Local<bool>,
){
    if let Some(atlas) = atlases.get(handle.0.id()) && !*is_ran {
        *is_ran = true;
        commands.spawn((
            Sprite {
                image: atlas.image.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: atlas.atlas.clone(),
                    index: 0,
                }),
                ..default()
            },
            Transform::from_scale(Vec3::splat(10.))
        )); 
    }
}
