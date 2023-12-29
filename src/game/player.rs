use crate::game::{constants::PLAYER_SCALE, GameAssets};
use bevy::{asset::LoadState, prelude::*};

use web_sys::console;

// Player!
#[derive(Component)]
pub struct Player {
    pub x: f32, // x position of player
    pub y: f32, // y position of player
}

/// Create player
pub fn spawn_player(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: game_assets.player.clone(),
            transform: Transform::from_xyz(0., 0., 1.) // set initial position to (0,0)
                .with_scale(Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.)) // with no scaling
                .with_rotation(Quat::from_rotation_z(0.)), // with no rotation
            ..default()
        })
        .insert(Player { x: 0., y: 0. }); // insert player component

    console::log_1(&"player created".into());
}
