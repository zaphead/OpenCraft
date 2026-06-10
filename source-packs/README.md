# Source packs

Upstream Minecraft resource packs kept for curated import into `assets/`. Nothing here is loaded at runtime.

| Pack | Import |
| ---- | ------ |
| `whimscape-26.1-r2/` | See below |

```bash
cargo run -p engine-assets --bin import-texture-pack -- \
  --pack source-packs/whimscape-26.1-r2/whimscape-26.1-r2.zip
```

To browse files locally, unzip into `extracted/` inside the pack folder (gitignored).
