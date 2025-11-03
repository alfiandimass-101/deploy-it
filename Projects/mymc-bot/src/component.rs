use std::sync::Arc;

use azalea::prelude::*;

enum Task {
    Attack,
    DoNothing,
}

#[derive(Clone, Component)]
pub struct BotComponent {
    task: Arc<Task>,
}

impl Default for BotComponent {
    fn default() -> Self {
        BotComponent { 
            task: Arc::new(Task::DoNothing),
         }
    }
}