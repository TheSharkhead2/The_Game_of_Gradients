use bevy::prelude::*;

mod game;
mod webpage;

pub use game::{
    setup, spawn_player, GameAssets, GameLoadingState, Gradient, GradientArrowPlugin,
    GradientOperation, GradientOperationState, Player, BACKGROUND_COLOR,
};
pub use webpage::{MainApp, MainAppProps};

#[derive(Clone, Debug)]
/// struct to store level information
pub struct Level {
    pub level_number: u32,                               // level number
    pub start_location: (f32, f32),                      // starting location
    pub end_location: (f32, f32),                        // ending location
    pub x_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for x dimension (String representation of function, function itself)
    pub y_functions: Vec<(String, fn(f32, f32) -> f32)>, // functions available for y dimension (String representation of function, function itself)
    pub gas_locations: Vec<(f32, f32)>,                  // locations of gas stops
    pub tick_time: f32,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
/// enum to store information on simulating or not
pub enum Simulating {
    Simulating,
    NotSimulating,
}

#[derive(Component, Clone, Debug)]
/// Game state. Stores relevant information about the game
pub struct GameState {
    pub level_info: Vec<Level>, // essentially a constant that includes all information about levels for the game
    pub current_level: u32,     // current level number
    pub gas_collected: Vec<u32>, // number of gas stops collected as a one-hot encoded vector
}

impl GameState {
    /// Initialize new player with basic level info
    pub fn new() -> Self {
        GameState {
            level_info: vec![
                Level {
                    // Linear Function
                    level_number: 0,
                    start_location: (-15., -15.),
                    end_location: (0., 0.),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("-100".into(), |_x, _y| -100.),
                        ("1".into(), |_x, _y| 1.),
                        ("x".into(), |x, _y| x),
                        ("y".into(), |_x, y| y),
                    ],
                    gas_locations: Vec::new(),
                    tick_time: 0.012,
                },
                Level {
                    level_number: 1,
                    start_location: (15., -5.),
                    end_location: (0., 9.),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)),
                        ("-3".into(), |_x, _y| -3.),
                        ("x/2".into(), |x, _y| (x / 2.)),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("10".into(), |_x, _y| 10.),
                        ("1/2".into(), |_x, _y| (1. / 2.)),
                        ("cosx".into(), |x, _y| x.cos()),
                        ("y".into(), |_x, y| y),
                    ],
                    gas_locations: Vec::new(),
                    tick_time: 0.01,
                },
                Level {
                    level_number: 2,
                    start_location: (-11.7, -14.8),
                    end_location: (14.5, 12.),
                    x_functions: vec![
                        ("y^2".into(), |_x, y| y.powf(2.)),
                        ("-3".into(), |_x, _y| -3.),
                        ("x/2".into(), |x, _y| (x / 2.)),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("10".into(), |_x, _y| 10.),
                        ("1/2".into(), |_x, _y| (1. / 2.)),
                        ("cosx".into(), |x, _y| x.cos()),
                        ("x^2".into(), |x, _y| x.powf(2.)),
                    ],
                    gas_locations: Vec::new(),
                    tick_time: 0.0001,
                },
                Level {
                    level_number: 3,
                    start_location: (0., 0.),
                    end_location: (3., 9.),
                    x_functions: vec![
                        ("y^2".into(), |_x, y| y.powf(2.)),
                        ("1".into(), |_x, _y| 1.),
                        ("x/2".into(), |x, _y| (x / 2.)),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)),
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: Vec::new(),
                    tick_time: 0.005,
                },
                Level {
                    // Spiral Level
                    level_number: 4,
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
                    gas_locations: vec![(-14., -7.5), (-10., 0.), (0., 2.1)],
                    tick_time: 0.001,
                },
                Level {
                    level_number: 5,
                    start_location: (-15., 15.),
                    end_location: (-1., -18.5),
                    x_functions: vec![
                        ("cbrt(x)".into(), |x, _y| x.cbrt()),
                        ("300".into(), |_x, _y| 300.),
                        ("x/2".into(), |x, _y| (x / 2.)),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("x/2".into(), |x, _y| x / 2.),
                        ("y".into(), |_x, y| y),
                        ("cbrt(y)".into(), |x, _y| x.cbrt()),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: vec![(-14., -16.), (-25., 5.), (-25., -5.)],
                    tick_time: 0.001,
                },
                Level {
                    level_number: 6,
                    start_location: (-15., 15.),
                    end_location: (-15., -15.),
                    x_functions: vec![
                        ("cbrt(x)".into(), |x, _y| x.cbrt()),
                        ("300".into(), |_x, _y| 300.),
                        ("x/2".into(), |x, _y| (x / 2.)),
                        ("y".into(), |_x, y| y),
                    ],
                    y_functions: vec![
                        ("x/2".into(), |x, _y| x / 2.),
                        ("y".into(), |_x, y| y),
                        ("cbrt(y)".into(), |x, _y| x.cbrt()),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: vec![(26., 0.), (0., 18.)],
                    tick_time: 0.001,
                },
                Level {
                    // Circle Function
                    level_number: 7,
                    start_location: (-10., 5.),
                    end_location: (10., 4.3),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)),
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    y_functions: vec![
                        ("x".into(), |x, _y| x),
                        ("y/2".into(), |_x, y| y / 2.),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: vec![(0., 15.)],
                    tick_time: 0.001,
                },
                Level {
                    // Circle Function
                    level_number: 8,
                    start_location: (-10., 0.),
                    end_location: (10., 0.),
                    x_functions: vec![
                        ("x^2".into(), |x, _y| x.powf(2.)),
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    y_functions: vec![
                        ("x".into(), |x, _y| x),
                        ("y/2".into(), |_x, y| y / 2.),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: vec![(0., 10.), (0., -10.)],
                    tick_time: 0.001,
                },
                Level {
                    level_number: 9,
                    start_location: (2., 0.3),
                    end_location: (0., -10.),
                    x_functions: vec![
                        ("x".into(), |x, _y| x),
                        ("y".into(), |_x, y| y),
                        ("xy".into(), |x, y| x * y),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    y_functions: vec![
                        ("x".into(), |x, _y| x),
                        ("y".into(), |_x, y| y),
                        ("1".into(), |_x, _y| 1.),
                        ("-1".into(), |_x, _y| -1.),
                    ],
                    gas_locations: vec![(2., 4.), (17., 0.)],
                    tick_time: 0.001,
                },
            ],
            current_level: 0,
            gas_collected: vec![0],
        }
    }
}

/// Setup game state
pub fn initialize_gamestate(mut commands: Commands) {
    commands.spawn(GameState::new()); // spawn game state
}
