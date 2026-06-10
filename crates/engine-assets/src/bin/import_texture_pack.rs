//! Import curated blocks from a Minecraft resource pack into engine assets.
use std::env;
use std::path::PathBuf;

use engine_assets::{assets_dir, import_texture_pack, load_manifest};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut pack_path: Option<PathBuf> = None;
    let mut manifest_path: Option<PathBuf> = None;
    let mut assets_root: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--pack" => {
                i += 1;
                pack_path = Some(PathBuf::from(args.get(i).expect("--pack requires a path")));
            }
            "--manifest" => {
                i += 1;
                manifest_path = Some(PathBuf::from(
                    args.get(i).expect("--manifest requires a path"),
                ));
            }
            "--assets" => {
                i += 1;
                assets_root = Some(PathBuf::from(args.get(i).expect("--assets requires a path")));
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unknown argument: {other}");
                print_usage();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    let pack_path = pack_path.unwrap_or_else(|| {
        eprintln!("missing --pack <zip-or-directory>");
        print_usage();
        std::process::exit(1);
    });
    let manifest_path = manifest_path.unwrap_or_else(|| {
        assets_dir(env!("CARGO_MANIFEST_DIR")).join("import/manifest.toml")
    });
    let assets_root = assets_root.unwrap_or_else(|| assets_dir(env!("CARGO_MANIFEST_DIR")));

    let manifest = load_manifest(&manifest_path).unwrap_or_else(|error| {
        eprintln!("{error}");
        std::process::exit(1);
    });

    let report = import_texture_pack(&pack_path, &manifest, &assets_root).unwrap_or_else(|error| {
        eprintln!("import failed: {error}");
        std::process::exit(1);
    });

    println!(
        "imported {} blocks [{}], {} colormaps [{}]",
        report.blocks_imported.len(),
        report.blocks_imported.join(", "),
        report.colormaps_imported.len(),
        report.colormaps_imported.join(", ")
    );
}

fn print_usage() {
    eprintln!(
        "usage: import-texture-pack --pack <zip-or-dir> [--manifest path] [--assets path]\n\
         \n\
         Converts Minecraft block models/textures into engine cube_v1 assets.\n\
         Block ids and registry entries must already exist under assets/blocks/."
    );
}
