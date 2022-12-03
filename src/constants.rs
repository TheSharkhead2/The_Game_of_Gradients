use bevy::prelude::Color;
use std::f32::consts::PI;

pub const TICK_TIME: f32 = 0.001; // "time" for velocity jumps per tick. controls speed of player
pub const VERTICAL_WINDOW_HEIGHT: f32 = 20.; // world units for height of window
pub const NUM_ARROWS_X: u32 = 11; // number of arrows in x direction
pub const NUM_ARROWS_Y: u32 = 11; // number of arrows in y direction
pub const BASE_ARROW_SCALE: f32 = 0.0005; // base scaling factor for arrows
pub const EXPECTED_MAX_ARROW_SCALE: f32 = 18.; // expected maximum scaling factor from BASE_ARROW_SCALE for arrows (Could change to dynamic system based on max arrow size in future)
pub const ARROW_SCALING_FUNCTION: fn(f32) -> f32 = |x| (BASE_ARROW_SCALE*(EXPECTED_MAX_ARROW_SCALE*0.8))*(x/(BASE_ARROW_SCALE*(EXPECTED_MAX_ARROW_SCALE*0.8)) * PI/2.).atan(); // function for scaling arrows based on magnitude of gradient in order to get a more "capped" max arrow size


// button constants
pub const NORMAL_BUTTON_COLOR: Color = Color::rgb(0.5, 0.5, 0.5);
pub const HOVERED_BUTTON_COLOR: Color = Color::rgb(0.6, 0.6, 0.6);
pub const PRESSED_BUTTON_COLOR: Color = Color::rgb(0.3, 0.3, 0.3);
pub const HOVERED_PRESSED_BUTTON_COLOR: Color = Color::rgb(0.4, 0.4, 0.4); // color for hovering over pressed button
pub const NORMAL_BUTTON_TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9); // normal color for text on buttons
pub const PRESSED_BUTTON_TEXT_COLOR: Color = Color::rgb(0.6, 0.6, 0.6); // color for text on pressed buttons
pub const BUTTONS_PER_DIMENSION: u32 = 4; // number of buttons, or function components, per dimension (x, y)
pub const BUTTON_SPACING: f32 = 10.; // spacing between buttons in pixels 
pub const BUTTON_WIDTH: f32 = 100.; // width of buttons in pixels
pub const BUTTON_HEIGHT: f32 = 60.; // height of buttons in pixels

pub const SIM_BUTTON_OFF: Color = Color::rgb(0.1, 0.8, 0.1); // color for simulate button when not simulating 
pub const SIM_BUTTON_ON: Color = Color::rgb(0.8, 0.1, 0.1); // color for simulate button when simulating
pub const SIM_BUTTON_OFF_HOVER: Color = Color::rgb(0.2, 0.9, 0.2); // color for simulate button when not simulating and hovered
pub const SIM_BUTTON_ON_HOVER: Color = Color::rgb(0.9, 0.2, 0.2); // color for simulate button when simulating and hovered