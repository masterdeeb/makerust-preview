use bevy::prelude::*;
use crate::ui::theme::*;
use crate::ui::components::*;
use crate::systems::core_logic::TaskEvent;

pub fn button_hover_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Hovered => {
                // Simple highlight effect for all buttons
                let current = color.0;
                *color = BackgroundColor(Color::rgb(
                    (current.r() + 0.1).min(1.0),
                    (current.g() + 0.1).min(1.0),
                    (current.b() + 0.1).min(1.0),
                ));
            }
            Interaction::None => {
                // Reset handled by specific systems or we rely on render system to reset
                // For generic buttons without specific state, we might need a base color component.
                // For simplicity in this flat design, we let the render system enforce base colors,
                // or we just do a slight dimming here.
            }
            _ => {}
        }
    }
}

pub fn handle_add_task_click(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<AddTaskButton>)>,
    mut task_events: EventWriter<TaskEvent>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            task_events.send(TaskEvent::Add(format!("New Task")));
        }
    }
}

pub fn handle_toggle_task_click(
    interaction_query: Query<(&Interaction, &ToggleTaskButton), Changed<Interaction>>,
    mut task_events: EventWriter<TaskEvent>,
) {
    for (interaction, toggle) in &interaction_query {
        if *interaction == Interaction::Pressed {
            task_events.send(TaskEvent::Toggle(toggle.0));
        }
    }
}

pub fn handle_delete_task_click(
    interaction_query: Query<(&Interaction, &DeleteTaskButton), Changed<Interaction>>,
    mut task_events: EventWriter<TaskEvent>,
) {
    for (interaction, delete) in &interaction_query {
        if *interaction == Interaction::Pressed {
            task_events.send(TaskEvent::Delete(delete.0));
        }
    }
}
