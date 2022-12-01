use bevy::{
    prelude::*,
    render::camera::ScalingMode
};

mod constants; 
mod gradient_field;
mod ui;

use constants::{TICK_TIME, VERTICAL_WINDOW_HEIGHT};

use gradient_field::{GradientArrowPlugin, Gradient};

use ui::{UiPlugin};

#[derive(Clone, Debug, Hash, PartialEq, Eq,)]
/// struct to store level information 
pub struct Level {
    pub level_number: u32, // level number
    pub start_location: (i32, i32), // starting location
    pub x_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for x dimension (String representation of function, function itself)
    pub y_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for y dimension (String representation of function, function itself)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq,)]
/// Game state. Stores relevant information about the game
struct GameState {
    pub level_info: Vec<Level>, // essentially a constant that includes all information about levels for the game
    pub simulating: bool, // whether or not the game is currently simulating
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
                    start_location: (1, 0),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)), 
                        ("3".into(), |_x, _y| 3.),
                        ("x".into(), |x, _y| x),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("y^2".into(), |_x, y| y.powf(2.)), 
                        ("3".into(), |_x, _y| 3.),
                        ("y".into(), |_x, y| y),
                        ("x".into(), |x, _y| x),
                    ],
                }
            ],
            simulating: false,
            current_level: 0,
        }
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.19, 0.19, 0.19))) // set background color of window/game
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_state(Gradient) // set initial gradient to be the identity function
        .add_state(GameState::new()) // set initial game state
        .add_state(Simulating::NotSimulating) // set initial simulating state
        .add_system(player_movement)
        .add_plugin(GradientArrowPlugin)
        .add_plugin(UiPlugin)
        .run();
} 

/// Setup the game
fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(VERTICAL_WINDOW_HEIGHT);
    commands.spawn(camera_bundle);
}

/// Create player 
fn spawn_player(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.9, 0.9, 0.9),
            custom_size: Some(Vec2::new(1., 1.)),
            ..default()
        },
        transform: Transform::from_xyz(5., 0., 0.), // set position of player
        ..default()
    })
    .insert(Player);
}


/// move player 
fn player_movement(mut player: Query<(&Player, &mut Transform)>, gradient: Res<State<Gradient>>, simulating_state: Res<State<Simulating>>, game_state: Res<State<GameState>>) {
    match simulating_state.current() {
        Simulating::Simulating => { // move player on if currently simulating 
            for (_, mut transform) in player.iter_mut() {
                transform.translation.x += TICK_TIME * gradient.current().x(transform.translation.x, transform.translation.y);
                transform.translation.y += TICK_TIME * gradient.current().y(transform.translation.x, transform.translation.y);
            }
        }, 
        Simulating::NotSimulating => { // set player to start location when not simulating
            for (_, mut transform) in player.iter_mut() {
                transform.translation.x = game_state.current().level_info[game_state.current().current_level as usize].start_location.0 as f32;
                transform.translation.y = game_state.current().level_info[game_state.current().current_level as usize].start_location.1 as f32;
            }
        },
    } 
}