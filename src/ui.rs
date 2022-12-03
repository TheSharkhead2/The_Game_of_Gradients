use bevy::{
    prelude::*,
    asset::AssetServer,
};

use crate::{Simulating, Gradient, GradientOperation, GameState};

use crate::constants::{
    NORMAL_BUTTON_COLOR, 
    HOVERED_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR,
    BUTTON_WIDTH,
    BUTTON_HEIGHT,
    BUTTONS_PER_DIMENSION,
    BUTTON_SPACING,
    SIM_BUTTON_OFF,
    SIM_BUTTON_ON,
    SIM_BUTTON_OFF_HOVER,
    SIM_BUTTON_ON_HOVER,
    HOVERED_PRESSED_BUTTON_COLOR
};

/// whether or not a button is for x or y
pub enum ButtonXY {
    X,
    Y,
}

#[derive(Component)]
/// information for each button allowing user to select a function
pub struct GradComponentButton {
    pub id: u32, // button id for updating purposes 
    pub xy: ButtonXY, // whether or not the button is for x or y
    pub used: bool, // whether or not the corresponding button has been added already
}

#[derive(Component)]
/// Struct to indicate button that toggles between simulating and not
pub struct SimulatingButton {
    pub toggled: bool, // whether or not the button is toggled
}

impl SimulatingButton {
    /// New method. Starts untoggled
    pub fn new() -> Self {
        Self {
            toggled: false,
        }
    }
}

fn ui_setup(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::FlexEnd, // align to bottom of screen
                align_content: AlignContent::FlexEnd, // align to bottom of screen
                align_items: AlignItems::FlexEnd, // align to bottom of screen
                flex_direction: FlexDirection::Column, // align button sets in column
                ..default()
            }, 
            ..default()
        })
        .with_children(|parent| {
            parent // x buttons 
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.), Val::Px(BUTTON_HEIGHT + BUTTON_SPACING)), // size of button set taking up half the screen
                        margin: UiRect::all(Val::Px(BUTTON_SPACING)),
                        flex_direction: FlexDirection::Row, // align buttons in row
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // spawn buttons 
                    for i in 0..BUTTONS_PER_DIMENSION {
                        parent 
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)), // size of button
                                    margin: UiRect::all(Val::Px(BUTTON_SPACING)), // spacing between buttons
                                    justify_content: JustifyContent::Center, // center text
                                    align_items: AlignItems::Center, // center text
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(TextBundle::from_section(
                                        format!("x {}", i), // placeholer text to update later when start simulating frames
                                        TextStyle {
                                            font: asset_server.load("../assets/fonts/tahoma.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                            })
                            .insert(GradComponentButton {
                                id: i,
                                xy: ButtonXY::X,
                                used: false,
                            });
                    }
                });
            parent // y buttons
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.), Val::Px(BUTTON_HEIGHT + BUTTON_SPACING)), // size of button set taking up half the screen
                        margin: UiRect::all(Val::Px(BUTTON_SPACING)),
                        flex_direction: FlexDirection::Row, // align buttons in row
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // spawn buttons 
                    for i in 0..BUTTONS_PER_DIMENSION {
                        parent 
                            .spawn(ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)), // size of button
                                    margin: UiRect::all(Val::Px(BUTTON_SPACING)), // spacing between buttons
                                    justify_content: JustifyContent::Center, // center text
                                    align_items: AlignItems::Center, // center text
                                    ..default()
                                },
                                background_color: NORMAL_BUTTON_COLOR.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(TextBundle::from_section(
                                        format!("y {}", i), // placeholer text to update later when start simulating frames
                                        TextStyle {
                                            font: asset_server.load("../assets/fonts/tahoma.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                            })
                            .insert(GradComponentButton {
                                id: i,
                                xy: ButtonXY::Y,
                                used: false,
                            });
                    }
                });
        });

    // place button to toggle simulating
    commands 
        .spawn( 
            ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect { // place in top corner 
                        left: Val::Px(BUTTON_SPACING+BUTTON_WIDTH/2.),
                        top: Val::Px(BUTTON_SPACING+BUTTON_HEIGHT/2.),
                        ..default()
                    },
                    ..default()
                },
                background_color: SIM_BUTTON_OFF.into(),
                ..default()
            }
        )
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Simulate",
                TextStyle {
                    font: asset_server.load("../assets/fonts/tahoma.ttf"),
                    font_size: 20.0, 
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        })
        .insert(SimulatingButton::new()); // add button

}

