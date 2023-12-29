use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};
use bevy_asset_loader::prelude::*;

use web_sys::console;

mod constants;
mod gradient_field;
mod player;
// mod ui;

pub use constants::BACKGROUND_COLOR;
use constants::{PLAYER_SCALE, VERTICAL_WINDOW_HEIGHT};

pub use gradient_field::{
    Gradient, GradientArrowPlugin, GradientOperation, GradientOperationState,
};

pub use player::{spawn_player, Player};

/// State variable to separate out loading of assets
#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameLoadingState {
    #[default]
    AssetLoading,
    Ready,
}

/// Collection of game assets
#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "player.png")]
    player: Handle<Image>,
}

/// Setup the game
pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();

    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(VERTICAL_WINDOW_HEIGHT);
    commands.spawn(camera_bundle);
    console::log_1(&"camera created".into());

    // spawn_player(commands, asset_server);
}
