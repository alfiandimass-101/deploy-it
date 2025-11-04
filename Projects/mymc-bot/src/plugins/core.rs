
use azalea::{app::{App, Plugin, Startup, Update}, prelude::*};

pub struct CoreLogicPlugin;

impl Plugin for CoreLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bot);
        app.add_systems(Update, (handle_chat, tick_commands));
    }
}

fn setup_bot(mut bot: ResMut<Client>) {
    bot.set_client_information(azalea::ClientInformation {
        view_distance: 32,
        ..Default::default()
    });
}

fn handle_chat(
    mut bot: ResMut<Client>,
    mut events: EventReader<azalea::chat::ChatPacket>,
) {
    for event in events.read() {
        if let Some(uuid) = event.sender_uuid() {
            if uuid == super::OWNER_UUID {
                let content = event.content();
                println!("Owner said: {}", content);
                owner_commands(&content, &mut bot);
            }
        }
    }
}

fn tick_commands(
    mut bot: ResMut<Client>,
    mut query: Query<&mut super::bot::component::BotComponent>,
) {
    let mut component = query.single_mut();
    let task = component.get_task();
    let task = task.lock().unwrap();

    match *task {
        super::bot::component::Task::Attack => {
            // TODO: Implement attack logic
        }
        super::bot::component::Task::DoNothing => {
            // Do nothing
        }
    }
}

fn owner_commands(msg: &str, bot: &mut Client) {
    let mut words = msg.trim().split_whitespace();
    let command = if let Some(c) = words.next() {
        c
    } else {
        return;
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
