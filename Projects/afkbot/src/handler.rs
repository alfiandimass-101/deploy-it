use azalea::Event as AzaleaEvent;
use azalea::prelude::*;
use std::time::{Duration, Instant};

use crate::behavior::perform_active_logic;
use crate::chat::handle_chat;
use crate::state::{BotState, StateEnum};

pub async fn handle(mut bot: Client, event: AzaleaEvent, state: BotState) -> anyhow::Result<()> {
    match event {
        AzaleaEvent::Init => {
            let mut data = state.data.lock();
            data.state = StateEnum::JustJoined;
            data.last_phase = Instant::now();
        }
        AzaleaEvent::Tick => {
            handle_tick(&mut bot, &state).await?;
        }
        AzaleaEvent::Chat(chat) => {
            handle_chat(&mut bot, chat).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn handle_tick(bot: &mut Client, state: &BotState) -> anyhow::Result<()> {
    let should_perform_active = {
        let mut data = state.data.lock();

        match data.state {
            StateEnum::JustJoined => {
                if data.last_phase.elapsed() >= Duration::from_secs(1) {
                    bot.chat("/login rifaigg123");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    bot.chat("/register rifaigg123 rifaigg123");
                    data.state = StateEnum::LoggedIn;
                    data.last_phase = Instant::now();
                }
                false
            }
            StateEnum::LoggedIn => {
                if data.last_phase.elapsed() >= Duration::from_secs(3) {
                    bot.chat("/server survivalmix");
                    println!("Switched to survivalmix");
                    data.state = StateEnum::SwitchedServer;
                    data.last_phase = Instant::now();
                }
                false
            }
            StateEnum::SwitchedServer => {
                if data.last_phase.elapsed() >= Duration::from_secs(2) {
                    data.state = StateEnum::Active;
                }
                false
            }
            StateEnum::Active => true,
        }
    }; // Lock dropped here

    if should_perform_active {
        perform_active_logic(bot, state.data.clone()).await;
    }

    Ok(())
}
