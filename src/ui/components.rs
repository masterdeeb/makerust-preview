use bevy::prelude::*;

#[derive(Component)]
pub struct TaskListContainer;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum ButtonAction {
    AddTask,
    ToggleTask(Entity),
    DeleteTask(Entity),
    SidebarMenu,
}
