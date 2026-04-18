pub mod theme;

use bevy::prelude::*;
use crate::systems::core_logic::{CalcAction, CalculatorData};
use theme::*;

#[derive(Component)]
pub struct DisplayText;

#[derive(Component)]
pub struct CalcButton(pub CalcAction);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (button_interaction_system, update_display_system));
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Root node
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: COLOR_BACKGROUND.into(),
        ..default()
    }).with_children(|parent| {
        // Calculator Container
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Px(340.0),
                height: Val::Px(520.0),
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(12.0),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: COLOR_CALC_BG.into(),
            border_color: Color::rgb(0.2, 0.2, 0.2).into(),
            ..default()
        }).with_children(|calc| {
            // Display Screen
            calc.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexEnd,
                    padding: UiRect::all(Val::Px(15.0)),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: COLOR_DISPLAY_BG.into(),
                ..default()
            }).with_children(|disp| {
                disp.spawn(( 
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font_size: 56.0,
                            color: COLOR_TEXT,
                            ..default()
                        }
                    ),
                    DisplayText,
                ));
            });

            // Buttons Grid Layout
            let buttons = vec![
                vec![("C", CalcAction::Clear, COLOR_BTN_SPECIAL, 3.0), ("/", CalcAction::Op('/'), COLOR_BTN_OP, 1.0)],
                vec![("7", CalcAction::Digit(7), COLOR_BTN_NORMAL, 1.0), ("8", CalcAction::Digit(8), COLOR_BTN_NORMAL, 1.0), ("9", CalcAction::Digit(9), COLOR_BTN_NORMAL, 1.0), ("*", CalcAction::Op('*'), COLOR_BTN_OP, 1.0)],
                vec![("4", CalcAction::Digit(4), COLOR_BTN_NORMAL, 1.0), ("5", CalcAction::Digit(5), COLOR_BTN_NORMAL, 1.0), ("6", CalcAction::Digit(6), COLOR_BTN_NORMAL, 1.0), ("-", CalcAction::Op('-'), COLOR_BTN_OP, 1.0)],
                vec![("1", CalcAction::Digit(1), COLOR_BTN_NORMAL, 1.0), ("2", CalcAction::Digit(2), COLOR_BTN_NORMAL, 1.0), ("3", CalcAction::Digit(3), COLOR_BTN_NORMAL, 1.0), ("+", CalcAction::Op('+'), COLOR_BTN_OP, 1.0)],
                vec![("0", CalcAction::Digit(0), COLOR_BTN_NORMAL, 2.0), (".", CalcAction::Decimal, COLOR_BTN_NORMAL, 1.0), ("=", CalcAction::Equals, COLOR_BTN_OP, 1.0)],
            ];

            for row in buttons {
                calc.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(65.0),
                        flex_direction: FlexDirection::Row,
                        column_gap: Val::Px(12.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|row_node| {
                    for (label, action, color, flex_grow) in row {
                        row_node.spawn((
                            ButtonBundle {
                                style: Style {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    flex_grow,
                                    ..default()
                                },
                                background_color: color.into(),
                                ..default()
                            },
                            CalcButton(action),
                        )).with_children(|btn| {
                            btn.spawn(TextBundle::from_section(
                                label,
                                TextStyle {
                                    font_size: 32.0,
                                    color: COLOR_TEXT,
                                    ..default()
                                }
                            ));
                        });
                    }
                });
            }
        });
    });
}

fn button_interaction_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CalcButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut action_events: EventWriter<CalcAction>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = COLOR_BTN_PRESSED.into();
                action_events.send(button.0);
            }
            Interaction::Hovered => {
                *color = COLOR_BTN_HOVER.into();
            }
            Interaction::None => {
                *color = match button.0 {
                    CalcAction::Clear => COLOR_BTN_SPECIAL.into(),
                    CalcAction::Op(_) | CalcAction::Equals => COLOR_BTN_OP.into(),
                    _ => COLOR_BTN_NORMAL.into(),
                };
            }
        }
    }
}

fn update_display_system(
    calc_data: Res<CalculatorData>,
    mut query: Query<&mut Text, With<DisplayText>>,
) {
    if calc_data.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = calc_data.display.clone();
        }
    }
}
