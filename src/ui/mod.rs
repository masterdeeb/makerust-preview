pub mod theme;

use bevy::prelude::*;
use crate::systems::core_logic::{ButtonAction, CalculatorDisplay, Operation};
use theme::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui);
    }
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Main container
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        background_color: COLOR_BACKGROUND.into(),
        ..default()
    }).with_children(|parent| {
        // Calculator body
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                width: Val::Px(320.0),
                padding: UiRect::all(Val::Px(20.0)),
                row_gap: Val::Px(10.0),
                ..default()
            },
            background_color: CALC_BACKGROUND.into(),
            ..default()
        }).with_children(|calc_body| {
            
            // Display Screen
            calc_body.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(80.0),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(15.0)),
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                },
                background_color: DISPLAY_BACKGROUND.into(),
                ..default()
            }).with_children(|display_node| {
                display_node.spawn((
                    TextBundle::from_section(
                        "0",
                        TextStyle {
                            font_size: 48.0,
                            color: TEXT_COLOR,
                            ..default()
                        },
                    ),
                    CalculatorDisplay,
                ));
            });

            // Buttons Grid
            let rows = vec![
                vec![
                    ("7", ButtonAction::Digit(7)),
                    ("8", ButtonAction::Digit(8)),
                    ("9", ButtonAction::Digit(9)),
                    ("/", ButtonAction::Op(Operation::Divide)),
                ],
                vec![
                    ("4", ButtonAction::Digit(4)),
                    ("5", ButtonAction::Digit(5)),
                    ("6", ButtonAction::Digit(6)),
                    ("*", ButtonAction::Op(Operation::Multiply)),
                ],
                vec![
                    ("1", ButtonAction::Digit(1)),
                    ("2", ButtonAction::Digit(2)),
                    ("3", ButtonAction::Digit(3)),
                    ("-", ButtonAction::Op(Operation::Subtract)),
                ],
                vec![
                    ("C", ButtonAction::Clear),
                    ("0", ButtonAction::Digit(0)),
                    ("=", ButtonAction::Calculate),
                    ("+", ButtonAction::Op(Operation::Add)),
                ],
            ];

            for row in rows {
                calc_body.spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Px(65.0),
                        justify_content: JustifyContent::SpaceBetween,
                        column_gap: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|row_node| {
                    for (label, action) in row {
                        let is_op = matches!(action, ButtonAction::Op(_) | ButtonAction::Calculate | ButtonAction::Clear);
                        let bg_color = if is_op { BUTTON_OP_NORMAL } else { BUTTON_NORMAL };

                        row_node.spawn((
                            ButtonBundle {
                                style: Style {
                                    flex_grow: 1.0,
                                    height: Val::Percent(100.0),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: bg_color.into(),
                                ..default()
                            },
                            action,
                        )).with_children(|btn| {
                            btn.spawn(TextBundle::from_section(
                                label,
                                TextStyle {
                                    font_size: 32.0,
                                    color: TEXT_COLOR,
                                    ..default()
                                },
                            ));
                        });
                    }
                });
            }
        });
    });
}
