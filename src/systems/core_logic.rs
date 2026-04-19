use bevy::prelude::*;
use crate::models::{TaskList, Task};
use crate::ui::components::{TaskListContainer, AddTaskButton, TaskAction, HoverEffect, spawn_task_row};

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            sync_task_list_ui,
            handle_add_task,
            handle_task_toggle,
            handle_hover_effects,
        ));
    }
}

// Syncs the UI with the TaskList resource
fn sync_task_list_ui(
    mut commands: Commands,
    task_list: Res<TaskList>,
    q_container: Query<Entity, With<TaskListContainer>>,
) {
    if task_list.is_changed() || task_list.is_added() {
        if let Ok(container_entity) = q_container.get_single() {
            // Clear existing tasks in UI
            commands.entity(container_entity).clear_children();
            
            // Rebuild task list
            commands.entity(container_entity).with_children(|parent| {
                for task in &task_list.tasks {
                    spawn_task_row(parent, task);
                }
            });
        }
    }
}

// Handles clicking the "Add Task" button
fn handle_add_task(
    mut interactions: Query<&Interaction, (Changed<Interaction>, With<AddTaskButton>)>,
    mut task_list: ResMut<TaskList>,
) {
    for interaction in &mut interactions {
        if *interaction == Interaction::Pressed {
            let id = task_list.tasks.len();
            task_list.tasks.push(Task {
                id,
                title: format!("مهمة جديدة رائعة رقم {}", id + 1),
                completed: false,
            });
        }
    }
}

// Handles clicking a task row to toggle completion
fn handle_task_toggle(
    mut interactions: Query<(&Interaction, &TaskAction), (Changed<Interaction>, With<Button>)>,
    mut task_list: ResMut<TaskList>,
) {
    for (interaction, action) in &mut interactions {
        if *interaction == Interaction::Pressed {
            if let Some(task) = task_list.tasks.iter_mut().find(|t| t.id == action.0) {
                task.completed = !task.completed;
            }
        }
    }
}

// Generic hover effect system for all buttons with HoverEffect component
fn handle_hover_effects(
    mut q_buttons: Query<(&Interaction, &mut BackgroundColor, &HoverEffect), Changed<Interaction>>,
) {
    for (interaction, mut bg_color, effect) in &mut q_buttons {
        match *interaction {
            Interaction::Hovered => {
                *bg_color = BackgroundColor(effect.hover);
            }
            Interaction::None => {
                *bg_color = BackgroundColor(effect.normal);
            }
            _ => {}
        }
    }
}
