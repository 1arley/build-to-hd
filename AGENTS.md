# Agent Guide — build-to-hd

This file is for AI coding agents working on this repository without prior session context.

## Project summary

`build-to-hd` moves heavy build artifacts (Rust `target/`, Node `node_modules/`,
etc.) to a secondary disk (`/mnt/linux-data`) and leaves symlinks in the project,
so the primary SSD stays free without breaking project paths or tooling.

Current implementation: a Bash helper (`scripts/build-to-hd`).
Future goal: a Rust CLI with registry, status, doctor and cleanup.

## Repository layout

```
.
├── AGENTS.md
├── Cargo.toml          # Rust skeleton (not built yet)
├── src/main.rs         # Rust skeleton (prints planned commands)
├── .cargo/config.toml  # target-dir -> /mnt/linux-data/build-to-hd/target
├── scripts/build-to-hd # Bash helper (reference implementation)
├── docs/PLAN.md        # Roadmap (Phase 0..4)
├── docs/USAGE.md       # Detailed usage
├── docs/ARCHITECTURE.md
└── README.md
```

## Key paths and conventions

- Secondary disk mount: `/mnt/linux-data`
- Per-project HD dir: `/mnt/linux-data/<project-name>/`
- Rust artifacts: `/mnt/linux-data/<name>/target`
- Node artifacts: `/mnt/linux-data/<name>/node_modules`
- Package caches (npm/pnpm/cargo registry) stay global by design.
- Symlinks are used (not bind mounts) for simplicity and reversibility.
- Operations must be idempotent and must not delete source code.

## Current helper commands

```bash
build-to-hd <project-dir> [rust|node|auto]
build-to-hd remove <project-dir>
```

Auto-detection uses `Cargo.toml` / `package.json`.

## Safety rules for agents

- Never delete project source directories.
- `remove` deletes only symlinks and the per-project HD directory.
- Always verify `/mnt/linux-data` is mounted before operations.
- Prefer dry-run when adding new behavior.
- Keep Bash helper as reference until Rust reaches parity.

## Roadmap

See `docs/PLAN.md`. Current phase: Phase 0 complete, Phase 1 next.
EOF