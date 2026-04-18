use bevy::prelude::*;

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TaskStore>()
           .add_systems(Startup, setup_initial_tasks);
    }
}

#[derive(Clone, Debug)]
pub struct Task {
    pub entity: Entity,
    pub title: String,
    pub completed: bool,
}

#[derive(Resource, Default)]
pub struct TaskStore {
    pub tasks: Vec<Task>,
    next_id: u32,
}

impl TaskStore {
    pub fn add_task(&mut self, title: String) {
        // We use a dummy entity ID for the UI logic mapping in this simple architecture
        let entity = Entity::from_raw(self.next_id);
        self.next_id += 1;
        
        self.tasks.push(Task {
            entity,
            title,
            completed: false,
        });
    }

    pub fn toggle_task(&mut self, entity: Entity) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.entity == entity) {
            task.completed = !task.completed;
        }
    }

    pub fn delete_task(&mut self, entity: Entity) {
        self.tasks.retain(|t| t.entity != entity);
    }
}

fn setup_initial_tasks(mut task_store: ResMut<TaskStore>) {
    task_store.add_task("Design the new UI mockups".to_string());
    task_store.add_task("Review pull requests".to_string());
    task_store.add_task("Update Bevy to version 0.12".to_string());
    
    // Mark one as completed for demonstration
    if let Some(task) = task_store.tasks.get_mut(1) {
        task.completed = true;
    }
}
