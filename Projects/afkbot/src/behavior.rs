use crate::state::BotStateData;
use azalea::prelude::*;
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn perform_active_logic(bot: &mut Client, data_arc: Arc<Mutex<BotStateData>>) {
    let data = data_arc.lock();
    if data.is_afk {
        bot.jump();
    }
}
