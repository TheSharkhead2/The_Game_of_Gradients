use bevy::{
    prelude::*,
    render::camera::ScalingMode
};

mod constants; 
mod gradient_field;

use constants::{TICK_TIME, VERTICAL_WINDOW_HEIGHT};

use gradient_field::{GradientArrowPlugin, Gradient};


#[derive(Component)]
struct Player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.19, 0.19, 0.19))) // set background color of window/game
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_state(Gradient) // set initial gradient to be the identity function
        .add_system(player_movement)
        .add_plugin(GradientArrowPlugin)
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
fn player_movement(mut player: Query<(&Player, &mut Transform)>, gradient: Res<State<Gradient>>) {
    for (_, mut transform) in player.iter_mut() {
        transform.translation.x += TICK_TIME * gradient.current().x(transform.translation.x, transform.translation.y);
        transform.translation.y += TICK_TIME * gradient.current().y(transform.translation.x, transform.translation.y);
    }
}