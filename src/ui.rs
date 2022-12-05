use bevy::{
    prelude::*,
    asset::AssetServer,
};

use crate::{Simulating, Gradient, GradientOperation, GameState, GradientOperationState};

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
    HOVERED_PRESSED_BUTTON_COLOR,
    NORMAL_BUTTON_TEXT_COLOR,
    PRESSED_BUTTON_TEXT_COLOR,
    NEW_LEVEL_TEXT_FADE_IN_SPEED,
    LEVEL_COMPLETE_TEXT_COLOR
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

#[derive(Component)]
/// Struct to indicate gradient x function 
pub struct XGradientText; 

#[derive(Component)]
/// Struct to indicate gradient y function
pub struct YGradientText;

#[derive(Component)]
/// Struct to label text of current level 
pub struct LevelText;

#[derive(Component)]
/// struct to label level change text 
pub struct NewLevelText {
    pub alpha: f32, // alpha value for fading in and out
    pub fade_in: bool, // whether or not the text is fading in
    pub fade_out: bool, // whether or not the text is fading out
    pub level: u32, // level number
}

impl NewLevelText {
    /// New method. Starts with alpha value of 0 and not fading in or out
    pub fn new() -> Self {
        Self {
            alpha: 0.,
            fade_in: false,
            fade_out: false,
            level: 0, // start at level 0
        }
    }
}

#[derive(Component)]
/// Struct to label gas collected text
pub struct GasCollectedText;

#[derive(Component)]
pub struct OperationButton {
    pub operation: GradientOperation,
}

