# build-to-hd

Move heavy build artifacts to a secondary disk and leave symlinks behind, so the
primary SSD stays free without breaking project paths.

Current implementation: a Bash helper (`scripts/build-to-hd`).
Future goal: a Rust CLI with the same behavior plus registry, status and cleanup.

## Problem

Build directories (`target/`, `node_modules/`, caches) grow quickly and fill the
SSD. Moving them manually is error-prone and hard to track.

## Current Bash helper

```bash
# Move heavy build dirs to HD (auto-detect)
build-to-hd <project-dir> [rust|node|auto]

# List projects on HD
build-to-hd list

# Remove a project's HD artifacts and symlinks
build-to-hd remove <project-dir>
```

It detects `Cargo.toml` / `package.json`, moves `target/` / `node_modules/` to
`/mnt/linux-data/<project>/...` and replaces the original with a symlink.

When a project is removed, run `build-to-hd remove <project-dir>` to delete the
HD directory and the symlinks (and the generated `.cargo/config.toml`).

See `scripts/build-to-hd`.

## Roadmap

See [docs/PLAN.md](docs/PLAN.md).

## License

MIT
EOF