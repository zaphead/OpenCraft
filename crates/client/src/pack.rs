use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::atlas::{self, blit_tile};

pub struct Pack {
    pub atlas: Vec<u8>,
    pub skin: Vec<u8>,
}

#[derive(Default)]
pub struct PackSounds {
    pub step: Vec<u16>,
    pub break_b: Vec<u16>,
    pub place: Vec<u16>,
    pub hurt: Vec<u16>,
    pub pickup: Vec<u16>,
}

pub fn load(path: Option<&str>) -> Pack {
    let mut atlas = atlas::procedural_atlas();
    let mut skin = default_skin();
    // Out of the box the game wears the vendored pack; a --pack path or
    // OPENCRAFT_PACK env var overrides it. No pack still plays procedural.
    let root = path.map(PathBuf::from).or_else(default_pack_dir);
    let Some(root) = root.filter(|r| r.exists()) else {
        if path.is_some() {
            eprintln!("pack path missing: {} (procedural fallback)", path.unwrap_or_default());
        } else {
            eprintln!("pack: no assets/pack found (procedural fallback)");
        }
        return Pack { atlas, skin };
    };
    let names: &[(&[&str], u32)] = &[
        (&["block/grass_block_top.png", "block/grass_top.png", "blocks/grass_top.png"], atlas::T_GRASS_TOP),
        (&["block/grass_block_side.png", "block/grass_side.png"], atlas::T_GRASS_SIDE),
        (&["block/dirt.png", "blocks/dirt.png"], atlas::T_DIRT),
        (&["block/stone.png", "blocks/stone.png"], atlas::T_STONE),
        (&["block/cobblestone.png", "blocks/cobblestone.png"], atlas::T_COBBLE),
        (&["block/oak_log.png", "block/log_oak.png", "blocks/log_oak.png"], atlas::T_LOG),
        (&["block/oak_log_top.png", "block/log_oak_top.png"], atlas::T_LOG_TOP),
        (&["block/oak_leaves.png", "block/leaves_oak.png", "blocks/leaves_oak.png"], atlas::T_LEAVES),
        (&["block/oak_planks.png", "block/planks_oak.png", "blocks/planks_oak.png"], atlas::T_PLANKS),
        (&["block/crafting_table_top.png"], atlas::T_TABLE_TOP),
        (&["block/crafting_table_front.png"], atlas::T_TABLE_FRONT),
        (&["block/crafting_table_side.png"], atlas::T_TABLE_SIDE),
        (&["block/chest_front.png", "entity/chest/normal.png"], atlas::T_CHEST),
        (&["item/stick.png", "items/stick.png"], atlas::T_STICK),
        (&["item/wooden_pickaxe.png", "items/wood_pickaxe.png"], atlas::T_WOOD_PICK),
        (&["item/wooden_axe.png"], atlas::T_WOOD_AXE),
        (&["item/wooden_shovel.png"], atlas::T_WOOD_SHOVEL),
        (&["item/stone_pickaxe.png"], atlas::T_STONE_PICK),
        (&["item/stone_axe.png"], atlas::T_STONE_AXE),
        (&["item/stone_shovel.png"], atlas::T_STONE_SHOVEL),
    ];
    for (cands, tile) in names {
        if let Some(img) = first_image(&root, cands) {
            blit_tile(&mut atlas, *tile, &img.data, img.w, img.h);
        }
    }
    let skin_paths = [
        "entity/player/wide/steve.png",
        "entity/player/slim/steve.png",
        "entity/steve.png",
        "mob/steve.png",
    ];
    if let Some(img) = first_image(&root, &skin_paths) {
        skin = resize_rgba(&img.data, img.w, img.h, 64, 64);
    }

    Pack { atlas, skin }
}

struct Image {
    w: u32,
    h: u32,
    data: Vec<u8>,
}

/// Vendored pack lookup: `<cwd>/assets/pack`, then next to the client binary.
/// Returns `None` when nothing is vendored, which is a supported way to play.
pub fn default_pack_dir() -> Option<PathBuf> {
    let rel = Path::new("assets/pack");
    if rel.is_dir() {
        return Some(rel.to_path_buf());
    }
    if let Ok(exe) = std::env::current_exe() {
        // `target/debug/client` layout: every ancestor up to the workspace
        // root is visited, so no `..` joins are needed.
        for up in exe.ancestors().skip(1).take(4) {
            let d = up.join("assets/pack");
            if d.is_dir() {
                return Some(d);
            }
        }
    }
    None
}

fn first_image(root: &Path, rels: &[&str]) -> Option<Image> {
    for rel in rels {
        for base in [
            root.join("assets/minecraft/textures").join(rel),
            root.join("assets/minecraft").join(rel),
            root.join("textures").join(rel),
            root.join(rel),
        ] {
            if let Some(img) = decode_image(&base) {
                return Some(img);
            }
        }
    }
    None
}

fn decode_image(path: &Path) -> Option<Image> {
    if !path.is_file() {
        return None;
    }
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    match ext.as_str() {
        "png" => decode_png(path),
        "jpg" | "jpeg" => decode_jpg(path),
        _ => decode_png(path).or_else(|| decode_jpg(path)),
    }
}

