pub const TICK_TIME: f32 = 0.001; // "time" for velocity jumps per tick. controls speed of player
pub const X_OFFSET: f32 = 0.0; // offset for (0,0) from center (x direction)
pub const Y_OFFSET: f32 = 0.0; // offset for (0,0) from center (y direction)
pub const VERTICAL_WINDOW_HEIGHT: f32 = 15.; // world units for height of window
pub const NUM_ARROWS_X: u32 = 11; // number of arrows in x direction
pub const NUM_ARROWS_Y: u32 = 11; // number of arrows in y direction
pub const BASE_ARROW_SCALE: f32 = 0.0005; // base scaling factor for arrows
pub const EXPECTED_MAX_ARROW_SCALE: f32 = 30.; // expected maximum scaling factor from BASE_ARROW_SCALE for arrows (Could change to dynamic system based on max arrow size in future)