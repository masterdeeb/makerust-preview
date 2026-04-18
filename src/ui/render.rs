use bevy::prelude::*;
use crate::ui::theme::*;
use crate::ui::components::*;
use crate::systems::core_logic::Tasks;

pub fn render_task_list(
    mut commands: Commands,
    tasks: Res<Tasks>,
    container_query: Query<Entity, With<TaskListContainer>>,
) {
    if !tasks.is_changed() {
        return;
    }

    let Ok(container_entity) = container_query.get_single() else {
        return;
    };

    // Clear existing tasks
    commands.entity(container_entity).clear_children();

    // Rebuild task list
    commands.entity(container_entity).with_children(|parent| {
        if tasks.list.is_empty() {
            parent.spawn(TextBundle::from_section(
                "No tasks yet. Enjoy your day!",
                TextStyle {
                    font_size: 18.0,
                    color: TEXT_MUTED,
                    ..default()
                },
            ).with_style(Style {
                margin: UiRect::all(Val::Px(PADDING_STANDARD)),
                ..default()
            }));
            return;
        }

        for (index, task) in tasks.list.iter().enumerate() {
            // Task Row
            parent.spawn((NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    padding: UiRect::all(Val::Px(PADDING_STANDARD)),
                    border: UiRect::bottom(Val::Px(BORDER_WIDTH)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                border_color: BorderColor(BORDER),
                background_color: BackgroundColor(BG_BASE),
                ..default()
            },)).with_children(|row| {
                
                // Left side: Checkbox + Title
                row.spawn(NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        gap: Size::all(Val::Px(16.0)),
                        ..default()
                    },
                    ..default()
                }).with_children(|left_group| {
                    // Checkbox
                    let checkbox_bg = if task.completed { ACCENT } else { BG_BASE };
                    left_group.spawn((ButtonBundle {
                        style: Style {
                            width: Val::Px(24.0),
                            height: Val::Px(24.0),
                            border: UiRect::all(Val::Px(2.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(if task.completed { ACCENT } else { BORDER }),
                        background_color: BackgroundColor(checkbox_bg),
                        ..default()
                    }, ToggleTaskButton(index)));

                    // Title
                    let text_color = if task.completed { TEXT_MUTED } else { TEXT_MAIN };
                    left_group.spawn(TextBundle::from_section(
                        &task.title,
                        TextStyle {
                            font_size: 18.0,
                            color: text_color,
                            ..default()
                        },
                    ));
                });

                // Right side: Delete Button
                row.spawn((ButtonBundle {
                    style: Style {
                        padding: UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    border_color: BorderColor(BORDER),
                    background_color: BackgroundColor(BG_BASE),
                    ..default()
                }, DeleteTaskButton(index))).with_children(|btn| {
                    btn.spawn(TextBundle::from_section(
                        "Delete",
                        TextStyle {
                            font_size: 14.0,
                            color: DANGER,
                            ..default()
                        },
                    ));
                });
            });
        }
    });
}