fn decode_png(path: &Path) -> Option<Image> {
    let f = File::open(path).ok()?;
    let decoder = png::Decoder::new(BufReader::new(f));
    let mut reader = decoder.read_info().ok()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).ok()?;
    let w = info.width;
    let h = info.height;
    let data = match info.color_type {
        png::ColorType::Rgba => buf[..info.buffer_size()].to_vec(),
        png::ColorType::Rgb => {
            let mut o = Vec::with_capacity((w * h * 4) as usize);
            for c in buf[..info.buffer_size()].chunks(3) {
                o.extend_from_slice(&[c[0], c[1], c[2], 255]);
            }
            o
        }
        png::ColorType::Grayscale => {
            let mut o = Vec::with_capacity((w * h * 4) as usize);
            for g in &buf[..info.buffer_size()] {
                o.extend_from_slice(&[*g, *g, *g, 255]);
            }
            o
        }
        png::ColorType::GrayscaleAlpha => {
            let mut o = Vec::with_capacity((w * h * 4) as usize);
            for c in buf[..info.buffer_size()].chunks(2) {
                o.extend_from_slice(&[c[0], c[0], c[0], c[1]]);
            }
            o
        }
        png::ColorType::Indexed => return None,
    };
    Some(Image { w, h, data })
}

fn decode_jpg(path: &Path) -> Option<Image> {
    let f = File::open(path).ok()?;
    let mut decoder = jpeg_decoder::Decoder::new(BufReader::new(f));
    let pixels = decoder.decode().ok()?;
    let info = decoder.info()?;
    let w = info.width as u32;
    let h = info.height as u32;
    let mut data = Vec::with_capacity((w * h * 4) as usize);
    match info.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => {
            for c in pixels.chunks(3) {
                data.extend_from_slice(&[c[0], c[1], c[2], 255]);
            }
        }
        jpeg_decoder::PixelFormat::L8 => {
            for g in pixels {
                data.extend_from_slice(&[g, g, g, 255]);
            }
        }
        jpeg_decoder::PixelFormat::CMYK32 => return None,
        jpeg_decoder::PixelFormat::L16 => return None,
    }
    Some(Image { w, h, data })
}

fn resize_rgba(src: &[u8], sw: u32, sh: u32, dw: u32, dh: u32) -> Vec<u8> {
    let mut out = vec![0u8; (dw * dh * 4) as usize];
    for y in 0..dh {
        for x in 0..dw {
            let sx = x * sw / dw;
            let sy = y * sh / dh;
            let si = ((sy * sw + sx) * 4) as usize;
            let di = ((y * dw + x) * 4) as usize;
            if si + 3 < src.len() {
                out[di..di + 4].copy_from_slice(&src[si..si + 4]);
            }
        }
    }
    out
}

fn default_skin() -> Vec<u8> {
    let mut s = vec![0u8; 64 * 64 * 4];
    fill_rect(&mut s, 8, 8, 8, 8, [210, 170, 130, 255]); // head
    fill_rect(&mut s, 20, 20, 8, 12, [40, 140, 130, 255]); // body unique teal
    fill_rect(&mut s, 44, 20, 4, 12, [210, 170, 130, 255]);
    fill_rect(&mut s, 4, 20, 4, 12, [90, 60, 40, 255]);
    s
}

fn fill_rect(s: &mut [u8], x: u32, y: u32, w: u32, h: u32, c: [u8; 4]) {
    for yy in y..y + h {
        for xx in x..x + w {
            let i = ((yy * 64 + xx) * 4) as usize;
            s[i..i + 4].copy_from_slice(&c);
        }
    }
}

pub fn load_oggs(root: &Path, mixer: &mut engine_audio::Mixer) -> PackSounds {
    let mut sounds = PackSounds::default();
    let mut next = 1u16;
    // Real packs number their variants (grass1..4) and span two eras of
    // paths (dig/grass1 vs block/grass/break1); probe every spelling and
    // register the first hit per slot. Missing everything stays silent.
    let search: &[(&[&str], &str)] = &[
        (&["dig/grass", "block/grass/break"], "break"),
        (&["dig/stone", "block/stone/break", "random/break"], "break"),
        (&["dig/wood", "block/wood/break"], "break"),
        (&["step/grass", "block/grass/step"], "step"),
        (&["step/stone", "block/stone/step"], "step"),
        (&["step/wood", "block/wood/step"], "step"),
        (&["block/grass/place", "dig/grass"], "place"),
        (&["block/stone/place", "dig/stone"], "place"),
        (&["block/wood/place", "dig/wood"], "place"),
        (&["damage/hit", "random/classic_hurt", "entity/player/hurt", "random/hurt"], "hurt"),
        (&["random/pop", "entity/item/pickup"], "pickup"),
    ];
    for &(rels, kind) in search {
        if load_first_ogg(root, mixer, rels, next).is_some() {
            match kind {
                "step" => sounds.step.push(next),
                "break" => sounds.break_b.push(next),
                "place" => sounds.place.push(next),
                "hurt" => sounds.hurt.push(next),
                "pickup" => sounds.pickup.push(next),
                _ => {}
            }
            next += 1;
        }
    }
    if sounds.break_b.is_empty() && sounds.step.is_empty() {
        eprintln!("pack: no usable sounds under {}", root.display());
    }
    sounds
}

