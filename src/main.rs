use std::f32::consts::PI;

use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    asset::AssetServer
};

mod constants; 
mod gradient_field;
mod ui;
mod level;

//use constants::{TICK_TIME, VERTICAL_WINDOW_HEIGHT, BACKGROUND_COLOR, PLAYER_SCALE};
use constants::{VERTICAL_WINDOW_HEIGHT, BACKGROUND_COLOR, PLAYER_SCALE};

use gradient_field::{GradientArrowPlugin, Gradient, GradientOperation, GradientOperationState};

use ui::{UiPlugin, NewLevelText, ButtonXY};

use level::{LevelPlugin};

#[derive(Clone, Debug)]
/// struct to store level information 
pub struct Level {
    pub level_number: u32, // level number
    pub start_location: (f32, f32), // starting location
    pub end_location: (f32, f32), // ending location
    pub x_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for x dimension (String representation of function, function itself)
    pub y_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for y dimension (String representation of function, function itself)
    pub gas_locations: Vec<(f32, f32)>, // locations of gas stops
    pub tick_time: f32,
}

#[derive(Component, Clone, Debug)]
/// Game state. Stores relevant information about the game
pub struct GameState {
    pub level_info: Vec<Level>, // essentially a constant that includes all information about levels for the game
    pub current_level: u32, // current level number
    pub gas_collected: Vec<u32>, // number of gas stops collected as a one-hot encoded vector
}

#[derive(Clone, Debug, Hash, PartialEq, Eq,)]
/// enum to store information on simulating or not
pub enum Simulating {
    Simulating,
    NotSimulating,
}

#[derive(Component)]
pub struct Player {
    pub x: f32, // x position of player
    pub y: f32, // y position of player
}

impl GameState {
    /// Initialize new player with basic level info 
    pub fn new() -> Self {
        GameState {
            level_info: vec![
                Level {
                    level_number: 0, 
                    start_location: (-15., -15.),
                    end_location: (0., 0.),
                    x_functions: vec![
                        ("x".into(), |x, _y| x), 
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    y_functions: vec![
                        ("x".into(), |x, _y| x), 
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                        ],
                    gas_locations: Vec::new(),
                    tick_time: 0.01
                },
                Level {
                    level_number: 1, 
                    start_location: (0., 0.),
                    end_location: (3., 9.),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)), 
                        ("y^2".into(), |_x, y| y.powf(2.)),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    y_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)), 
                        ("y^2".into(), |_x, y| y.powf(2.)),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                        ],
                    gas_locations: Vec::new(),
                    tick_time: 0.01
                },
            ],
            current_level: 0,
            gas_collected: vec![0],
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR)) // set background color of window/game
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system(initialize_gamestate)
        .add_state(Simulating::NotSimulating) // set initial simulating state
        .add_system(player_movement)
        .add_plugin(GradientArrowPlugin)
        .add_plugin(UiPlugin)
        .add_plugin(LevelPlugin)
        .run();
} 

/// Setup the game
fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(VERTICAL_WINDOW_HEIGHT);
    commands.spawn(camera_bundle);
}

/// Create player 
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("../assets/player.png"),        
            transform: Transform::from_xyz(0., 0., 1.) // set initial position to (0,0)
                    .with_scale(Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.)) // with no scaling 
                    .with_rotation(Quat::from_rotation_z(0.)), // with no rotation
                ..default()
            
        })
        .insert(Player {x: 0., y: 0.}); // insert player component
        
}


/// move player 
fn player_movement(mut player: Query<(&mut Player, &mut Transform)>, gradient: Query<&Gradient>, simulating_state: Res<State<Simulating>>, game_state: Query<&GameState>) {
    let gradient = gradient.single(); // should be exclusively 1 gradient
    let game_state = game_state.single(); // should be exclusively 1 game state
 
    match simulating_state.current() {
        Simulating::Simulating => { // move player on if currently simulating 
            for (mut player_struct, mut transform) in player.iter_mut() {
                transform.translation.x += game_state.level_info[game_state.current_level as usize].tick_time * gradient.x(transform.translation.x, transform.translation.y);
                transform.translation.y += game_state.level_info[game_state.current_level as usize].tick_time * gradient.y(transform.translation.x, transform.translation.y);

                // update player struct coords
                player_struct.x = transform.translation.x;
                player_struct.y = transform.translation.y;

                // update player angle 
                let angle = gradient.y(transform.translation.x, transform.translation.y).atan2(gradient.x(transform.translation.x, transform.translation.y)) - PI/2.;
                transform.rotation = Quat::from_rotation_z(angle);
            }
        }, 
        Simulating::NotSimulating => { // set player to start location when not simulating
            for (mut player_struct, mut transform) in player.iter_mut() {
                transform.translation.x = game_state.level_info[game_state.current_level as usize].start_location.0 as f32;
                transform.translation.y = game_state.level_info[game_state.current_level as usize].start_location.1 as f32;

                // update player struct coords 
                player_struct.x = transform.translation.x;
                player_struct.y = transform.translation.y;
            }
        },
    } 
}

/// Setup game state 
fn initialize_gamestate(mut commands: Commands) {
    commands
        .spawn(GameState::new()); // spawn game state
}