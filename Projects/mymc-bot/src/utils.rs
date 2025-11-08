use azalea::prelude::Component;
use tokio::sync::{Mutex};
use tracing::info;
use std::sync::Arc;

pub const OWNER_UUID: uuid::Uuid = uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    #[default]
    DoNothing,
    Attack,
}

pub struct OwnerCommand<'a>(&'a str, &'a str);

#[derive(Debug, Default, Component, Clone)]
pub struct BotState {
    pub can_kill: bool,
    task: Arc<Mutex<TaskState>>,
}

pub fn process_owner_command<'a>(content: &'a str) -> Option<OwnerCommand> {
    if !content.starts_with("!") {
        return None;
    }

    let trimmed_content = content.trim();
    let first_arg = trimmed_content.split_whitespace().next();
    if let Some(command) = first_arg {
        let command_len = command.len();
        if let Some(start_index) = trimmed_content.find(command) {
            let end_of_command_index = start_index + command_len;
            if end_of_command_index < trimmed_content.len() {
                return (command, &trimmed_content[end_of_command_index..].trim_start());
            }
        }
        return None;
    }
    None
}