/// First decodable ogg wins. Numbered spares (grass2..6) cover a missing
/// or corrupt grass1: a bad file never kills the slot, the next spelling is
/// tried instead. Keeps mixer ids 1:1 with PackSounds slots.
fn load_first_ogg(root: &Path, mixer: &mut engine_audio::Mixer, rels: &[&str], id: u16) -> Option<()> {
    for rel in rels {
        // Vanilla-numbered variants first, then the bare name.
        for n in ["1", "2", "3", "4", "5", "6", ""] {
            for ext in ["ogg", "OGG"] {
                let p = root
                    .join("assets/minecraft/sounds")
                    .join(format!("{rel}{n}.{ext}"));
                let p2 = root.join("sounds").join(format!("{rel}{n}.{ext}"));
                for path in [p, p2] {
                    if !path.is_file() {
                        continue;
                    }
                    let Ok(f) = std::fs::File::open(&path) else {
                        continue;
                    };
                    let Ok(clip) = engine_audio::decode_ogg(std::io::BufReader::new(f)) else {
                        continue;
                    };
                    mixer.register(id, clip);
                    return Some(());
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_load_never_blows_up() {
        let pack = load(None);
        assert_eq!(pack.atlas.len(), (atlas::ATLAS * atlas::ATLAS * 4) as usize);
        assert_eq!(pack.skin.len(), 64 * 64 * 4);
        // Wherever the vendored pack is checked out, it must leave a mark:
        // same world, different pixels proves art (not fallback) is worn.
        if default_pack_dir().is_some() {
            assert_ne!(
                pack.atlas,
                atlas::procedural_atlas(),
                "vendored pack left no mark on the atlas"
            );
        }
    }

    #[test]
    fn pack_sounds_land_or_stay_silent() {
        // With the vendored pack: every gameplay slot speaks. Without one:
        // silence, never a panic, never a half-registered id.
        let mut mixer = engine_audio::Mixer::silent();
        match default_pack_dir() {
            Some(root) => {
                let sounds = load_oggs(&root, &mut mixer);
                assert!(!sounds.break_b.is_empty(), "pack break silent");
                assert!(!sounds.step.is_empty(), "pack step silent");
                assert!(!sounds.place.is_empty(), "pack place silent");
                assert!(!sounds.hurt.is_empty(), "pack hurt silent");
                assert!(!sounds.pickup.is_empty(), "pack pickup silent");
            }
            None => {
                let sounds = load_oggs(std::path::Path::new("no-such-pack"), &mut mixer);
                assert!(sounds.break_b.is_empty());
                assert!(sounds.step.is_empty());
                assert!(sounds.place.is_empty());
                assert!(sounds.hurt.is_empty());
                assert!(sounds.pickup.is_empty());
            }
        }
    }

    /// Scratch pack root under the OS temp dir. Unique per process so
    /// parallel test binaries never share it; best-effort cleanup.
    fn scratch_pack(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("opencraft-pack-test-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("assets/minecraft/sounds/dig")).expect("scratch pack");
        dir
    }

    #[test]
    fn corrupt_first_file_falls_back_to_spares() {
        // grass1 is garbage and grass2 is a real clip borrowed from the
        // vendored pack: the break slot must speak via the spare. Runs
        // wherever the pack is checked out; without it there is no decodable
        // spare to fall back to, so only the always-run silence test applies.
        let Some(vendored) = default_pack_dir() else {
            return;
        };
        let dir = scratch_pack("fallback");
        std::fs::write(dir.join("assets/minecraft/sounds/dig/grass1.ogg"), b"not an ogg").expect("scratch");
        for spare in ["dig/grass2.ogg", "dig/grass3.ogg", "dig/grass4.ogg"] {
            let src = vendored.join("assets/minecraft/sounds").join(spare);
            if src.is_file() {
                std::fs::copy(src, dir.join("assets/minecraft/sounds").join(spare)).expect("scratch");
                break;
            }
        }
        let mut mixer = engine_audio::Mixer::silent();
        let sounds = load_oggs(&dir, &mut mixer);
        assert!(!sounds.break_b.is_empty(), "corrupt grass1 killed the slot");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn garbage_pack_stays_silent() {
        // Always runs, pack or not: undecodable files are skipped, missing
        // files are skipped, the slots stay empty, nothing panics.
        let dir = scratch_pack("garbage");
        std::fs::write(dir.join("assets/minecraft/sounds/dig/grass1.ogg"), b"not an ogg").expect("scratch");
        let mut mixer = engine_audio::Mixer::silent();
        let sounds = load_oggs(&dir, &mut mixer);
        assert!(sounds.break_b.is_empty());
        assert!(sounds.step.is_empty());
        assert!(sounds.place.is_empty());
        assert!(sounds.hurt.is_empty());
        assert!(sounds.pickup.is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
