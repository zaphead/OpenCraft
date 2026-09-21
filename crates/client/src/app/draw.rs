use std::f32::consts::TAU;
use std::time::Instant;

use engine_core::{partial_tick, BlockPos, Mat4, Vec2, Vec3};
use engine_render::{Camera, FrameSubmit, GroundShadow, ItemDraw, MobDraw, PlayerDraw};
use game::{as_block, at_secs, biome_at, break_ticks, Biome, Season};

use super::App;
use crate::atlas::{block_tile, item_tile, tile_uv};
use crate::mesh::{Sun, SHADOW_TONE};
use crate::particles;
use crate::ui::{self, AdvancementScreen, ContainerScreen, HudLayer, Panel, PauseMenu};

/// One full dawn-to-dawn in seconds. Presentation only: tick rate, physics,
/// and spawns never learn what time it is.
pub const DAY_SECS: f32 = 120.0;
/// Sun positions are quantized so shadows hold still between remesh waves.
const SUN_STEPS: u8 = 24;
/// Chunks re-queued per frame when the sun steps. A full refresh spreads
/// over ~a second at view distance instead of hitching one frame.
const REMESH_PER_FRAME: usize = 4;

pub struct Daylight {
    pub sun_dir: Vec3,
    pub up: bool,
    pub brightness: f32,
    pub sky: [f32; 4],
    pub step: u8,
}

/// Client-side derivation from wall time: no protocol change, no sim change.
pub fn daylight(elapsed_secs: f32) -> Daylight {
    let t = (elapsed_secs % DAY_SECS) / DAY_SECS;
    let ang = t * TAU;
    let elev = ang.sin();
    let sun_dir = Vec3::new(ang.cos(), elev, 0.35).normalize_or_zero();
    let up = elev > 0.02;
    let day = smoothstep(-0.06, 0.22, elev);
    let brightness = 0.22 + 0.78 * day;
    let night = [0.015, 0.02, 0.06];
    let noon = [0.45, 0.70, 0.95];
    let mut sky = [
        night[0] + (noon[0] - night[0]) * day,
        night[1] + (noon[1] - night[1]) * day,
        night[2] + (noon[2] - night[2]) * day,
    ];
    // Dawn/dusk ember while the sun hangs near the horizon.
    let ember = ((1.0 - elev.abs() * 3.0).clamp(0.0, 1.0)) * if elev > -0.12 { 1.0 } else { 0.0 };
    sky[0] += (0.98 - sky[0]) * ember * 0.55;
    sky[1] += (0.55 - sky[1]) * ember * 0.55;
    sky[2] += (0.35 - sky[2]) * ember * 0.55;
    Daylight {
        sun_dir,
        up,
        brightness,
        sky: [sky[0], sky[1], sky[2], 1.0],
        step: (t * SUN_STEPS as f32).floor() as u8,
    }
}

