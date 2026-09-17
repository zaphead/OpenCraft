use super::*;
use crate::embed::ServerConfig;
use engine_core::BlockPos;
use game::break_ticks;
use protocol::{ClientPlay, ServerPlay};
use std::sync::mpsc;

#[test]
    fn join_emits_chunks_and_seed() {
        let mut sim = Sim::new(ServerConfig::new(42));
        let (tx, rx) = mpsc::channel();
        sim.join_player(&tx);
        let mut saw_join = false;
        let mut chunks = 0;
        while let Ok(m) = rx.try_recv() {
            match m {
                ServerPlay::JoinGame { seed, .. } => {
                    assert_eq!(seed, 42);
                    saw_join = true;
                }
                ServerPlay::Chunk { .. } => chunks += 1,
                ServerPlay::KeepTick { .. }
                | ServerPlay::UnloadChunk { .. }
                | ServerPlay::BlockUpdate { .. }
                | ServerPlay::PlayerCorrection { .. }
                | ServerPlay::EntitySpawn { .. }
                | ServerPlay::EntityPos { .. }
                | ServerPlay::EntityDespawn { .. }
                | ServerPlay::Inventory(_)
                | ServerPlay::OpenWindow { .. }
                | ServerPlay::CloseWindow
                | ServerPlay::Particles { .. }
                | ServerPlay::Sound { .. }
                | ServerPlay::Hurt { .. }
                | ServerPlay::Respawned { .. } => {}
            }
        }
        assert!(saw_join);
        assert!(chunks >= 1);
    }

    #[test]
    fn break_dirt_spawns_item() {
        let mut sim = Sim::new(ServerConfig::new(1));
        let (tx, rx) = mpsc::channel();
        sim.join_player(&tx);
        while rx.try_recv().is_ok() {}
        let feet = BlockPos::from_vec3(sim.player.body.pos);
        let mut target = None;
        for y in (0..feet.y).rev() {
            let p = BlockPos::new(feet.x, y, feet.z);
            if sim.world.block(p) != 0 {
                target = Some(p);
                break;
            }
        }
        let pos = target.expect("generated spawn sits on solid ground");
        let need = break_ticks(sim.world.block(pos), None);
        sim.handle(&ClientPlay::StartDig { pos }, &tx);
        for _ in 0..need + 2 {
            sim.tick(&[], &tx);
        }
        let mut item = false;
        let mut air = false;
        while let Ok(m) = rx.try_recv() {
            match m {
                ServerPlay::EntitySpawn { kind: 1, .. } => item = true,
                ServerPlay::BlockUpdate { id: 0, .. } => air = true,
                ServerPlay::JoinGame { .. }
                | ServerPlay::KeepTick { .. }
                | ServerPlay::Chunk { .. }
                | ServerPlay::UnloadChunk { .. }
                | ServerPlay::BlockUpdate { .. }
                | ServerPlay::PlayerCorrection { .. }
                | ServerPlay::EntitySpawn { .. }
                | ServerPlay::EntityPos { .. }
                | ServerPlay::EntityDespawn { .. }
                | ServerPlay::Inventory(_)
                | ServerPlay::OpenWindow { .. }
                | ServerPlay::CloseWindow
                | ServerPlay::Particles { .. }
                | ServerPlay::Sound { .. }
                | ServerPlay::Hurt { .. }
                | ServerPlay::Respawned { .. } => {}
            }
        }
        assert!(air);
        assert!(item);
    }

    #[test]
    fn death_drops_and_respawns() {
        let mut sim = Sim::new(ServerConfig::new(7));
        let (tx, rx) = mpsc::channel();
        sim.join_player(&tx);
        while rx.try_recv().is_ok() {}
        sim.player.hotbar[0] = Some((game::blocks::DIRT, 12));
        sim.player.health = 0;
        sim.tick(&[], &tx);
        let mut dropped = false;
        let mut respawned = false;
        while let Ok(m) = rx.try_recv() {
            match m {
                ServerPlay::EntitySpawn { kind: 1, .. } => dropped = true,
                ServerPlay::Respawned { health, .. } => {
                    respawned = true;
                    assert_eq!(health, 20);
                }
                ServerPlay::JoinGame { .. }
                | ServerPlay::KeepTick { .. }
                | ServerPlay::Chunk { .. }
                | ServerPlay::UnloadChunk { .. }
                | ServerPlay::BlockUpdate { .. }
                | ServerPlay::PlayerCorrection { .. }
                | ServerPlay::EntitySpawn { .. }
                | ServerPlay::EntityPos { .. }
                | ServerPlay::EntityDespawn { .. }
                | ServerPlay::Inventory(_)
                | ServerPlay::OpenWindow { .. }
                | ServerPlay::CloseWindow
                | ServerPlay::Particles { .. }
                | ServerPlay::Sound { .. }
                | ServerPlay::Hurt { .. } => {}
            }
        }
        assert!(dropped);
        assert!(respawned);
        assert!(sim.player.hotbar.iter().all(Option::is_none));
    }

    #[test]
    fn inventory_crafts_planks() {
        let mut sim = Sim::new(ServerConfig::new(3));
        let (tx, rx) = mpsc::channel();
        sim.join_player(&tx);
        while rx.try_recv().is_ok() {}
        sim.player.hotbar[0] = Some((game::blocks::LOG, 1));
        sim.handle(&ClientPlay::OpenInventory, &tx);
        sim.handle(
            &ClientPlay::ClickSlot {
                slot: 0,
                button: 0,
                shift: false,
            },
            &tx,
        );
        sim.handle(
            &ClientPlay::ClickSlot {
                slot: 36,
                button: 0,
                shift: false,
            },
            &tx,
        );
        sim.handle(
            &ClientPlay::ClickSlot {
                slot: 40,
                button: 0,
                shift: false,
            },
            &tx,
        );
        let w = sim.player.window.as_ref().expect("OpenInventory leaves a window");
        assert_eq!(w.cursor, Some((game::blocks::PLANKS, 4)));
    }
