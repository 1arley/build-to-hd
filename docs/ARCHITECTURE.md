# Architecture — build-to-hd

## Overview

A lightweight tool that relocates heavy build directories to a secondary disk
while preserving original paths via symlinks.

## Components

- **Helper (Bash)**: `scripts/build-to-hd`
  - Detects ecosystems by manifest files.
  - Moves directories and creates symlinks.
  - Writes per-project Rust cargo config.
  - Supports `remove`.

- **Rust CLI (planned)**: `src/` + `Cargo.toml`
  - Will provide typed commands, registry, status, doctor, clean.

## Data flow

1. Input: project directory.
2. Detect manifests (`Cargo.toml`, `package.json`).
3. Ensure HD mounted.
4. For each ecosystem:
   - If artifact dir is real dir: move to `/mnt/linux-data/<name>/<artifact>`.
   - If missing: create empty dir on HD.
   - Create symlink from project to HD.
   - Write ecosystem-specific config (Rust cargo config).
5. `remove`: delete symlinks + HD dir + generated config.

## Storage layout

```
/mnt/linux-data/
└── <project-name>/
    ├── target/          (Rust)
    └── node_modules/    (Node)
```

## Design constraints

- Symlinks over bind mounts.
- No source code movement.
- Idempotent operations.
- Per-project isolation for build dirs.
- Global package caches by default.
EOF