use bevy::prelude::Color;

pub const PLAYER_SCALE: f32 = 0.05;
pub const PORTAL_SCALE: f32 = 0.1;

pub const BACKGROUND_COLOR: Color = Color::rgb(0.19, 0.19, 0.19);

pub const FIELD_SCALE: f32 = 100.; // scalar value to increase field strength by **CHANGE THIS INSTEAD OF TICK TIME**
pub const TICK_TIME: f32 = 0.001; // "time" for velocity jumps per tick. controls speed of player
pub const VERTICAL_WINDOW_HEIGHT: f32 = 20.; // world units for height of window
pub const NUM_ARROWS_X: u32 = 11; // number of arrows in x direction
pub const NUM_ARROWS_Y: u32 = 11; // number of arrows in y direction
pub const BASE_ARROW_SCALE: f32 = 0.001; // base scaling factor for arrows
pub const EXPECTED_MAX_ARROW_SCALE: f32 = 10.; // expected maximum scaling factor from BASE_ARROW_SCALE for arrows (Could change to dynamic system based on max arrow size in future)
pub const ENDING_LOCATION_ERROR: f32 = 0.1; // error allowed for ending location

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

pub const NEW_LEVEL_TEXT_FADE_IN_SPEED: f32 = 0.03; // speed at which new level text fades in
pub const LEVEL_COMPLETE_TEXT_COLOR: (f32, f32, f32) = (0.4, 0.8, 0.4); // rgb values for level complete text