use leptos::*;

use bevy::{asset::AssetServer, prelude::*, render::camera::ScalingMode, window::WindowResolution};

use the_game_of_gradients::{MainApp, MainAppProps};

use bevy::asset::LoadState;
use web_sys::console;

// use std::f32::consts::PI;
// mod constants;
// mod gradient_field;
// mod level;
// mod ui;

//use constants::{TICK_TIME, VERTICAL_WINDOW_HEIGHT, BACKGROUND_COLOR, PLAYER_SCALE};
// use constants::{
//     BACKGROUND_COLOR, MOVEMENT_SCALE_PER_SECOND, PLAYER_SCALE, VERTICAL_WINDOW_HEIGHT,
// };
use the_game_of_gradients::BACKGROUND_COLOR;

use the_game_of_gradients::{
    GameAssets, GameLoadingState, Gradient, GradientArrowPlugin, GradientOperation,
    GradientOperationState,
};

use the_game_of_gradients::{initialize_gamestate, setup, spawn_player};

use bevy_asset_loader::prelude::*;

// use ui::{ButtonXY, NewLevelText, UiPlugin};

// use level::LevelPlugin;

fn testing(asset_server: Res<AssetServer>) {
    let player_asset: Handle<Image> = asset_server.load("player.png");

    let load_state = asset_server.get_load_state(&player_asset);
    if let Some(load_state) = load_state {
        match load_state {
            LoadState::NotLoaded => console::log_1(&"not loaded".into()),
            LoadState::Loading => console::log_1(&"loading".into()),
            LoadState::Loaded => console::log_1(&"loaded".into()),
            LoadState::Failed => console::log_1(&"failed".into()),
        }
    } else {
        console::log_1(&"couldn't get player load state".into());
    }
}

// /// move player
// fn player_movement(
//     mut player: Query<(&mut Player, &mut Transform)>,
//     gradient: Query<&Gradient>,
//     simulating_state: Res<State<Simulating>>,
//     game_state: Query<&GameState>,
//     time: Res<Time>,
// ) {
//     let gradient = gradient.single(); // should be exclusively 1 gradient
//     let game_state = game_state.single(); // should be exclusively 1 game state

//     match simulating_state.current() {
//         Simulating::Simulating => {
//             // move player on if currently simulating
//             for (mut player_struct, mut transform) in player.iter_mut() {
//                 transform.translation.x += game_state.level_info[game_state.current_level as usize]
//                     .tick_time
//                     * gradient.x(transform.translation.x, transform.translation.y)
//                     * MOVEMENT_SCALE_PER_SECOND
//                     * time.delta_seconds();
//                 transform.translation.y += game_state.level_info[game_state.current_level as usize]
//                     .tick_time
//                     * gradient.y(transform.translation.x, transform.translation.y)
//                     * MOVEMENT_SCALE_PER_SECOND
//                     * time.delta_seconds();

//                 // update player struct coords
//                 player_struct.x = transform.translation.x;
//                 player_struct.y = transform.translation.y;

//                 // update player angle
//                 let angle = gradient
//                     .y(transform.translation.x, transform.translation.y)
//                     .atan2(gradient.x(transform.translation.x, transform.translation.y))
//                     - PI / 2.;
//                 transform.rotation = Quat::from_rotation_z(angle);
//             }
//         }
//         Simulating::NotSimulating => {
//             // set player to start location when not simulating
//             for (mut player_struct, mut transform) in player.iter_mut() {
//                 transform.translation.x = game_state.level_info[game_state.current_level as usize]
//                     .start_location
//                     .0 as f32;
//                 transform.translation.y = game_state.level_info[game_state.current_level as usize]
//                     .start_location
//                     .1 as f32;

//                 // update player struct coords
//                 player_struct.x = transform.translation.x;
//                 player_struct.y = transform.translation.y;
//             }
//         }
//     }
// }

fn main() {
    // leptos initialization
    // mount_to_body(|| view! { <MainApp />});

    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR)) // set background color of window/game
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#main-game-canvas".into()),
                resolution: (1280., 720.).into(),

                ..default()
            }),
            ..default()
        }))
        .add_loading_state(
            LoadingState::new(GameLoadingState::AssetLoading)
                .continue_to_state(GameLoadingState::Ready),
        )
        .add_collection_to_loading_state::<_, GameAssets>(GameLoadingState::AssetLoading)
        .add_systems(
            OnEnter(GameLoadingState::Ready),
            (setup, initialize_gamestate, spawn_player).run_if(in_state(GameLoadingState::Ready)),
        )
        .add_systems(Update, testing)
        // .add_state(Simulating::NotSimulating) // set initial simulating state
        // .add_system(player_movement)
        // .add_plugins(GradientArrowPlugin)
        // .add_plugin(UiPlugin)
        // .add_plugin(LevelPlugin)
        .run();
}

// this is useful: https://github.com/kristoff3r/yew-bevy-example