fn ui_setup(
    mut commands: Commands, asset_server: Res<AssetServer>
) {
    commands 
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // place simulate button and level text
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::FlexStart,
                        ..default() 
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // place button to toggle simulating
                    parent 
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Px(BUTTON_WIDTH + 2.*BUTTON_SPACING), Val::Px(BUTTON_HEIGHT + 2.*BUTTON_SPACING)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent 
                                .spawn( 
                                    ButtonBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
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
                                            font: asset_server.load("fonts/tahoma.ttf"),
                                            font_size: 20.0, 
                                            color: Color::rgb(0.9, 0.9, 0.9),
                                        },
                                    ));
                                })
                                .insert(SimulatingButton::new()); // add button
                        });

                    parent 
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.), Val::Px(BUTTON_HEIGHT + 2.*BUTTON_SPACING)),
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexStart,
                                flex_direction: FlexDirection::Row,
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(30.), Val::Percent(100.)),
                                    justify_content: JustifyContent::FlexEnd,
                                    align_items: AlignItems::FlexEnd,
                                    align_content: AlignContent::FlexEnd,
                                    flex_direction: FlexDirection::Column,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(|parent| {
                                parent 
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::FlexEnd,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent 
                                            .spawn(TextBundle::from_section(
                                                "Level: ",
                                                TextStyle {
                                                    font: asset_server.load("fonts/tahoma.ttf"),
                                                    font_size: 20.0, 
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ))
                                            .insert(LevelText);
                                    });
                                
                                
                                // add gas collected text 
                                parent 
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::FlexEnd,
                                            ..default()
                                        },
                                        visibility: Visibility { is_visible: false },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent 
                                            .spawn(TextBundle::from_section(
                                                "Gas Collected: ",
                                                TextStyle {
                                                    font: asset_server.load("fonts/tahoma.ttf"),
                                                    font_size: 20.0, 
                                                    color: Color::rgb(0.9, 0.9, 0.9),
                                                },
                                            ));
                                    })
                                    .insert(GasCollectedText);
                            });
                        });
                });

            // level complete text
            parent 
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(20.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent 
                        .spawn(TextBundle::from_section(
                            "Level ",
                            TextStyle {
                                font: asset_server.load("fonts/tahoma.ttf"),
                                font_size: 80.0, 
                                color: Color::rgba(0.7, 0.9, 0.7, 0.),
                            },
                        ))
                        .insert(NewLevelText::new());

                });

            // x and y components text and add/multiply button
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                        justify_content: JustifyContent::SpaceAround, // align to bottom of screen
                        align_content: AlignContent::SpaceAround, // align to bottom of screen
                        align_items: AlignItems::FlexEnd, // align to bottom of screen
                        flex_direction: FlexDirection::Row, // align button sets in column
                        ..default()
                    }, 
                    ..default()
                })
                .with_children(|parent| {
                    parent // x and y gradient text 
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.), Val::Px(2. * BUTTON_HEIGHT + 3. * BUTTON_SPACING)), // size of button set
                                flex_direction: FlexDirection::Row, // stack button sets in column
                                justify_content: JustifyContent::SpaceBetween, // align to center of height 
                                align_items: AlignItems::Center, // align to right
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent 
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(75.), Val::Percent(100.)),
                                        flex_direction: FlexDirection::Column, // stack button sets in column
                                        justify_content: JustifyContent::Center, // align to center of height 
                                        align_items: AlignItems::Center, // align to right
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent 
                                        .spawn( NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.), Val::Px(BUTTON_HEIGHT)), // size of button
                                                justify_content: JustifyContent::Center,
                                                align_content: AlignContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent
                                                .spawn(TextBundle::from_section(
                                                        "x = ",
                                                        TextStyle {
                                                            font: asset_server.load("fonts/tahoma.ttf"),
                                                            font_size: 30.,
                                                            color: Color::rgb(0.9, 0.9, 0.9)
                                                        }
                                                ))
                                                .insert(XGradientText); // insert label that this is for the x component of the gradient 
                                        });
                                    
                                    parent 
                                        .spawn( NodeBundle {
                                            style: Style {
                                                size: Size::new(Val::Percent(100.), Val::Px(BUTTON_HEIGHT)), // size of button
                                                justify_content: JustifyContent::Center,
                                                align_content: AlignContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|parent| {
                                            parent 
                                            .spawn(TextBundle::from_section(
                                                    "y = ",
                                                    TextStyle {
                                                        font: asset_server.load("fonts/tahoma.ttf"),
                                                        font_size: 30.,
                                                        color: Color::rgb(0.9, 0.9, 0.9)
                                                    }
                                            ))
                                            .insert(YGradientText); // insert label that this is for the y component of the gradient
                                        });
                                    
                                });

                            parent 
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(25.), Val::Percent(100.)),
                                        justify_content: JustifyContent::Center,
                                        align_content: AlignContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|parent| {
                                    // place button to toggle addition/multiplication
                                    parent
                                    .spawn(NodeBundle {
                                        style: Style {
                                            size: Size::new(Val::Px(BUTTON_WIDTH + 2.*BUTTON_SPACING), Val::Px(BUTTON_HEIGHT+2.*BUTTON_SPACING)),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .with_children(|parent| {
                                        parent 
                                            .spawn(ButtonBundle {
                                                style: Style {
                                                    size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                                                    justify_content: JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                background_color: NORMAL_BUTTON_COLOR.into(),
                                                ..default()
                                            })
                                            .with_children(|parent| {
                                                parent.spawn(TextBundle::from_section(
                                                    "Add",
                                                    TextStyle {
                                                        font: asset_server.load("../assets/fonts/tahoma.ttf"),
                                                        font_size: 20.0,
                                                        color: Color::rgb(0.9, 0.9, 0.9),
                                                    },
                                                ));
                                            })
                                            .insert(OperationButton {
                                                operation: GradientOperation::new(), // default starting
                                            });
                                    });
                                });
                        });
                        
        
                    parent
                        // button rows
                        .spawn(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(50.), Val::Px(2. * BUTTON_HEIGHT + 3. * BUTTON_SPACING)), // size of button set
                                flex_direction: FlexDirection::Column, // stack button sets in column
                                justify_content: JustifyContent::Center, 
                                align_content: AlignContent::Center, // align to center of height
                                align_items: AlignItems::Center, 
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
        
                            parent // x buttons 
                                .spawn(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.), Val::Px(BUTTON_HEIGHT + BUTTON_SPACING)), // size of button set taking up half the screen
                                        margin: UiRect::all(Val::Px(BUTTON_SPACING)),
                                        flex_direction: FlexDirection::Row, // align buttons in row
                                        align_items: AlignItems::Center, // align to center of height
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
                                        size: Size::new(Val::Percent(100.), Val::Px(BUTTON_HEIGHT + BUTTON_SPACING)), // size of button set taking up half the screen
                                        margin: UiRect::all(Val::Px(BUTTON_SPACING)),
                                        flex_direction: FlexDirection::Row, // align buttons in row
                                        align_items: AlignItems::Center, // align to center of height
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
                });
        });
}

/// Update gas collected text 
fn update_gas_collected_text(
    mut query: Query<(&Children, &mut Visibility), With<GasCollectedText>>,
    mut game_state: Query<&mut GameState>,
    mut text_query: Query<&mut Text>,
) {
    let game_state = game_state.single_mut(); // get game state 

    // update gas collected text
    for (children, mut visibility) in query.iter_mut() {
        if game_state.level_info[game_state.current_level as usize].gas_locations.len() > 0 { // if there are gas cans this level 
            let mut text = text_query.get_mut(children[0]).unwrap(); // get text
            visibility.is_visible = true; // make visible
            // update text
            text.sections[0].value = format!("Gas Collected: {}/{}", game_state.gas_collected.iter().sum::<u32>(), game_state.level_info[game_state.current_level as usize].gas_locations.len());
        } else {
            visibility.is_visible = false; // hide text if no gas cans this level 
        }
        
    }
    
}

