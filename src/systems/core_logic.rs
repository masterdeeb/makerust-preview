use bevy::prelude::*;

// --- Data Models ---
#[derive(Clone, Debug)]
pub struct Task {
    pub title: String,
    pub completed: bool,
}

#[derive(Resource, Default)]
pub struct Tasks {
    pub list: Vec<Task>,
    pub task_counter: usize,
}

// --- Events ---
#[derive(Event)]
pub enum TaskEvent {
    Add(String),
    Toggle(usize),
    Delete(usize),
}

// --- Plugin ---
pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Tasks>()
           .add_event::<TaskEvent>()
           .add_systems(Startup, setup_initial_tasks)
           .add_systems(Update, handle_task_events);
    }
}

fn setup_initial_tasks(mut tasks: ResMut<Tasks>) {
    tasks.list.push(Task { title: "Review Pull Requests".into(), completed: true });
    tasks.list.push(Task { title: "Design new UI architecture".into(), completed: false });
    tasks.list.push(Task { title: "Update Bevy to 0.12".into(), completed: false });
    tasks.task_counter = 3;
}

fn handle_task_events(
    mut events: EventReader<TaskEvent>,
    mut tasks: ResMut<Tasks>,
) {
    for event in events.read() {
        match event {
            TaskEvent::Add(base_title) => {
                tasks.task_counter += 1;
                let title = format!("{} #{", base_title, tasks.task_counter);
                tasks.list.push(Task { title, completed: false });
            }
            TaskEvent::Toggle(index) => {
                if let Some(task) = tasks.list.get_mut(*index) {
                    task.completed = !task.completed;
                }
            }
            TaskEvent::Delete(index) => {
                if *index < tasks.list.len() {
                    tasks.list.remove(*index);
                }
            }
        }
    }
}