fn simulating_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &mut SimulatingButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut simulating_state: ResMut<State<Simulating>>,
) {
    for (interaction, mut color, children, mut simulating_button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap(); // get text of button
        match *interaction {
            Interaction::Clicked => {
                if simulating_button.toggled { // if currently simulating 
                    simulating_button.toggled = false; // untoggle button 
                    simulating_state.set(Simulating::NotSimulating).unwrap(); // stop simulating 

                    text.sections[0].value = "Simulate".to_string(); // change text of button
                    *color = SIM_BUTTON_OFF.into();
                } else { // when not toggled already
                    simulating_button.toggled = true; // toggle button 
                    simulating_state.set(Simulating::Simulating).unwrap(); // start simulating 

                    text.sections[0].value = "Stop".to_string(); // change text of button
                    *color = SIM_BUTTON_ON.into();
                }
            },
            Interaction::Hovered => { // on hover slightly change color 
                if simulating_button.toggled {
                    *color = SIM_BUTTON_ON_HOVER.into();
                } else {
                    *color = SIM_BUTTON_OFF_HOVER.into();
                }
            },
            Interaction::None => { // on no interaction change color back to normal 
                if simulating_button.toggled {
                    *color = SIM_BUTTON_ON.into();
                } else {
                    *color = SIM_BUTTON_OFF.into();
                }
            }
        }
    }
}

fn grad_component_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &mut GradComponentButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut gradient: Query<&mut Gradient>,
    game_state: Query<&GameState>,
) {
    let mut gradient = gradient.single_mut(); // get gradient
    let game_state = game_state.single(); // get game state

    for (interaction, mut color, children, mut button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap(); // get text of button
        match *interaction {
            Interaction::Clicked => {
                if button.used { // if button is already used 
                    // remove corresponding function from gradient 
                    match button.xy {
                        ButtonXY::X => {
                            gradient.remove_x_function(button.id);
                        },
                        ButtonXY::Y => {
                            gradient.remove_y_function(button.id);
                        }
                    }

                    *color = NORMAL_BUTTON_COLOR.into(); // change color back to normal 
                    button.used = false; // set button to unused 
                } else { // if button is not used 
                    // add corresponding function to gradient 
                    match button.xy {
                        ButtonXY::X => {
                            // add function to gradient 
                            gradient.add_x_function(
                                button.id, // take button id as function id
                                GradientOperation::Add, // TEMP, just set to add
                                game_state.level_info[game_state.current_level as usize].x_functions[button.id as usize].1, // get function
                                game_state.level_info[game_state.current_level as usize].x_functions[button.id as usize].0.clone() // get string representing function
                            );

                        },
                        ButtonXY::Y => {
                            // add function to gradient 
                            gradient.add_y_function(
                                button.id, // take button id as function id
                                GradientOperation::Add, // TEMP, just set to add
                                game_state.level_info[game_state.current_level as usize].y_functions[button.id as usize].1, // get function
                                game_state.level_info[game_state.current_level as usize].y_functions[button.id as usize].0.clone() // get string representing function
                            );
                        },
                    }

                    *color = PRESSED_BUTTON_COLOR.into(); // change color to pressed 
                    button.used = true; // set button to used 
                }
            },
            Interaction::Hovered => {
                if button.used {
                    *color = HOVERED_PRESSED_BUTTON_COLOR.into(); // if pressed, different color change on hover then when not pressed 
                } else {
                    *color = HOVERED_BUTTON_COLOR.into(); // color change to indicate hover
                }
            },
            Interaction::None => {
                // on every None interaction, update button text (instead of extra function)
                match button.xy {
                    ButtonXY::X => {
                        text.sections[0].value = game_state.level_info[game_state.current_level as usize].x_functions[button.id as usize].0.clone(); // get string representing function
                    },
                    ButtonXY::Y => {
                        text.sections[0].value = game_state.level_info[game_state.current_level as usize].y_functions[button.id as usize].0.clone(); // get string representing function
                    },
                }
                
                if button.used {
                    *color = PRESSED_BUTTON_COLOR.into(); // if function has been added, changed to toggled color 
                } else {
                    *color = NORMAL_BUTTON_COLOR.into(); // if not currently toggled, change to normal color
                }
                
            },
        }
    }
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system(grad_component_button_system);
        app.add_system(simulating_button_system);
    }
}