/// Update system for new level text 
fn new_level_text_system(
    mut query: Query<(&mut Text, &mut NewLevelText)>,
) {
    let (mut text, mut text_info) = query.single_mut();

    // update text
    if text_info.fade_in { // if currently fading in, increment alpha until 1.0 
        text_info.alpha += NEW_LEVEL_TEXT_FADE_IN_SPEED; 
        text.sections[0].style.color = Color::rgba(LEVEL_COMPLETE_TEXT_COLOR.0, LEVEL_COMPLETE_TEXT_COLOR.1, LEVEL_COMPLETE_TEXT_COLOR.2, text_info.alpha);

        text.sections[0].value = format!("Level {}", text_info.level);

        if text_info.alpha >= 1.0 { // if alpha is 1.0, stop fading in and start fading out 
            text_info.fade_in = false;
            text_info.fade_out = true;
        }
    } else if text_info.fade_out {
        text_info.alpha -= NEW_LEVEL_TEXT_FADE_IN_SPEED; // if currently fading out, decrement alpha until 0.0 
        text.sections[0].style.color = Color::rgba(LEVEL_COMPLETE_TEXT_COLOR.0, LEVEL_COMPLETE_TEXT_COLOR.1, LEVEL_COMPLETE_TEXT_COLOR.2, text_info.alpha);

        if text_info.alpha <= 0.0 { // if alpha is 0.0, stop fading out and reset text 
            text_info.fade_out = false;
            text_info.alpha = 0.0;
            text.sections[0].value = "".to_string();
        }
    }
}

/// update system for current level text
fn current_level_text_update(
    mut query: Query<(&mut Text, With<LevelText>)>,
    game_state: Query<&GameState>,
) {
    let game_state = game_state.single();

    for (mut text, _) in query.iter_mut() {
        text.sections[0].value = format!("Level: {}", game_state.current_level+1);
    }
}

/// update system for operation choice button
fn operation_state_button_handling(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &mut OperationButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut operation_state: ResMut<State<GradientOperationState>>,
) {
    for  (interaction, mut color, children, mut button) in &mut interaction_query { // get button interaction
        let mut text = text_query.get_mut(children[0]).unwrap(); // get button text

        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into(); // change button color 

                match button.operation {
                    GradientOperation::Add => {
                        button.operation = GradientOperation::Multiply; // cycle through operations 

                        text.sections[0].value = String::from("Multiply"); // update text

                        operation_state.set(GradientOperationState::Multiply).unwrap(); // update state
                    },
                    GradientOperation::Multiply => {
                        button.operation = GradientOperation::Add; // cycle through operations 

                        text.sections[0].value = String::from("Add"); // update text

                        operation_state.set(GradientOperationState::Add).unwrap(); // update state
                    },
                }
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into(); // change color on hover 
            },
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into(); // change color back on no hover/interaction   
            },
        }
    }
}

/// update system for simulating button
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
                // check to make sure toggle and simulating state agree 
                match simulating_state.current() {
                    Simulating::NotSimulating => {
                        simulating_button.toggled = false;
                    }
                    Simulating::Simulating => {
                        simulating_button.toggled = true;
                    }
                }

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
                // check to make sure toggle and simulating state agree 
                match simulating_state.current() {
                    Simulating::NotSimulating => {
                        simulating_button.toggled = false;
                    }
                    Simulating::Simulating => {
                        simulating_button.toggled = true;
                    }
                }

                if simulating_button.toggled {
                    *color = SIM_BUTTON_ON_HOVER.into();
                } else {
                    *color = SIM_BUTTON_OFF_HOVER.into();
                }
            },
            Interaction::None => { // on no interaction change color back to normal 
                // check to make sure toggle and simulating state agree 
                match simulating_state.current() {
                    Simulating::NotSimulating => {
                        simulating_button.toggled = false;
                    }
                    Simulating::Simulating => {
                        simulating_button.toggled = true;
                    }
                }

                if simulating_button.toggled {
                    text.sections[0].value = "Stop".to_string(); // change text of button
                    *color = SIM_BUTTON_ON.into();
                } else {
                    text.sections[0].value = "Simulate".to_string(); // change text of button
                    *color = SIM_BUTTON_OFF.into();
                }
            }
        }
    }
}

