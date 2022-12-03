use std::f32::consts::PI;

use bevy::{
    prelude::*,
    asset::AssetServer
};

use crate::constants::{NUM_ARROWS_X, NUM_ARROWS_Y, BASE_ARROW_SCALE, VERTICAL_WINDOW_HEIGHT, EXPECTED_MAX_ARROW_SCALE};

/// This enum represents the valid operations between parts of the gradient function 
pub enum GradientOperation {
    Add,
    Multiply,
}

impl GradientOperation {
    /// initializes enum for consistency 
    pub fn new() -> Self {
        GradientOperation::Add
    }
}

/// This component stores the gradient field of a given level. X and Y velocities at a point.
#[derive(Component)]
pub struct Gradient {
    pub x_functions: Vec<(u32, GradientOperation, fn(f32, f32) -> f32, String)>, // (function id (for current level), operation to combine with previous functions, function itself, string representing function)
    pub y_functions: Vec<(u32, GradientOperation, fn(f32, f32) -> f32, String)>, // (function id (for current level), operation to combine with previous functions, function itself, string representing function)
}   

impl Gradient {
    /// Compute the x value of the current gradient function at the specified point 
    pub fn x(&self, x: f32, y: f32) -> f32 {
        if self.x_functions.len() == 0 { // if no x functions currently in gradient, evaulate to 0
            return 0.
        } else {
            let mut x_value = self.x_functions[0].2(x, y); // evaulate the first x function 
            for function in self.x_functions.iter().skip(1) { // iterate through all other x functions
                match function.1 { // match operation to combine with previous functions
                    GradientOperation::Add => x_value = x_value + function.2(x,y), // add function to previous function
                    GradientOperation::Multiply => x_value = x_value * function.2(x,y), // multiply function to previous function
                }
            }

            x_value // return computed value 
        }
    }

    /// Compute the y value of the current gradient function at the specified point 
    pub fn y(&self, x: f32, y: f32) -> f32 {
        if self.y_functions.len() == 0 { // if no y functions currently in gradient, evaulate to 0 
            return 0.
        } else {
            let mut y_value = self.y_functions[0].2(x, y); // evaulate the first y function
            for function in self.y_functions.iter().skip(1) { // iterate through all other y functions
                match function.1 { // match operation to combine with previous functions
                    GradientOperation::Add => y_value = y_value + function.2(x,y), // add function to previous function
                    GradientOperation::Multiply => y_value = y_value * function.2(x,y), // multiply function to previous function
                }
            }

            y_value // return computed value
        }
    }

    /// Add a new x function to the gradient 
    pub fn add_x_function(&mut self, function_id: u32, operation: GradientOperation, function: fn(f32, f32) -> f32, function_string: String) {
        self.x_functions.push((function_id, operation, function, function_string)); // add function to x functions
    }

    /// Add a new y function to the gradient
    pub fn add_y_function(&mut self, function_id: u32, operation: GradientOperation, function: fn(f32, f32) -> f32, function_string: String) {
        self.y_functions.push((function_id, operation, function, function_string)); // add function to y functions
    }

    /// Remove x function from gradient. If function_id is not found, do nothing 
    pub fn remove_x_function(&mut self, function_id: u32) {
        let function_pos = &self.x_functions.iter().position(|x| x.0 == function_id); // search for location of function id

        match function_pos {
            Some(pos) => {self.x_functions.remove(*pos);}, // if function found, remove it from vector
            None => (),
        }
    }

    /// Remove y function from gradient. If function_id is not found, do nothing
    pub fn remove_y_function(&mut self, function_id: u32) {
        let function_pos = &self.y_functions.iter().position(|x| x.0 == function_id); // search for location of function id

        match function_pos {
            Some(pos) => {self.y_functions.remove(*pos);}, // if function found, remove it from vector
            None => (),
        }
    }

    /// Generates text representing the current gradient function for the x direction 
    pub fn x_text(&self) -> String {
        if self.x_functions.len() == 0 { // if no x functions currently in gradient, return 0
            return String::from("0")
        } else {
            let mut x_text = self.x_functions[0].3.clone(); // get string representing first x function 
            for function in self.x_functions.iter().skip(1) { // iterate through all other x functions 
                match function.1 { // how functions are combined depends on the operation 
                    GradientOperation::Add => {x_text = format!("{} + {}", x_text, function.3);}, // add function to previous function
                    GradientOperation::Multiply => {x_text = format!("({}) * {}", x_text, function.3)}, // with multiplication, need to add parenthesis 
                }
            }
            x_text // return string representing gradient function
        }
    }

