use std::sync::{Arc, Mutex};
use std::cell::RefCell;
use azalea::prelude::*;

type TaskModel<T> = Arc<Mutex<RefCell<T>>>;

pub enum Task {
    Attack,
    DoNothing,
}

#[derive(Clone, Component)]
pub struct BotComponent {
    pub task: Arc<Mutex<RefCell<Task>>>,
}

impl Default for BotComponent {
    fn default() -> Self {
        BotComponent { 
            task: Arc::new(Mutex::new(RefCell::new(Task::DoNothing))),
         }
    }
}

impl BotComponent {
    pub fn get_task(&self) -> Arc<Mutex<RefTask>> {
        self.task.clone()
    }

    pub fn set_task(&mut self) ->
}