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
build-to-hd <project-dir> [rust|node|auto]
```

It detects `Cargo.toml` / `package.json`, moves `target/` / `node_modules/` to
`/mnt/linux-data/<project>/...` and replaces the original with a symlink.

See `scripts/build-to-hd`.

## Roadmap

See [docs/PLAN.md](docs/PLAN.md).

## License

MIT
EOF