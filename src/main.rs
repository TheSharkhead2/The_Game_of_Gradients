use bevy::{
    prelude::*,
    render::camera::ScalingMode,
    asset::AssetServer
};

mod constants; 
mod gradient_field;
mod ui;
mod level;

use constants::{TICK_TIME, VERTICAL_WINDOW_HEIGHT, BACKGROUND_COLOR, PLAYER_SCALE};

use gradient_field::{GradientArrowPlugin, Gradient, GradientOperation, GradientOperationState};

use ui::{UiPlugin};

use level::{LevelPlugin};

#[derive(Clone, Debug)]
/// struct to store level information 
pub struct Level {
    pub level_number: u32, // level number
    pub start_location: (f32, f32), // starting location
    pub end_location: (f32, f32), // ending location
    pub x_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for x dimension (String representation of function, function itself)
    pub y_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for y dimension (String representation of function, function itself)
}

#[derive(Component, Clone, Debug)]
/// Game state. Stores relevant information about the game
struct GameState {
    pub level_info: Vec<Level>, // essentially a constant that includes all information about levels for the game
    pub current_level: u32, // current level number
}

#[derive(Clone, Debug, Hash, PartialEq, Eq,)]
/// enum to store information on simulating or not
pub enum Simulating {
    Simulating,
    NotSimulating,
}

#[derive(Component)]
struct Player;

impl GameState {
    /// Initialize new player with basic level info 
    pub fn new() -> Self {
        GameState {
            level_info: vec![
                Level {
                    level_number: 0, 
                    start_location: (0., 10.),
                    end_location: (10., 4.3),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)), 
                        ("-1".into(), |_x, _y| -1.),
                        ("1".into(), |_x, _y| 1.),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("-1".into(), |_x, _y| -1.), 
                        ("1/2".into(), |_x, _y| (1./2.)),
                        ("x".into(), |x, _y| x),
                        ("y".into(), |_x, y| y),
                    ],
                }
            ],
            current_level: 0,
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
            transform: Transform::from_xyz(0., 0., 0.) // set initial position to (0,0)
                    .with_scale(Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.)) // with no scaling 
                    .with_rotation(Quat::from_rotation_z(0.)), // with no rotation
                ..default()
            
        })
        .insert(Player);
        
}


/// move player 
fn player_movement(mut player: Query<(&Player, &mut Transform)>, gradient: Query<&Gradient>, simulating_state: Res<State<Simulating>>, game_state: Query<&GameState>) {
    let gradient = gradient.single(); // should be exclusively 1 gradient
    let game_state = game_state.single(); // should be exclusively 1 game state
 
    match simulating_state.current() {
        Simulating::Simulating => { // move player on if currently simulating 
            for (_, mut transform) in player.iter_mut() {
                transform.translation.x += TICK_TIME * gradient.x(transform.translation.x, transform.translation.y);
                transform.translation.y += TICK_TIME * gradient.y(transform.translation.x, transform.translation.y);
            }
        }, 
        Simulating::NotSimulating => { // set player to start location when not simulating
            for (_, mut transform) in player.iter_mut() {
                transform.translation.x = game_state.level_info[game_state.current_level as usize].start_location.0 as f32;
                transform.translation.y = game_state.level_info[game_state.current_level as usize].start_location.1 as f32;
            }
        },
    } 
}

/// Setup game state 
fn initialize_gamestate(mut commands: Commands) {
    commands
        .spawn(GameState::new()); // spawn game state
}