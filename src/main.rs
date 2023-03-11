use bevy::prelude::*;
use player::PlayerPlugin;

mod components;
mod player;

// region: --- Asset Constants ---

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants ---

// region: -- Game Constants ---

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion: -- Game Constants ---

// region: --- Resources ---
pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

struct GameTextures {
    player: Handle<Image>,
}
// endregion: --- Resources ---

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .insert_resource(WindowDescriptor {
            title: "Hello Bevy".to_string(),
            width: 500.0,
            height: 500.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());
    let win_size = WinSize {
        width: win_w,
        height: win_h,
    };
    commands.insert_resource(win_size);

    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
    };
    commands.insert_resource(game_textures);
}
