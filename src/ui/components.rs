use bevy::prelude::*;

// Markers for UI elements
#[derive(Component)]
pub struct TaskListContainer;

#[derive(Component)]
pub struct AddTaskButton;

#[derive(Component)]
pub struct ToggleTaskButton(pub usize);

#[derive(Component)]
pub struct DeleteTaskButton(pub usize);

#[derive(Component)]
pub struct GenericButton;
