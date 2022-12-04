use bevy::ecs::system::Command;
use bevy::prelude::{*};

use crate::{GameState, Player};

use crate::constants::{ENDING_LOCATION_ERROR};

fn ending_location_setup(
    mut commands: Commands
) {
    
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
    }
}