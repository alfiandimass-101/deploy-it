use azalea::{prelude::*, chat::ChatPacket};

use crate::bot::component::BotComponent;

/// UUID dari pemilik bot.
const OWNER_UUID: uuid::Uuid = uuid::uuid!("452cb59a-adf3-3ebe-814b-53015c4e4279");

#[derive(Default, Clone)]
pub struct MasterCommandsPlugin;

impl Plugin for MasterCommandsPlugin {
    fn handle(self: Box<Self>, event: Event, bot: Client, state: State<BotComponent>) {
        tokio::spawn(async move {
            if let Event::Chat(msg) = event {
                if let Some(uuid) = msg.sender_uuid() {
                    if uuid == OWNER_UUID {
                        let content = msg.content();
                        println!("Owner said: {}", content);
                        owner_commands(&content, &bot);
                    }
                }
            }
        });
    }
}

pub fn owner_commands(msg: &str, bot: &Client) {
    let mut words = msg.trim().split_whitespace();
    // Ambil kata pertama sebagai perintah
    let command = if let Some(c) = words.next() {
        c
    } else {
        return; // Pesan kosong, tidak ada perintah
    };

    match command {
        "!say" => {
            let content_to_say = words.collect::<Vec<&str>>().join(" ");
            if !content_to_say.is_empty() {
                bot.chat(&content_to_say);
            }
        }
        _ => {}
    }
}