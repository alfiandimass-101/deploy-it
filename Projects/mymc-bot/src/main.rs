use azalea::{Account, ClientInformation, blocks::{BlockState, BlockStates}, prelude::*};
use tracing::{info, warn};

mod utils;
pub use utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    let account = Account::offline("itzbot");
    ClientBuilder::new()
    .set_handler(handler)
    .start(account, "itzyuurz.aternos.me:11068")
    .await?;
    Ok(())
}
async fn handler(mut bot: Client, mut event: Event, mut state: BotState) -> anyhow::Result<()> {
    match event {
        Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32u8,
                ..Default::default()
            });
        }
        Event::Chat(content_packet) => {
            if let Some(uuid) = content_packet.sender_uuid() {
                if uuid == OWNER_UUID {
                    let content = content_packet.content();
                    let processed = process_owner_command(&content);
                    if let Some(command) = processed {
                        info!(name: "OWNER COMMAND", "COMMAND: {command}, ARG: {arg}", command=command.0, arg=command.1);
                        match command.0 {
                            "!say" => bot.chat(command.1),
                            "!killaura" => match command.1.parse::<u8>()? {
                                1 => state.can_kill = true,
                                0 => state.can_kill = false,
                                _ => warn!("Cannot assign killaura to n'either 0 or 1"),
                            },
                            "!position" => {
                                let bot_pos = bot.position();
                                let pos_str = format!("x: {x},y: {y},z: {z}", y=bot_pos.y, x=bot_pos.x, z=bot_pos.z);
                                bot.chat(&pos_str);
                                info!(name: "BOT POSITION", pos_str);
                            },
                            "!health" => {
                                let health = bot.health();
                                bot.chat(format!("HEALTH: {health}"));
                                info!("BOT HEALTH: {health}");
                            },
                            "!scanblock" => {
    let command_arg = command.1.parse::<u32>()?;
    let handle = tokio::task::spawn_blocking(async move || {
        let bot_pos = bot.position();
        let world = bot.world();
        let readed_world = world.read();
        if command_arg > 1165 { panic!("not valid block_id"); }
        let block_from_id = unsafe {
            azalea::registry::Block::from_u32_unchecked(command_arg)
        };
        let block_states = BlockStates::from(block_from_id);
        let block_find = readed_world.find_blocks(bot_pos, &block_states);
        
        // --- MODIFIKASI DIMULAI DI SINI ---
        
        // Inisialisasi batas waktu 3 detik
        let timeout_duration = std::time::Duration::from_secs(3);
        // Waktu penemuan terakhir (diinisialisasi saat mulai)
        let mut last_find_instant = std::time::Instant::now(); 
        
        let mut index = 0;
        // Ganti .enumerate().for_each() dengan loop 'loop' untuk kontrol break
        let mut iter = block_find.into_iter();

        loop {
            // 1. Cek counter waktu: Berhenti jika waktu berlalu > 3 detik
            if last_find_instant.elapsed() > timeout_duration {
                bot.chat(format!("Scan dihentikan: Tidak menemukan blok baru selama lebih dari 3 detik."));
                break; // Hentikan iterator
            }

            // Coba ambil item berikutnya dari iterator
            let next_block = iter.next();

            match next_block {
                Some(block) => {
                    // 2. Reset counter: Setel ulang waktu karena data baru ditemukan
                    last_find_instant = std::time::Instant::now(); 

                    if !(index > 16) {
                        bot.chat(format!("{block:?}"));
                        std::thread::sleep(std::time::Duration::from_millis(100));
                    }
                    index += 1;
                },
                None => {
                    // Iterator sudah selesai mencari (tidak ada lagi blok yang tersisa)
                    break;
                }
            }
        }
        
        // --- MODIFIKASI SELESAI ---

    });
    handle.await?.await
}
                    } else {
                        info!(name: "OWNER LOG", "OWNER SAID: {}", content);
                    }
                }
            }
        }
        _ => {}
    }
    Ok(())
}