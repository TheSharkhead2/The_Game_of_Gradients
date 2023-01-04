use std::char::MAX;

use bevy::{
    prelude::*,
};
use bevy::transform::TransformSystem;

use crate::ui::ButtonXY;

use crate::constants::{
    MAX_COMPONENTS_PER_DIMENSION,
    BUTTON_HEIGHT,
    BUTTON_WIDTH,
    NORMAL_BUTTON_COLOR,
    DEFAULT_FONT,
    DEFAULT_FONT_SIZE, 
    NORMAL_BUTTON_TEXT_COLOR, 
    BUTTON_SPACING, 
    PRESSED_BUTTON_COLOR, HOVERED_BUTTON_COLOR
};

#[derive(Component)]
/// Represents a component to the x or y function
pub struct FunctionComponent {
    pub id: u32, // function id
    pub xy: ButtonXY, // for x or y direction 
    pub text: String, // function text
    pub visible: bool, // whether or not this specific function component is "active" (part of current level)
}

/// Function for generating the function components 
fn spawn_function_components(
    asset_server: Res<AssetServer>, 
    xy: ButtonXY
) -> impl FnOnce(&mut ChildBuilder) {
    let loaded_font = asset_server.load(DEFAULT_FONT);

    move |base_parent| {
        // create maximum count of function componenets
        for i in 0..MAX_COMPONENTS_PER_DIMENSION {
            base_parent
                .spawn(ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(BUTTON_WIDTH), Val::Px(BUTTON_HEIGHT)),
                        margin: UiRect::all(Val::Px(BUTTON_SPACING)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON_COLOR.into(),
                    ..default()
                })
                // text inside button 
                .with_children(|parent| {
                    parent 
                        .spawn(TextBundle::from_section(
                            format!("{}", i),
                            TextStyle {
                                font: loaded_font.clone(),
                                font_size: DEFAULT_FONT_SIZE,
                                color: NORMAL_BUTTON_TEXT_COLOR
                            }    
                        ));
                })
                .insert(FunctionComponent {
                    id: i, 
                    xy: xy,
                    text: format!("{}", i),
                    visible: true
                });
        }
    } 
}

/// Function for initializing ui elements for crafting x and y components of gradient 
pub fn ui_function_creator_setup<'a>(
    asset_server: Res<'a, AssetServer>,
) -> impl FnOnce(&mut ChildBuilder) -> () + 'a {
    |base_parent: &mut ChildBuilder| {
        base_parent
            // base of function selector ui element. 
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.),Val::Percent(100.)),
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent 
                    // grab 40 percent for x 
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent 
                            .spawn(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::FlexStart,
                                    ..default()
                                },
                                ..default()
                            })
                            .with_children(spawn_function_components(asset_server, ButtonXY::X));
                    });

                parent 
                    // grab 20 percent for add/multiply button and simulation button 
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(20.), Val::Percent(100.)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent 
                            .spawn(NodeBundle {
                                ..default()
                            });
                    });

                parent
                    // grab 40 percent for y functions 
                    .spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(40.), Val::Percent(100.)),
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent 
                            .spawn(NodeBundle {
                                ..default()
                            });
                    }); 
            });
    }
}

/// Function for handling interactions with function component buttons
fn function_component_interaction_handling(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children, &mut GlobalTransform, &mut FunctionComponent),
        (Changed<Interaction>, With<Button>)
    >,
    mut text_query: Query<&mut Text>,
) {
    // loop through all interactions 
    for (interaction, mut color, children, mut transform, mut function_component) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap(); // get text specific to function component

        // match all possible interactions
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON_COLOR.into(); // set color to pressed color 

                *transform = GlobalTransform::from_xyz(0., 0., 0.);
            },
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into(); // set color to hovered color 
            },
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into(); // reset color on no interaction
            },
        }
    }
}

/// Function components plugin 
pub struct FunctionComponentPlugin;

impl Plugin for FunctionComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            CoreStage::PostUpdate,
            function_component_interaction_handling
                .after(TransformSystem::TransformPropagate));
    }
}