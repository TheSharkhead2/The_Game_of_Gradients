use std::f32::consts::PI;

use bevy::{
    prelude::*,
    asset::AssetServer
};

use crate::constants::{NUM_ARROWS_X, NUM_ARROWS_Y, BASE_ARROW_SCALE, VERTICAL_WINDOW_HEIGHT, EXPECTED_MAX_ARROW_SCALE, ARROW_SCALING_FUNCTION};

// This component stores the gradient field of a given level. X and Y velocities at a point.
#[derive(Component, Clone, PartialEq, Eq, Debug, Hash, Copy)]
pub struct Gradient;

impl Gradient {
    pub fn x(self, x: f32, y: f32) -> f32 {
        x.powf(3.) + y
    }
    pub fn y(self, x: f32, y: f32) -> f32 {
        4. + y
    }

    /// Get magnitude of the gradient at a point 
    pub fn magnitude(self, x: f32, y: f32) -> f32 {
        (self.x(x,y).powf(2.) + self.y(x,y).powf(2.)).sqrt()
    }
}

#[derive(Component)]
pub struct GradientArrow {
    pub x: f32, // x coordinate 
    pub y: f32, // y coordinate 
    pub scale: f32, // scaling factor for self 
    pub x_number: u32, // the "index" of the arrow in the x direction. ie, if it is the 3rd arrow in the x direction, this will be 3 
    pub y_number: u32, // the "index" of the arrow in the y direction. ie, if it is the 3rd arrow in the y direction, this will be 3
    pub angle: f32, // angle of arrow corresponding to the gradient at the point (x, y)
}

fn spawn_gradient_arrows(mut commands: Commands, asset_server: Res<AssetServer>) {
    for i in 0..NUM_ARROWS_X {
        for j in 0..NUM_ARROWS_Y {
            commands.spawn(SpriteBundle {
                texture: asset_server.load("../assets/arrow.png"),
                transform: Transform::from_xyz(0., 0., 0.) // set initial position to (0,0)
                    .with_scale(Vec3::new(BASE_ARROW_SCALE, BASE_ARROW_SCALE, 1.)) // with no scaling 
                    .with_rotation(Quat::from_rotation_z(0.)), // with no rotation
                ..default()
            })
            .insert(GradientArrow {
                x: 0.,
                y: 0.,
                scale: BASE_ARROW_SCALE*10.,
                x_number: i,
                y_number: j,
                angle: 0.,
            });
        }
    }
}

fn update_gradient_arrows(
    mut gradient_arrows: Query<(&mut GradientArrow, &mut Sprite, &mut Transform)>,
    gradient: Res<State<Gradient>>,
    wnds: Res<Windows>,
) {
    // get main window (from https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
    let wnd = wnds.get_primary().unwrap();
    
    let pixel_to_coord_scale = wnd.height() / VERTICAL_WINDOW_HEIGHT; // scale factor to convert from world coordinates to pixels

    let window_width = wnd.width() / pixel_to_coord_scale; // width of the window in world coordinates

    for (mut gradient_arrow, mut sprite, mut transform) in gradient_arrows.iter_mut() {
        let (x_number, y_number) = (gradient_arrow.x_number, gradient_arrow.y_number); // get the index of the arrow in the x and y directions

        gradient_arrow.x = (x_number as f32) * window_width/((NUM_ARROWS_X as f32)-1.) - window_width/2.; // get the x coordinate of the arrow
        gradient_arrow.y = (y_number as f32) * VERTICAL_WINDOW_HEIGHT/((NUM_ARROWS_Y as f32)-1.) - VERTICAL_WINDOW_HEIGHT/2.; // get the y coordinate of the arrow 

        gradient_arrow.scale = BASE_ARROW_SCALE * gradient.current().magnitude(gradient_arrow.x, gradient_arrow.y); // get the scaling factor for the arrow

        gradient_arrow.angle = gradient.current().y(gradient_arrow.x, gradient_arrow.y).atan2(gradient.current().x(gradient_arrow.x, gradient_arrow.y)) - 0.25*PI; // get the angle of the arrow

        sprite.color = Color::hsla(gradient_arrow.scale/(BASE_ARROW_SCALE*EXPECTED_MAX_ARROW_SCALE)*360., 1., 0.8, 1.);

        *transform = Transform::from_xyz(gradient_arrow.x, gradient_arrow.y, 0.) // set position to (x,y)
            .with_scale(Vec3::new(ARROW_SCALING_FUNCTION(gradient_arrow.scale), ARROW_SCALING_FUNCTION(gradient_arrow.scale), gradient_arrow.scale)) // edit scaling
            .with_rotation(Quat::from_rotation_z(gradient_arrow.angle)); // edit rotation

    }

}

pub struct GradientArrowPlugin; // plugin for spawning and controlling gradient arrows

/// Plugin implementation 
impl Plugin for GradientArrowPlugin {
    /// Initialization 
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_gradient_arrows);
        app.add_system(update_gradient_arrows);
    }
}