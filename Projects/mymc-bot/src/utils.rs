use azalea::prelude::Component;
use tokio::sync::{Mutex};
use std::sync::Arc;

pub const OWNER_UUID: uuid::Uuid = uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    #[default]
    DoNothing,
    Attack,
}

#[derive(Debug, Default, Component, Clone)]
pub struct BotState {
    pub can_kill: bool,
    task: Arc<Mutex<TaskState>>,
}

fn process_owner_command<'a>(content: &'a str) -> &'a str {
    if !content.starts_with("!") return content;
    
}