/// system to ensure that the simulating button always agrees with current state of game 
fn simulating_button_check(
    mut button: Query<(&mut BackgroundColor, &Children, &mut SimulatingButton)>,
    mut text_query: Query<&mut Text>,
    simulating_state: Res<State<Simulating>>,
) {
    let (mut color, children, mut simulating_button) = button.single_mut(); // get button (should only be 1 simulating button)
    let mut text = text_query.get_mut(children[0]).unwrap(); // get text of button

    match simulating_state.current() {
        Simulating::NotSimulating => {
            if simulating_button.toggled { // if currently simulating 
                simulating_button.toggled = false; // untoggle button 

                text.sections[0].value = "Simulate".to_string(); // change text of button
                *color = SIM_BUTTON_OFF.into();
            }
        }
        Simulating::Simulating => {
            if simulating_button.toggled == false { // if currently simulating 
                simulating_button.toggled = true; // toggle button 

                text.sections[0].value = "Stop".to_string(); // change text of button
                *color = SIM_BUTTON_ON.into();
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
    mut simulating_state: ResMut<State<Simulating>>,
    operation_state: Res<State<GradientOperationState>>,
) {
    let mut gradient = gradient.single_mut(); // get gradient
    let game_state = game_state.single(); // get game state

    for (interaction, mut color, children, mut button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap(); // get text of button
        match *interaction {
            Interaction::Clicked => {
                // if any button is clicked for any reason, stop simulating 
                match simulating_state.current() {
                    Simulating::Simulating => {
                        simulating_state.set(Simulating::NotSimulating).unwrap(); // only change if not already in state
                    }
                    Simulating::NotSimulating => {}
                }

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
                    text.sections[0].style.color = NORMAL_BUTTON_TEXT_COLOR; // change text color back to normal
                    button.used = false; // set button to unused 
                } else { // if button is not used 
                    // add corresponding function to gradient 
                    match button.xy {
                        ButtonXY::X => {
                            // add function to gradient 
                            gradient.add_x_function(
                                button.id, // take button id as function id
                                match operation_state.current() {GradientOperationState::Add => {GradientOperation::Add}, GradientOperationState::Multiply => {GradientOperation::Multiply}}, // operation based on state 
                                game_state.level_info[game_state.current_level as usize].x_functions[button.id as usize].1, // get function
                                game_state.level_info[game_state.current_level as usize].x_functions[button.id as usize].0.clone() // get string representing function
                            );

                        },
                        ButtonXY::Y => {
                            // add function to gradient 
                            gradient.add_y_function(
                                button.id, // take button id as function id
                                match operation_state.current() {GradientOperationState::Add => {GradientOperation::Add}, GradientOperationState::Multiply => {GradientOperation::Multiply}}, // operation based on state
                                game_state.level_info[game_state.current_level as usize].y_functions[button.id as usize].1, // get function
                                game_state.level_info[game_state.current_level as usize].y_functions[button.id as usize].0.clone() // get string representing function
                            );
                        },
                    }

                    *color = PRESSED_BUTTON_COLOR.into(); // change color to pressed 
                    text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR; // change text color to pressed
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

                    text.sections[0].style.color = PRESSED_BUTTON_TEXT_COLOR; // change text color when pressed 
                } else {
                    *color = NORMAL_BUTTON_COLOR.into(); // if not currently toggled, change to normal color

                    text.sections[0].style.color = NORMAL_BUTTON_TEXT_COLOR; // change text color when not pressed
                }
                
            },
        }
    }
}

/// function for updating the x gradient text 
fn x_gradient_text_system(
    mut text_query: Query<&mut Text, With<XGradientText>>,
    gradient: Query<&Gradient>,
) {
    let gradient = gradient.single(); // get gradient

    let mut text = text_query.single_mut(); // get text

    // update text 
    text.sections[0].value = format!("x = {}", gradient.x_text());
}

/// function for updating the y gradient text
fn y_gradient_text_system(
    mut text_query: Query<&mut Text, With<YGradientText>>,
    gradient: Query<&Gradient>,
) {
    let gradient = gradient.single(); // get gradient

    let mut text = text_query.single_mut(); // get text

    // update text 
    text.sections[0].value = format!("y = {}", gradient.y_text());
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system(grad_component_button_system);
        app.add_system(simulating_button_system);
        app.add_system(simulating_button_check);
        app.add_system(x_gradient_text_system);
        app.add_system(y_gradient_text_system);
        app.add_system(operation_state_button_handling);
        app.add_system(current_level_text_update);
        app.add_system(new_level_text_system);
        app.add_system(update_gas_collected_text);
    }
}