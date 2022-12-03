use bevy::{
    prelude::*,
    asset::AssetServer,
};

use crate::{Simulating};

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
    pub text: String, // text to display on button
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

fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                                        format!("Button {}", i),
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
                                text: format!("Button {}", i),
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
                                        format!("Button {}", i),
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
                                text: format!("Button {}", i),
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

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &GradComponentButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children, button) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON_COLOR.into();
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                text.sections[0].value = button.text.to_string();
                *color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(ui_setup);
        app.add_system(button_system);
        app.add_system(simulating_button_system);
    }
}