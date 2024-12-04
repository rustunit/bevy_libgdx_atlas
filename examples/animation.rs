use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResolution};
use bevy_libgdx_atlas::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy LibGDX Atlas Animation Example".to_string(),
                    // So that the resolution matches the sprites aspect ratio.
                    resolution: WindowResolution::new(512., 512.),
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
        SpriteBundle {
            texture: animation_sheet.image.clone(),
            ..default()
        },
        TextureAtlas {
            layout: animation_sheet.atlas.clone(),
            index: 0,
        },
        AnimationConfig {
            first_index: 0,
            last_index: animation_sheet.files.len().saturating_sub(1),
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        },
    ));

    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 16.,
        height: 16.,
    };
    commands.spawn(camera);
}

#[derive(Component)]
struct AnimationConfig {
    first_index: usize,
    last_index: usize,
    timer: Timer,
}

fn animate_sheet(time: Res<Time>, mut query: Query<(&mut AnimationConfig, &mut TextureAtlas)>) {
    for (mut config, mut atlas) in &mut query {
        config.timer.tick(time.delta());

        // Early return if it isn't time for the next
        // step or frame in the animation.
        if !config.timer.just_finished() {
            return;
        }

        // Restart the animation to the first index if
        // it has reached the last index.
        atlas.index = if atlas.index == config.last_index {
            config.first_index
        } else {
            atlas.index + 1
        };
    }
}
