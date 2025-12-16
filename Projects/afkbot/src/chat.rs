use azalea::prelude::*;
// use uuid::Uuid;

pub async fn handle_chat(bot: &mut Client, chat: azalea::chat::ChatPacket) -> anyhow::Result<()> {
    // Robust UUID check: Convert to string to avoid crate version mismatches
    let sender_uuid_opt = chat.sender_uuid();

    let sender_uuid = if let Some(uuid) = sender_uuid_opt {
        uuid
    } else {
        return Ok(());
    };

    // Target: 6f6e29fe-30c2-30ae-86ac-73a9e331ab35
    let target_uuid_str = "6f6e29fe-30c2-30ae-86ac-73a9e331ab35";

    if sender_uuid.to_string() == target_uuid_str {
        let msg = chat.message().to_string();
        println!("{}", msg);
        // Extract command after "!say" even if surrounded by extra formatting
        if let Some(start) = msg.find("!say ") {
            // Capture everything after "!say "
            let mut args = &msg[start + 5..];
            // Trim possible trailing '>' and surrounding whitespace
            args = args.trim_end_matches('>').trim();
            // Optionally trim leading whitespace
            let args = args.trim_start();
            if !args.is_empty() {
                bot.chat(args);
            }
        }
    }
    Ok(())
}
