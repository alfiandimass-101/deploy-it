use crate::state::BotStateData;
use azalea::entities::{Arrow, SpectralArrow, Trident};
use azalea::entity::Position;
use azalea::prelude::*;
use azalea::{SprintDirection, WalkDirection};
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn perform_active_logic(bot: &mut Client, data_arc: Arc<Mutex<BotStateData>>) {
    let mut data = data_arc.lock();

    // Anti-AFK Logic
    if data.is_afk {
        data.tick_counter += 1;

        let should_look = data.tick_counter % 200 == 0;
        let should_jump = data.tick_counter % 100 == 0;
        let should_walk = data.tick_counter % 400 == 0;
        let should_stop = data.tick_counter % 400 == 20;

        if should_look {
            let pos = bot.position();
            bot.look_at(pos + azalea::Vec3::new(1.0, 0.0, 0.0));
        }

        if should_jump {
            bot.jump();
        }

        if should_walk {
            bot.walk(WalkDirection::Forward);
        } else if should_stop {
            bot.walk(WalkDirection::None);
        }
    }

    // Projectile Avoidance Logic
    let bot_pos = bot.position();
    let mut dodge_vec = azalea::Vec3::default();
    let mut dodging = false;

    {
        let mut ecs = bot.ecs.lock();

        // Helper closure to process entities
        let mut process_entity = |entity_pos: azalea::Vec3| {
            let dist = bot_pos.distance_to(entity_pos);
            if dist < 10.0 {
                let to_projectile = entity_pos - bot_pos;
                if to_projectile.length_squared() > 0.1 {
                    let cross = to_projectile
                        .cross(azalea::Vec3::new(0.0, 1.0, 0.0))
                        .normalize();
                    dodge_vec = dodge_vec + cross;
                    dodging = true;
                }
            }
        };

        // Query Arrows
        let mut query_arrow = ecs.query_filtered::<&Position, With<Arrow>>();
        for pos in query_arrow.iter(&ecs) {
            process_entity(**pos);
        }

        // Query Spectral Arrows
        let mut query_spectral = ecs.query_filtered::<&Position, With<SpectralArrow>>();
        for pos in query_spectral.iter(&ecs) {
            process_entity(**pos);
        }

        // Query Tridents
        let mut query_trident = ecs.query_filtered::<&Position, With<Trident>>();
        for pos in query_trident.iter(&ecs) {
            process_entity(**pos);
        }
    }

    if dodging {
        drop(data);

        let target = bot_pos + dodge_vec * 3.0;
        bot.look_at(target);
        bot.walk(WalkDirection::Forward);
        bot.sprint(SprintDirection::Forward);
    }
}
