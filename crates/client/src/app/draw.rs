use engine_core::{partial_tick, Vec3};
use engine_render::{Camera, FrameSubmit, ItemDraw, PlayerDraw};
use game::{as_block, break_ticks};

use super::App;
use crate::atlas::{block_tile, item_tile, tile_uv};
use crate::particles;
use crate::ui::{self, ContainerScreen, HudLayer, PauseMenu};

impl App {
    pub(super) fn render(&mut self) {
        let Some(renderer) = self.renderer.as_mut() else { return };
        let Some(window) = &self.window else { return };
        let size = window.inner_size();
        self.size = (size.width.max(1), size.height.max(1));
        let now_ms = self.start.elapsed().as_millis() as u64;
        let last_tick_ms = now_ms.saturating_sub(self.last_tick.elapsed().as_millis() as u64);
        let partial = partial_tick(now_ms, last_tick_ms);
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
        let player = pred.map(|p| PlayerDraw {
            pos: body_pos,
            yaw: p.body.yaw,
            pitch: p.body.pitch,
            sneaking: p.body.sneaking,
            first_person_arm: !self.third,
            held_uv,
        });

        let items: Vec<ItemDraw> = self
            .items
            .values()
            .map(|(pos, item)| {
                let id = item.map(|i| i.0).unwrap_or(0);
                let (uv0, uv1) = tile_uv(item_tile(id));
                ItemDraw {
                    pos: *pos,
                    yaw: self.start.elapsed().as_secs_f32(),
                    uv_min: uv0,
                    uv_max: uv1,
                    is_block: as_block(id).is_some(),
                }
            })
            .collect();

        particles::tick(&mut self.particles, 1.0 / 60.0);
        if let Some((pos, t0, block)) = self.digging {
            let need = break_ticks(block, held.map(|h| h.0)).max(1);
            let interval = (50 * need as u128 / 8).max(40);
            if t0.elapsed().as_millis() % interval < 20 {
                particles::burst(&mut self.particles, pos.center(), block_tile(block, 4), 2, 0.04);
            }
        }

        let mut ui = ui::UiFrame::new(self.size.0 as f32, self.size.1 as f32);
        let play = self.overlay.is_none() && !self.paused;
        HudLayer::draw(
            &mut ui,
            &self.inv,
            self.health,
            if self.debug { Some(body_pos) } else { None },
            play,
            play,
        );
        self.slot_hits.clear();
        if !self.paused {
            if let Some(kind) = self.overlay {
                self.slot_hits = ContainerScreen::draw(&mut ui, kind, &self.inv, self.snap.cursor);
            }
        }
        if self.paused {
            let (back, quit) = PauseMenu::draw(&mut ui, self.seed, self.confirm_quit, self.snap.cursor);
            self.pause_back = Some(back);
            self.pause_quit = Some(quit);
        }

        let uploads = std::mem::take(&mut self.pending_upload);
        let removes = std::mem::take(&mut self.pending_remove);
        if let Err(e) = renderer.submit(FrameSubmit {
            camera: Some(camera),
            upload_chunks: uploads,
            remove_chunks: removes,
            particles: particles::draw(&self.particles),
            items,
            player,
            ui: ui.quads,
            size: self.size,
            clear: [0.45, 0.70, 0.95, 1.0],
        }) {
            eprintln!("submit: {e}");
        }
        window.request_redraw();
    }
}
