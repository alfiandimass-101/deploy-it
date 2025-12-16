use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Component)]
pub struct BotState {
    pub data: Arc<Mutex<BotStateData>>,
}

pub struct BotStateData {
    pub state: StateEnum,
    pub last_phase: Instant,
    pub is_afk: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum StateEnum {
    JustJoined,
    LoggedIn,
    SwitchedServer,
    Active,
}

impl Default for BotState {
    fn default() -> Self {
        Self {
            data: Arc::new(Mutex::new(BotStateData {
                state: StateEnum::JustJoined,
                last_phase: Instant::now(),
                is_afk: true,
            })),
        }
    }
}
