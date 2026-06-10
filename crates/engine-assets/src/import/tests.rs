#[cfg(test)]
mod whimscape {
    use std::path::Path;

    use crate::import::{import_texture_pack, load_manifest};
    use crate::material::pack_block_materials;
    use crate::server::{assets_dir, blocks_asset_path};
    use crate::{load_block_registry, textures_asset_path};

    const WHIMSCAPE: &str = "/Users/spencerterry/Downloads/Whimscape 26.1 r2.zip";

    #[test]
    #[ignore = "requires local Whimscape pack zip"]
    fn imports_whimscape_blocks_into_temp_assets() {
        if !Path::new(WHIMSCAPE).is_file() {
            return;
        }

        let temp = tempfile::tempdir().expect("tempdir");
        let assets = temp.path().join("assets");
        let blocks_dir = assets.join("blocks");
        std::fs::create_dir_all(&blocks_dir).expect("blocks dir");

        let repo_blocks = blocks_asset_path(env!("CARGO_MANIFEST_DIR"));
        for name in ["grass", "dirt", "stone", "leaves", "air"] {
            std::fs::copy(
                repo_blocks.join(format!("{name}.toml")),
                blocks_dir.join(format!("{name}.toml")),
            )
            .expect("copy block toml");
        }

        let manifest =
            load_manifest(&assets_dir(env!("CARGO_MANIFEST_DIR")).join("import/manifest.toml"))
                .expect("manifest");

        let report = import_texture_pack(Path::new(WHIMSCAPE), &manifest, &assets)
            .expect("import");
        assert_eq!(report.blocks_imported.len(), 4);

        let registry = load_block_registry(&blocks_dir);
        let textures = textures_asset_path(assets.to_str().unwrap());
        pack_block_materials(&textures, &registry).expect("pack imported materials");
    }
}
