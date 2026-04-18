use bevy::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}

#[derive(Resource, Default)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(Event)]
pub struct AddTaskEvent(pub String);

#[derive(Event)]
pub struct ToggleTaskEvent(pub Uuid);

#[derive(Event)]
pub struct DeleteTaskEvent(pub Uuid);

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TaskList>()
            .add_event::<AddTaskEvent>()
            .add_event::<ToggleTaskEvent>()
            .add_event::<DeleteTaskEvent>()
            .add_systems(Update, (handle_add_task, handle_toggle_task, handle_delete_task));
    }
}

fn handle_add_task(mut events: EventReader<AddTaskEvent>, mut task_list: ResMut<TaskList>) {
    for ev in events.read() {
        task_list.tasks.push(Task {
            id: Uuid::new_v4(),
            title: ev.0.clone(),
            completed: false,
        });
    }
}

fn handle_toggle_task(mut events: EventReader<ToggleTaskEvent>, mut task_list: ResMut<TaskList>) {
    for ev in events.read() {
        if let Some(task) = task_list.tasks.iter_mut().find(|t| t.id == ev.0) {
            task.completed = !task.completed;
        }
    }
}

fn handle_delete_task(mut events: EventReader<DeleteTaskEvent>, mut task_list: ResMut<TaskList>) {
    for ev in events.read() {
        task_list.tasks.retain(|t| t.id != ev.0);
    }
}