fn smoothstep(a: f32, b: f32, x: f32) -> f32 {
    let t = ((x - a) / (b - a)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn project(cam: Camera, world: Vec3, size: (u32, u32)) -> Option<(f32, f32)> {
    let vp = Mat4::from_cols_array_2d(&cam.view_proj());
    let v = vp * world.extend(1.0);
    if v.w <= 0.0 {
        return None;
    }
    let ndc = Vec3::new(v.x, v.y, v.z) / v.w;
    if ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || ndc.z < -1.0 || ndc.z > 1.0 {
        return None;
    }
    Some((
        (ndc.x * 0.5 + 0.5) * size.0 as f32,
        (1.0 - (ndc.y * 0.5 + 0.5)) * size.1 as f32,
    ))
}

impl App {
    pub(super) fn render(&mut self) {
        let frame_dt = self.last_frame.elapsed();
        self.last_frame = Instant::now();
        let dt_ms = frame_dt.as_secs_f64() * 1000.0;
        self.frame_ema_ms += (dt_ms - self.frame_ema_ms) * 0.05;
        if self.last_perf_log.elapsed().as_secs() >= 10 {
            self.last_perf_log = Instant::now();
            eprintln!(
                "perf: frame {:.2}ms (~{:.0} fps), mesh {:.2}ms x{}, chunks={} queued={} inflight={}",
                self.frame_ema_ms,
                1000.0 / self.frame_ema_ms.max(0.01),
                self.mesh_ema_ms,
                self.meshes_done,
                self.replica.loaded_chunks().count(),
                self.remesh_queue.len(),
                self.in_flight.len(),
            );
        }

        let size = match &self.window {
            Some(w) => w.inner_size(),
            None => return,
        };
        self.size = (size.width.max(1), size.height.max(1));
        let now_ms = self.start.elapsed().as_millis() as u64;
        let last_tick_ms = now_ms.saturating_sub(self.last_tick.elapsed().as_millis() as u64);
        let partial = partial_tick(now_ms, last_tick_ms);

        // Day is the server tick. 2400 ticks is still two minutes.
        let secs = self.server_tick as f32 / 20.0 + partial * 0.05;
        let mut light = daylight(secs);
        let phase = at_secs(secs);
        if phase.blood {
            light.sky = [0.45, 0.05, 0.06, 1.0];
        } else if phase.season == Season::Winter {
            light.sky[0] *= 0.85;
            light.sky[2] = (light.sky[2] + 0.15).min(1.0);
        } else if phase.season == Season::Autumn {
            light.sky[0] = (light.sky[0] + 0.08).min(1.0);
        }
        if light.step != self.sun_step {
            self.sun_step = light.step;
            self.sun = Sun {
                dir: light.sun_dir,
                up: light.up,
            };
            self.queue_sun_refresh();
            self.sun_was_up = light.up;
        }
        for _ in 0..REMESH_PER_FRAME {
            let Some(pos) = self.remesh_queue.pop_front() else {
                break;
            };
            // A fresher edit already re-queued this chunk: don't double-send.
            if self.meshed_step.get(&pos) == Some(&self.sun_step) && !self.dirty.contains(&pos) {
                continue;
            }
            self.queue_chunk(pos);
        }

        let pred = self.predict.as_ref();
        let body_pos = pred.map(|p| p.render_pos(partial)).unwrap_or(self.spawn);
        let yaw = pred.map(|p| p.body.yaw).unwrap_or(0.0);
        let pitch = pred.map(|p| p.body.pitch).unwrap_or(0.0);
        let eye = pred.map(|p| body_pos + Vec3::Y * p.body.eye_height()).unwrap_or(self.spawn);
        let look = pred.map(|p| p.body.look_dir()).unwrap_or(Vec3::Z);
        let cam_pos = if self.third {
            eye - look * 4.0 + Vec3::Y * 0.2
        } else {
            eye
        };
        let camera = Camera {
            pos: cam_pos,
            yaw,
            pitch,
            fov_y: 1.22,
            aspect: self.size.0 as f32 / self.size.1 as f32,
        };
        self.mixer.set_listener(cam_pos, yaw);

        let held = self.inv.slots.get(self.inv.selected as usize).copied().flatten();
        let held_uv = held.map(|(id, _)| {
            let (a, b) = tile_uv(item_tile(id));
            (a, b, as_block(id).is_some())
        });
        let player_shade = pred.map(|p| self.sun_shade(p.body.eye())).unwrap_or(1.0);
        let player = pred.map(|p| PlayerDraw {
            pos: body_pos,
            yaw: p.body.yaw,
            pitch: p.body.pitch,
            sneaking: p.body.sneaking,
            first_person_arm: !self.third,
            held_uv,
            shade: player_shade,
        });

        let items: Vec<ItemDraw> = self
            .items
            .values()
            .map(|(pos, item)| {
                let id = item.map(|i| i.0).unwrap_or(0);
                let (uv0, uv1) = tile_uv(item_tile(id));
                // Far drops skip the replica march: tiny on screen, and the
                // day brightness still darkens them at night.
                let shade = if pos.distance_squared(cam_pos) < 64.0 * 64.0 {
                    self.sun_shade(*pos + Vec3::Y * 0.2)
                } else {
                    1.0
                };
                ItemDraw {
                    pos: *pos,
                    yaw: self.start.elapsed().as_secs_f32(),
                    uv_min: uv0,
                    uv_max: uv1,
                    is_block: as_block(id).is_some(),
                    shade,
                }
            })
            .collect();

        let mut ground_shadows = Vec::new();
        if light.up {
            if pred.is_some() {
                self.push_shadow(&mut ground_shadows, body_pos, 0.32);
            }
            // Same 64-block cap as the shade march: far streaks are
            // sub-pixel anyway, and the loop stays flat in item count.
            for (pos, _) in self.items.values() {
                if pos.distance_squared(cam_pos) < 64.0 * 64.0 {
                    self.push_shadow(&mut ground_shadows, *pos, 0.16);
                }
            }
        }

        particles::tick(&mut self.particles, 1.0 / 60.0);
        if let Some((pos, t0, block)) = self.digging {
            let need = break_ticks(block, held.map(|h| h.0)).max(1);
            let interval = (50 * need as u128 / 8).max(40);
            if t0.elapsed().as_millis() % interval < 20 {
                particles::burst(&mut self.particles, pos.center(), block_tile(block, 4), 2, 0.04);
            }
        }

        let mut ui = ui::UiFrame::new(self.size.0 as f32, self.size.1 as f32);
        // Sun disc, drawn with the HUD but occluded like scenery: a replica
        // march from the camera toward the sun hides it behind hills, so it
        // sets instead of shining through terrain. No new pass, no widget.
        if light.up && self.sun_visible(cam_pos, light.sun_dir) {
            let sun_world = cam_pos + light.sun_dir * 400.0;
            if let Some((sx, sy)) = project(camera, sun_world, self.size) {
                let r = 14.0 * ui.scale.clamp(2.0, 3.0);
                Panel::draw(
                    &mut ui,
                    Vec2::new(sx - r, sy - r),
                    Vec2::new(sx + r, sy + r),
                    [1.0, 0.92, 0.62, 1.0],
                    0.9,
                );
            }
        }
        let play = self.overlay.is_none() && !self.paused;
        let boss = self.boss_near(body_pos);
        HudLayer::draw(
            &mut ui,
            &self.inv,
            self.health,
            if self.debug { Some(body_pos) } else { None },
            play,
            play,
            boss,
        );
        self.slot_hits.clear();
        if !self.paused {
            if let Some(kind) = self.overlay {
                self.slot_hits = ContainerScreen::draw(&mut ui, kind, &self.inv, self.snap.cursor);
            }
        }
        if self.paused {
            if self.adv_open {
                let back = AdvancementScreen::draw(&mut ui, self.adv, self.snap.cursor);
                self.pause_back = Some(back);
                self.pause_adv = None;
                self.pause_quit = None;
            } else {
                let (back, adv, quit) = PauseMenu::draw(&mut ui, self.seed, self.confirm_quit, self.snap.cursor);
                self.pause_back = Some(back);
                self.pause_adv = Some(adv);
                self.pause_quit = Some(quit);
            }
        }

        let uploads = std::mem::take(&mut self.pending_upload);
        let removes = std::mem::take(&mut self.pending_remove);
        let bright = if self.glow_boost() { (light.brightness + 0.35).min(1.0) } else { light.brightness };
        let fog_density = fog_for(biome_at(self.seed, body_pos.x as i32, body_pos.z as i32));
        let tint = frame_tint(&phase);
        let mobs = self.mob_draws();
        let particle_draws = particles::draw(&self.particles);
        let Some(renderer) = self.renderer.as_mut() else { return };
        if let Err(e) = renderer.submit(FrameSubmit {
            camera: Some(camera),
            upload_chunks: uploads,
            remove_chunks: removes,
            particles: particle_draws,
            items,
            player,
            ground_shadows,
            selection: None,
            ui: ui.quads,
            size: self.size,
            clear: light.sky,
            sun_dir: light.sun_dir,
            brightness: bright,
            fog_color: [light.sky[0], light.sky[1], light.sky[2]],
            fog_density,
            sway: secs,
            tint,
            mobs,
        }) {
            eprintln!("submit: {e}");
        }
        if let Some(w) = &self.window {
            w.request_redraw();
        }
    }

    /// How lit is this point: 1.0 in sun, SHADOW_TONE behind terrain.
    /// Same march the mesher bakes, so bodies agree with blocks.
    fn sun_shade(&self, from: Vec3) -> f32 {
        if !self.sun.up {
            return 1.0;
        }
        let mut p = from;
        let step = self.sun.dir * 0.5;
        for _ in 0..40 {
            p += step;
            let b = BlockPos::from_vec3(p);
            if b.x == from.x.floor() as i32 && b.y == from.y.floor() as i32 && b.z == from.z.floor() as i32 {
                continue;
            }
            if self.replica.block(b) != 0 {
                return SHADOW_TONE;
            }
        }
        1.0
    }

    /// Clear line from the camera toward the sun? Marches the live replica,
    /// so the sun disc sets behind hills instead of shining through them.
    fn sun_visible(&self, from: Vec3, dir: Vec3) -> bool {
        let mut p = from;
        let step = dir * 2.0;
        for _ in 0..150 {
            p += step;
            if self.replica.block(BlockPos::from_vec3(p)) != 0 {
                return false;
            }
        }
        true
    }

    /// One streak from a body's feet away from the sun, parked on the ground.
    /// Searches a full fall's worth down: jumping bodies keep their streak.
    fn push_shadow(&self, out: &mut Vec<GroundShadow>, feet: Vec3, half: f32) {
        let fx = feet.x.floor() as i32;
        let fz = feet.z.floor() as i32;
        let mut ground = None;
        for y in (feet.y.floor() as i32 - 12..=feet.y.floor() as i32 + 1).rev() {
            if self.replica.block(BlockPos::new(fx, y, fz)) != 0 {
                ground = Some(y as f32 + 1.0 + 0.02);
                break;
            }
        }
        let Some(gy) = ground else { return };
        let elev = self.sun.dir.y.max(0.12);
        let away = Vec2::new(-self.sun.dir.x, -self.sun.dir.z);
        let horizontal = away.length().max(1e-3);
        out.push(GroundShadow {
            foot: Vec3::new(feet.x, gy, feet.z),
            run: [away.x / horizontal, away.y / horizontal],
            len: (1.6 / elev).clamp(0.8, 4.5),
            half,
        });
    }

    fn boss_near(&self, pos: Vec3) -> Option<(&'static str, u8, u8)> {
        let mut best: Option<(&'static str, u8, u8, f32)> = None;
        for m in self.mobs.values() {
            if m.state != 2 {
                continue;
            }
            let (name, max) = match m.kind {
                24 => ("Treant elder", 40u8),
                26 => ("The Wyrm", 60u8),
                _ => continue,
            };
            let d = m.pos.distance(pos);
            if d > 24.0 {
                continue;
            }
            if best.map(|b| d < b.3).unwrap_or(true) {
                best = Some((name, m.hp, max, d));
            }
        }
        best.map(|(n, hp, max, _)| (n, hp, max))
    }

    fn glow_boost(&self) -> bool {
        if self.glow {
            return true;
        }
        self.inv
            .slots
            .get(self.inv.selected as usize)
            .and_then(|s| *s)
            .is_some_and(|(id, _)| id == 73)
    }

    fn mob_draws(&self) -> Vec<MobDraw> {
        self.mobs
            .values()
            .map(|m| {
                let (height, radius, color) = mob_look(m.kind, m.variant);
                MobDraw {
                    pos: m.pos,
                    yaw: m.yaw,
                    height,
                    radius,
                    color,
                }
            })
            .collect()
    }
}

fn fog_for(biome: Biome) -> f32 {
    match biome {
        Biome::Swamp | Biome::Moor => 0.045,
        Biome::Jungle | Biome::Mushroom => 0.03,
        Biome::Ash => 0.028,
        Biome::Meadow => 0.008,
        _ => 0.014,
    }
}

fn frame_tint(phase: &game::Phase) -> [f32; 3] {
    let mut tint = match phase.season {
        Season::Winter => [0.82, 0.88, 1.0],
        Season::Autumn => [1.05, 0.9, 0.72],
        Season::Spring | Season::Summer => [1.0, 1.0, 1.0],
    };
    if phase.blood {
        tint = [1.3, 0.45, 0.4];
    }
    tint
}

fn mob_look(kind: u8, variant: u8) -> (f32, f32, [f32; 3]) {
    let white = variant == 1;
    match kind {
        2 => (1.4, 0.7, [0.45, 0.28, 0.16]),
        3 => (0.8, 0.45, [0.35, 0.22, 0.16]),
        4 => (0.9, 0.35, [0.75, 0.75, 0.7]),
        5 => (0.4, 0.2, if white { [0.95, 0.95, 0.95] } else { [0.55, 0.4, 0.28] }),
        6 => (1.4, 0.4, [0.55, 0.4, 0.25]),
        7 => (0.5, 0.3, [0.85, 0.4, 0.15]),
        8 | 9 | 28 => (0.3, 0.15, [0.2, 0.2, 0.22]),
        10 => (0.2, 0.2, [0.3, 0.55, 0.7]),
        11 => (0.3, 0.2, [0.3, 0.6, 0.25]),
        12 => (0.2, 0.12, [0.6, 0.9, 0.3]),
        13 => (0.2, 0.12, [0.9, 0.75, 0.15]),
        14 => (1.5, 0.6, [0.4, 0.26, 0.14]),
        15 => (1.1, 0.6, [0.55, 0.4, 0.2]),
        16 => (1.2, 0.25, [0.3, 0.55, 0.25]),
        17 => (1.8, 0.3, [0.62, 0.5, 0.32]),
        18 => (1.9, 0.3, [0.25, 0.35, 0.28]),
        19 => (1.8, 0.3, [0.7, 0.85, 0.95]),
        20 => (0.6, 0.2, [0.7, 0.2, 0.1]),
        21 => (0.6, 0.3, [0.45, 0.2, 0.55]),
        22 => (0.9, 0.45, [0.5, 0.32, 0.15]),
        23 => (2.0, 0.4, [0.45, 0.45, 0.48]),
        24 => (3.0, 0.7, [0.25, 0.4, 0.18]),
        25 => (1.5, 2.0, [0.55, 0.65, 0.75]),
        26 => (1.2, 1.0, [0.55, 0.12, 0.08]),
        27 => (1.8, 0.3, [0.35, 0.3, 0.55]),
        _ => (1.0, 0.3, [0.8, 0.8, 0.8]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_is_two_minutes_from_dawn() {
        // t=0 rises in the east; noon is full bright; midnight is the
        // navigable floor; the cycle closes exactly.
        let dawn = daylight(0.0);
        assert!(dawn.sun_dir.x > 0.9, "dawn sun not east: {:?}", dawn.sun_dir);
        assert!((-0.05..0.05).contains(&dawn.sun_dir.y));
        assert_eq!(daylight(DAY_SECS).step, dawn.step);
        assert_eq!(daylight(DAY_SECS).sun_dir, dawn.sun_dir);
        let noon = daylight(DAY_SECS * 0.25);
        assert!((noon.brightness - 1.0).abs() < 1e-6);
        assert!(noon.sun_dir.y > 0.9);
        let midnight = daylight(DAY_SECS * 0.75);
        assert!((midnight.brightness - 0.22).abs() < 1e-9);
        assert!(!midnight.up);
        for t in [0.0, 15.0, 30.0, 60.0, 90.0, 105.0] {
            let b = daylight(t).brightness;
            assert!((0.22..=1.0).contains(&b), "brightness {b} at t={t}");
        }
    }
}
