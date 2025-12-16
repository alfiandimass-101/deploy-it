use azalea::pathfinder::PathfinderClientExt;
use azalea::prelude::*;
use azalea::{SprintDirection, WalkDirection};
// Guessing path: metadata contains the specific entity components
use crate::state::BotStateData;
use azalea::ecs::prelude::*; // For With, Query
use azalea::entity::Position;
use azalea::entity::metadata::{Arrow, SpectralArrow, Trident}; // Trident is ThrownTrident usually?
use parking_lot::Mutex;
use std::sync::Arc;

pub async fn perform_active_logic(bot: &mut Client, data_arc: Arc<Mutex<BotStateData>>) {
    // Projectile Avoidance Logic (Highest Priority)
    let bot_pos = bot.position();
    let mut dodge_vec = azalea::Vec3::default();
    let mut dodging = false;

    {
        let mut ecs = bot.ecs.lock();

        let mut process_entity = |entity_pos: azalea::Vec3| {
            let dist = bot_pos.distance_to(entity_pos);
            if dist < 10.0 {
                let to_projectile = entity_pos - bot_pos;
                if to_projectile.length_squared() > 0.1 {
                    // Manual cross product with UP (0,1,0)
                    let cross =
                        azalea::Vec3::new(-to_projectile.z, 0.0, to_projectile.x).normalize();
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

        // Query Trident
        let mut query_trident = ecs.query_filtered::<&Position, With<Trident>>();
        for pos in query_trident.iter(&ecs) {
            process_entity(**pos);
        }
    }

    if dodging {
        // Stop any pathfinding immediately to take full control
        bot.force_stop_pathfinding();

        let target = bot_pos + dodge_vec * 3.0;
        bot.look_at(target);
        bot.walk(WalkDirection::Forward);
        bot.sprint(SprintDirection::Forward);
        return;
    }

    let target_to_go = {
        let mut data = data_arc.lock();

        // Anti-AFK Logic
        if data.is_afk {
            let mut needs_new_target = false;

            if let Some(target) = data.afk_target {
                let pos = bot.position();
                let dist = pos.distance_to(target);

                // Reached target?
                if dist < 1.0 {
                    needs_new_target = true;
                }

                // Timeout?
                if let Some(timer) = data.afk_timer {
                    if timer.elapsed() >= std::time::Duration::from_secs(6) {
                        needs_new_target = true;
                    }
                } else {
                    // Should have a timer if we have a target, but just in case
                    data.afk_timer = Some(std::time::Instant::now());
                }
            } else {
                needs_new_target = true;
            }

            if needs_new_target {
                use rand::Rng; // Ensure rand is imported or available
                let mut rng = rand::rng();

                let pos = bot.position();
                // Random offset 5-10 blocks
                // We want a random point in an annulus (ring) between 5 and 10 radius.
                // Simplified: random angle, random distance 5-10
                let angle = rng.random_range(0.0..std::f64::consts::TAU);
                let distance = rng.random_range(5.0..10.0);

                let offset_x = angle.cos() * distance;
                let offset_z = angle.sin() * distance;

                let new_target = pos + azalea::Vec3::new(offset_x, 0.0, offset_z);
                let target_block_pos = azalea::BlockPos::from(new_target);

                data.afk_target = Some(new_target);
                data.afk_timer = Some(std::time::Instant::now());

                Some(target_block_pos)
            } else {
                None
            }
        } else {
            None
        }
    };

    if let Some(target_block_pos) = target_to_go {
        let opts = azalea::pathfinder::PathfinderOpts::new().allow_mining(false);
        bot.start_goto_with_opts(
            azalea::pathfinder::goals::BlockPosGoal(target_block_pos),
            opts,
        );
    }
}
