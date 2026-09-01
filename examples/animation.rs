#![allow(clippy::unwrap_used, reason = "keeps the example terse")]

use bevy::{prelude::*, window::WindowResolution};
use bevy_libgdx_atlas::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy LibGDX Atlas Animation Example".to_string(),
                    // So that the resolution matches the sprites aspect ratio.
                    resolution: WindowResolution::new(512, 512),
                    ..default()
                }),
                ..default()
            })
            // To allow the pixel art to be clear.
            .set(ImagePlugin::default_nearest()),
    );
    // Needed to initialize the basics of the LibGdxAssets.
    app.add_plugins(LibGdxAssetPlugin);
    app.init_state::<ExampleState>();
    app.init_resource::<AnimationSheetHandle>();
    // This, and the check_if_loaded system, are to allow for
    // the assets to be loaded, since we can't use them immediately.
    app.add_systems(OnEnter(ExampleState::Loading), load_assets);
    app.add_systems(
        Update,
        check_if_loaded.run_if(in_state(ExampleState::Loading)),
    );
    app.add_systems(OnEnter(ExampleState::Loaded), setup);
    app.add_systems(Update, animate_sheet.run_if(in_state(ExampleState::Loaded)));
    app.run();
}

#[derive(States, Hash, Debug, Default, Clone, Eq, PartialEq)]
enum ExampleState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Resource, Default, Debug)]
struct AnimationSheetHandle(Handle<LibGdxAtlasAsset>);

fn load_assets(mut handle: ResMut<AnimationSheetHandle>, asset_server: Res<AssetServer>) {
    handle.0 = asset_server.load("animation_sheet.libgdx.atlas");
}

fn check_if_loaded(
    handle: Res<AnimationSheetHandle>,
    atlases: Res<Assets<LibGdxAtlasAsset>>,
    mut next_state: ResMut<NextState<ExampleState>>,
) {
    if atlases.get(&handle.0).is_some() {
        next_state.set(ExampleState::Loaded);
    }
}

fn setup(
    mut commands: Commands,
    handle: Res<AnimationSheetHandle>,
    atlases: Res<Assets<LibGdxAtlasAsset>>,
) {
    // Get the LibGDX atlas asset for however we wish to use it.
    let Some(animation_sheet) = atlases.get(&handle.0) else {
        return;
    };

    commands.spawn((
        animation_sheet.sprite("tile007").unwrap(),
        AnimationConfig {
            // In name order, unlike the raw indices in `files`.
            frames: animation_sheet.frames("tile"),
            current: 0,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));

    let mut projection = OrthographicProjection::default_2d();
    projection.scaling_mode = bevy::camera::ScalingMode::Fixed {
        width: 16.,
        height: 16.,
    };
    commands.spawn((Camera2d, Projection::Orthographic(projection)));
}

#[derive(Component)]
struct AnimationConfig {
    frames: Vec<usize>,
    /// Position within `frames`, not an atlas index.
    current: usize,
    timer: Timer,
}

fn animate_sheet(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut Sprite)>) {
    for (mut config, mut sprite) in &mut query {
        let Some(texture_atlas) = &mut sprite.texture_atlas else {
            continue;
        };
        config.timer.tick(time.delta());

        if !config.timer.just_finished() {
            continue;
        }

        // Wraps back to the first frame at the end.
        config.current = config
            .current
            .saturating_add(1)
            .checked_rem(config.frames.len())
            .unwrap_or_default();

        if let Some(index) = config.frames.get(config.current) {
            texture_atlas.index = *index;
        }
    }
}
