use bevy::{
    prelude::*,
    asset::AssetServer,
};

use crate::{GameState, Player};

use crate::constants::{ENDING_LOCATION_ERROR, PORTAL_SCALE};

#[derive(Component)]
/// struct to label ending location sprite 
pub struct EndingLocation;

/// load ending location sprite and place in world 
fn ending_location_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands 
        .spawn(SpriteBundle {
            texture: asset_server.load("../assets/portal.png"),
            transform: Transform::from_xyz(0., 0., 0.)
                .with_scale(Vec3::splat(PORTAL_SCALE)),
            ..default()
        })
        .insert(EndingLocation);
}

/// update ending location sprite position 
fn ending_location_update(
    mut query: Query<&mut Transform, With<EndingLocation>>,
    game_state: Query<&GameState>,
) {
    let game_state = game_state.single(); // get game state

    // always set to ending location for current level 
    for mut transform in query.iter_mut() {
        transform.translation.x = game_state.level_info[game_state.current_level as usize].end_location.0 as f32;
        transform.translation.y = game_state.level_info[game_state.current_level as usize].end_location.1 as f32;
    }
}

fn level_update_system(
    mut player: Query<(&Player, &Transform)>,
    mut game_state: Query<&mut GameState>,
) {
    let (_, player_transform) = player.single_mut(); // should be exclusively 1 player

    let mut game_state = game_state.single_mut();

    let distance_from_end_x = game_state.level_info[game_state.current_level as usize].end_location.0 - player_transform.translation.x; 
    let distance_from_end_y = game_state.level_info[game_state.current_level as usize].end_location.1 - player_transform.translation.y;

    let distance_from_end = (distance_from_end_x.powi(2) + distance_from_end_y.powi(2)).sqrt();

    if distance_from_end < ENDING_LOCATION_ERROR { // if within allowable error from end 
        if game_state.current_level == game_state.level_info.len() as u32 - 1 { // if last level
            game_state.current_level = 0; // reset to first level
        } else {
            game_state.current_level += 1; // increment level
        }
    }
}

/// Plugin for controlling level logic 
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_update_system);
        app.add_startup_system(ending_location_setup);
        app.add_system(ending_location_update);
    }
}