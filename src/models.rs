use bevy::prelude::*;

pub struct ModelsPlugin;

impl Plugin for ModelsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TaskList {
            tasks: vec![
                Task {
                    id: 0,
                    title: "تصميم واجهة المستخدم الأساسية".to_string(),
                    completed: true,
                },
                Task {
                    id: 1,
                    title: "برمجة منطق إضافة المهام".to_string(),
                    completed: false,
                },
                Task {
                    id: 2,
                    title: "مراجعة الكود وتحسين الأداء".to_string(),
                    completed: false,
                },
            ],
        });
    }
}

#[derive(Resource, Clone)]
pub struct TaskList {
    pub tasks: Vec<Task>,
}

#[derive(Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub completed: bool,
}
