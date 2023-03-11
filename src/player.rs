use crate::{
    components::{Player, Velocity},
    GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_keyboard_event_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    let bottom = -(win_size.height / 2.);
    commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. });
}

fn player_keyboard_event_system(
    mut query: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for mut velocity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            velocity.x = -1.;
        } else if keyboard_input.pressed(KeyCode::Right) {
            velocity.x = 1.;
        } else {
            velocity.x = 0.;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            velocity.y = 1.;
        } else if keyboard_input.pressed(KeyCode::Down) {
            velocity.y = -1.;
        } else {
            velocity.y = 0.;
        }
    }
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}
