use azalea::prelude::*;
// use uuid::Uuid;

pub async fn handle_chat(bot: &mut Client, chat: azalea::chat::ChatPacket) -> anyhow::Result<()> {
    // Robust UUID check: Convert to string to avoid crate version mismatches
    let sender_uuid_opt = chat.sender();

    let sender_uuid = if let Some(uuid) = sender_uuid_opt {
        uuid
    } else {
        return Ok(());
    };

    // Target: 6f6e29fe-30c2-30ae-86ac-73a9e331ab35
    let target_uuid_str = "6f6e29fe-30c2-30ae-86ac-73a9e331ab35";

    if sender_uuid.to_string() == target_uuid_str {
        let msg = chat.message().to_string();
        if msg.starts_with("!say ") {
            let args = msg.trim_start_matches("!say ");
            bot.chat(args);
        }
    }
    Ok(())
}