    /// Generates text representing the current gradient function for the y direction 
    pub fn y_text(&self) -> String {
        if self.y_functions.len() == 0 { // if no y functions currently in gradient, return 0
            return String::from("0")
        } else {
            let mut y_text = self.y_functions[0].3.clone(); // get string representing first y function 
            for function in self.y_functions.iter().skip(1) { // iterate through all other y functions 
                match function.1 { // how functions are combined depends on the operation 
                    GradientOperation::Add => {y_text = format!("{} + {}", y_text, function.3);}, // add function to previous function
                    GradientOperation::Multiply => {y_text = format!("({}) * {}", y_text, function.3)}, // with multiplication, need to add parenthesis 
                }
            }
            y_text // return string representing gradient function
        }
    }

    /// Get magnitude of the gradient at a point 
    pub fn magnitude(&self, x: f32, y: f32) -> f32 {
        (&self.x(x,y).powf(2.) + &self.y(x,y).powf(2.)).sqrt()
    }

    pub fn new() -> Self {
        // empty grad 
        Gradient {
            x_functions: Vec::new(),
            y_functions: Vec::new(),
        }
    }
}

/// Function for initializing gradient object
fn initialize_gradient(mut commands: Commands) {
    commands.spawn(Gradient::new());
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
    gradient: Query<&Gradient>,
    wnds: Res<Windows>,
) {
    // get main window (from https://bevy-cheatbook.github.io/cookbook/cursor2world.html)
    let wnd = wnds.get_primary().unwrap();
    
    let pixel_to_coord_scale = wnd.height() / VERTICAL_WINDOW_HEIGHT; // scale factor to convert from world coordinates to pixels

    let window_width = wnd.width() / pixel_to_coord_scale; // width of the window in world coordinates

    let gradient = gradient.single(); // should only be 1 gradient

    // precompute all of the mangitudes of the gradient at each relevant point to get the maximum magnitude 
    let mut max_magnitude = BASE_ARROW_SCALE;

    for (mut gradient_arrow, _sprite, _transform) in gradient_arrows.iter_mut() {
        let (x_number, y_number) = (gradient_arrow.x_number, gradient_arrow.y_number); // get the index of the arrow in the x and y directions

        gradient_arrow.x = (x_number as f32) * window_width/((NUM_ARROWS_X as f32)-1.) - window_width/2.; // get the x coordinate of the arrow
        gradient_arrow.y = (y_number as f32) * VERTICAL_WINDOW_HEIGHT/((NUM_ARROWS_Y as f32)-1.) - VERTICAL_WINDOW_HEIGHT/2.; // get the y coordinate of the arrow 

        gradient_arrow.scale = BASE_ARROW_SCALE * gradient.magnitude(gradient_arrow.x, gradient_arrow.y); // get the scaling factor for the arrow

        if max_magnitude < gradient_arrow.scale { // if the current magnitude is greater than the current max, update the max
            max_magnitude = gradient_arrow.scale;
        }
    }

    for (mut gradient_arrow, mut sprite, mut transform) in gradient_arrows.iter_mut() {
        gradient_arrow.scale = (EXPECTED_MAX_ARROW_SCALE*BASE_ARROW_SCALE)/max_magnitude *gradient_arrow.scale;

        gradient_arrow.angle = gradient.y(gradient_arrow.x, gradient_arrow.y).atan2(gradient.x(gradient_arrow.x, gradient_arrow.y)) - 0.25*PI; // get the angle of the arrow

        sprite.color = Color::hsla(gradient_arrow.scale/(BASE_ARROW_SCALE*EXPECTED_MAX_ARROW_SCALE)*360., 1., 0.8, 1.);

        *transform = Transform::from_xyz(gradient_arrow.x, gradient_arrow.y, 0.) // set position to (x,y)
            .with_scale(Vec3::new(gradient_arrow.scale, gradient_arrow.scale, gradient_arrow.scale)) // edit scaling
            .with_rotation(Quat::from_rotation_z(gradient_arrow.angle)); // edit rotation

    }

}

pub struct GradientArrowPlugin; // plugin for spawning and controlling gradient arrows

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
/// special state for gradient operation 
pub enum GradientOperationState {
    Add, // add operation 
    Multiply, // multiply operation 
}

/// Plugin implementation 
impl Plugin for GradientArrowPlugin {
    /// Initialization 
    fn build(&self, app: &mut App) {
        app.add_startup_system(initialize_gradient);
        app.add_startup_system(spawn_gradient_arrows);
        app.add_system(update_gradient_arrows);
        app.add_state(GradientOperationState::Add); // state for when adding new functions to gradient... operation to add with 
    }
}