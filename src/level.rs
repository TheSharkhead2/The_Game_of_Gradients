use bevy::{
    prelude::*,
    asset::AssetServer,
};

use crate::ui::GradComponentButton;
use crate::{GameState, Player, Simulating, Gradient, NewLevelText, ButtonXY};

use crate::constants::{ENDING_LOCATION_ERROR, PORTAL_SCALE, MAX_GAS_CANS, GAS_CAN_SCALE, NORMAL_BUTTON_COLOR, NORMAL_BUTTON_TEXT_COLOR};

#[derive(Component)]
/// struct to label ending location sprite 
pub struct EndingLocation;

#[derive(Component)]
/// Struct to label gas can 
pub struct GasCan {
    pub collected: bool, // whether gas can has been collected
    pub index: u32, // specific index of this gas can 
}

/// load ending location sprite and place in world 
fn ending_location_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands 
        .spawn(SpriteBundle {
            texture: asset_server.load("portal.png"),
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
    mut simulating_state: ResMut<State<Simulating>>,
    mut gradient: Query<&mut Gradient>,
    mut new_level_text: Query<&mut NewLevelText>,
    mut text_query: Query<&mut Text>,
    mut grad_buttons: Query<(&Children, &mut BackgroundColor, &mut GradComponentButton)>,
) {
    let (_, player_transform) = player.single_mut(); // should be exclusively 1 player

    let mut game_state = game_state.single_mut();
    let mut gradient = gradient.single_mut();
    let mut new_level_text = new_level_text.single_mut();

    let distance_from_end_x = game_state.level_info[game_state.current_level as usize].end_location.0 - player_transform.translation.x; 
    let distance_from_end_y = game_state.level_info[game_state.current_level as usize].end_location.1 - player_transform.translation.y;

    let distance_from_end = (distance_from_end_x.powi(2) + distance_from_end_y.powi(2)).sqrt();

    if full_gas(&game_state) && distance_from_end < ENDING_LOCATION_ERROR { // if within allowable error from end and collected all the gas
        match simulating_state.current() { // stop simulating on level end
            Simulating::NotSimulating => {},
            Simulating::Simulating => {
                simulating_state.set(Simulating::NotSimulating).unwrap(); // stop simulating 
            },
        }

        gradient.clear_field(); // clear gradient field

        if game_state.current_level == game_state.level_info.len() as u32 - 1 { // if last level
            game_state.current_level = 0; // reset to first level

            // update new level text to fade in and out with new level 
            new_level_text.fade_in = true;
            new_level_text.fade_out = false;
            new_level_text.level = game_state.current_level + 1;
        } else {
            game_state.current_level += 1; // increment level

            // update new level text to fade in and out with new level 
            new_level_text.fade_in = true;
            new_level_text.fade_out = false;
            new_level_text.level = game_state.current_level + 1;
        }

        for (children, mut background_color, mut grad_component_button) in grad_buttons.iter_mut() { // reset gradient buttons
            let mut text = text_query.get_mut(children[0]).unwrap();
            
            *background_color = NORMAL_BUTTON_COLOR.into(); // update button background 
            text.sections[0].style.color = NORMAL_BUTTON_TEXT_COLOR; // update button text color
            grad_component_button.used = false; // update button used

            match grad_component_button.xy {
                ButtonXY::X => {
                    text.sections[0].value = game_state.level_info[game_state.current_level as usize].x_functions[grad_component_button.id as usize].0.clone(); // update button text
                },
                ButtonXY::Y => {
                    text.sections[0].value = game_state.level_info[game_state.current_level as usize].y_functions[grad_component_button.id as usize].0.clone(); // update button text
                },
            }

        }
    }
}

/// initialize gas sprites into world and they start invisible 
pub fn gas_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for i in 0..MAX_GAS_CANS {
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("gas_can.png"),
                transform: Transform::from_xyz(0., 0., 0.)
                    .with_scale(Vec3::splat(GAS_CAN_SCALE)),
                visibility: Visibility {
                    is_visible: false, // starts invisible
                },
                ..default()
            })
            .insert(GasCan {collected: false, index: i});
    }
}

/// update gas sprite positions based on current level 
pub fn gas_update(
    mut query: Query<(&mut Transform, &mut Visibility, &mut GasCan)>,
    player: Query<&Player>,
    mut game_state: Query<&mut GameState>,
    simulating_state: Res<State<Simulating>>
) {
    let mut game_state = game_state.single_mut(); // get game state
    let player = player.single(); // get player

    if game_state.gas_collected.len() != game_state.level_info[game_state.current_level as usize].gas_locations.len() { // if gas collected is not the same length as gas locations
        game_state.gas_collected = vec![0; game_state.level_info[game_state.current_level as usize].gas_locations.len()]; // reset gas collected
    }

    match simulating_state.current() {
        Simulating::NotSimulating => {
            for (mut transform, mut visibility, mut gas_can) in query.iter_mut() {
                if gas_can.index < game_state.level_info[game_state.current_level as usize].gas_locations.len() as u32 { // if gas can is in current level
                    let gas_can_position = game_state.level_info[game_state.current_level as usize].gas_locations[gas_can.index as usize]; // get gas can position

                    transform.translation.x = gas_can_position.0 as f32; // set gas can position
                    transform.translation.y = gas_can_position.1 as f32;

                    visibility.is_visible = true; // make gas can visibile when not simulating
                    gas_can.collected = false; // reset gas can collected state
                    game_state.gas_collected[gas_can.index as usize] = 0; // reset gas can collected state
                } else {
                    visibility.is_visible = false; // make gas can invisible
                    gas_can.collected = false; // reset gas can collected state (just in case)
                }
            }
        },
        Simulating::Simulating => {
            for (mut transform, mut visibility, mut gas_can) in query.iter_mut() {
                if gas_can.index < game_state.level_info[game_state.current_level as usize].gas_locations.len() as u32 { // if gas can is in current level
                    let gas_can_position = game_state.level_info[game_state.current_level as usize].gas_locations[gas_can.index as usize]; // get gas can position

                    transform.translation.x = gas_can_position.0 as f32; // set gas can position
                    transform.translation.y = gas_can_position.1 as f32;

                    let distance_from_player_x = gas_can_position.0 as f32 - player.x; // get distance from player
                    let distance_from_player_y = gas_can_position.1 as f32 - player.y;

                    let distance_from_player = (distance_from_player_x.powi(2) + distance_from_player_y.powi(2)).sqrt();

                    if distance_from_player < ENDING_LOCATION_ERROR {
                        gas_can.collected = true; // set gas can collected state 

                        game_state.gas_collected[gas_can.index as usize] = 1; // set gas can collected state in game state

                    }

                    if gas_can.collected {
                        visibility.is_visible = false; // make gas can invisible when collected
                    } else {
                        visibility.is_visible = true; // make gas can visible when not collected
                    }

                } else {
                    visibility.is_visible = false; // make gas can invisible
                }
            }
        },
    }
}

/// internal function for whether or not the player has collected all the gas 
pub fn full_gas(
    game_state: &GameState, // raw game state 
) -> bool {
    if game_state.gas_collected.iter().sum::<u32>() == game_state.level_info[game_state.current_level as usize].gas_locations.len() as u32 { // if all gas was collected 
        return true
    } else {
        return false
    }
}

/// Plugin for controlling level logic 
pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(level_update_system);
        app.add_startup_system(ending_location_setup);
        app.add_system(ending_location_update);
        app.add_startup_system(gas_setup);
        app.add_system(gas_update);
    }
}