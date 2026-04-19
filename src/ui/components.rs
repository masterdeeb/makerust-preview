use bevy::prelude::*;
use super::theme;
use crate::models::Task;

// Markers for Systems
#[derive(Component)]
pub struct TaskListContainer;

#[derive(Component)]
pub struct AddTaskButton;

#[derive(Component)]
pub struct TaskAction(pub usize);

#[derive(Component)]
pub struct HoverEffect {
    pub normal: Color,
    pub hover: Color,
}

pub fn spawn_sidebar(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            width: Val::Px(280.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(32.0)),
            border: UiRect::right(Val::Px(1.0)),
            ..default()
        },
        background_color: BackgroundColor(theme::BG_SIDEBAR),
        border_color: BorderColor(theme::BORDER),
        ..default()
    }).with_children(|sidebar| {
        // App Logo / Title
        sidebar.spawn(TextBundle::from_section(
            "TaskFlow",
            TextStyle {
                font_size: 32.0,
                color: theme::TEXT_PRIMARY,
                ..default()
            },
        ).with_style(Style {
            margin: UiRect::bottom(Val::Px(40.0)),
            ..default()
        }));

        // Navigation Item (Mock)
        spawn_nav_item(sidebar, "جميع المهام", true);
        spawn_nav_item(sidebar, "مهام اليوم", false);
        spawn_nav_item(sidebar, "المهمة", false);
    });
}

fn spawn_nav_item(parent: &mut ChildBuilder, text: &str, active: bool) {
    let bg_color = if active { theme::SURFACE } else { Color::NONE };
    let text_color = if active { theme::ACCENT } else { theme::TEXT_MUTED };

    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(12.0)),
            margin: UiRect::bottom(Val::Px(8.0)),
            ..default()
        },
        background_color: BackgroundColor(bg_color),
        ..default()
    }, HoverEffect { normal: bg_color, hover: theme::SURFACE_HOVER })).with_children(|btn| {
        btn.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 18.0,
                color: text_color,
                ..default()
            },
        ));
    });
}

pub fn spawn_main_content(parent: &mut ChildBuilder) {
    parent.spawn(NodeBundle {
        style: Style {
            flex_grow: 1.0,
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(48.0)),
            ..default()
        },
        ..default()
    }).with_children(|main| {
        
        // Header Row
        main.spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(40.0)),
                ..default()
            },
            ..default()
        }).with_children(|header| {
            header.spawn(TextBundle::from_section(
                "جميع المهام",
                TextStyle {
                    font_size: 42.0,
                    color: theme::TEXT_PRIMARY,
                    ..default()
                },
            ));

            // Add Task Button
            header.spawn((ButtonBundle {
                style: Style {
                    padding: UiRect::axes(Val::Px(24.0), Val::Px(12.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: BackgroundColor(theme::ACCENT),
                ..default()
            }, AddTaskButton, HoverEffect { normal: theme::ACCENT, hover: theme::ACCENT_HOVER })).with_children(|btn| {
                btn.spawn(TextBundle::from_section(
                    "+ إضافة مهمة",
                    TextStyle {
                        font_size: 18.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));
            });
        });

        // Task List Container
        main.spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }, TaskListContainer));
    });
}

pub fn spawn_task_row(parent: &mut ChildBuilder, task: &Task) {
    let text_color = if task.completed { theme::TEXT_MUTED } else { theme::TEXT_PRIMARY };
    let checkbox_color = if task.completed { theme::SUCCESS } else { Color::NONE };

    parent.spawn((ButtonBundle {
        style: Style {
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            margin: UiRect::bottom(Val::Px(12.0)),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        background_color: BackgroundColor(theme::SURFACE),
        border_color: BorderColor(theme::BORDER),
        ..default()
    }, TaskAction(task.id), HoverEffect { normal: theme::SURFACE, hover: theme::SURFACE_HOVER })).with_children(|row| {
        
        // Checkbox Visual
        row.spawn(NodeBundle {
            style: Style {
                width: Val::Px(24.0),
                height: Val::Px(24.0),
                margin: UiRect::right(Val::Px(16.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            background_color: BackgroundColor(checkbox_color),
            border_color: BorderColor(if task.completed { theme::SUCCESS } else { theme::BORDER }),
            ..default()
        });

        // Task Title
        row.spawn(TextBundle::from_section(
            &task.title,
            TextStyle {
                font_size: 20.0,
                color: text_color,
                ..default()
            },
        ));
    });
}
