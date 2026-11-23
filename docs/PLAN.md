# Plan: evolve build-to-hd into a Rust CLI

## Vision

A small, fast CLI that manages heavy build artifacts across projects and disks,
with per-project isolation, registry, status and cleanup. Bash helper remains
the reference behavior until the Rust version reaches parity.

## Non-goals (for now)

- Replacing package managers or build tools.
- Moving source code.
- Cloud sync.

## Phases

### Phase 0 — Bash helper (done)

- [x] Move `target/` and `node_modules/` to HD
- [x] Leave symlinks in place
- [x] Per-project directories on HD
- [x] Idempotent runs
- [x] `auto` detection via manifests
- [x] `remove` command: delete HD dir + symlinks + generated config

### Phase 1 — Rust skeleton

- [ ] Add CLI parsing (clap)
- [ ] Commands: `add`, `list`, `status`, `clean`, `run`
- [ ] Config file: `~/.config/build-to-hd/config.toml`
- [ ] Registry: `~/.config/build-to-hd/registry.toml` (project -> HD path)
- [ ] Cross-platform paths (Linux first)

### Phase 2 — Core operations

- [ ] `add <project>`: detect ecosystems, move artifacts, write per-project config
- [ ] Support Rust, Node (npm/pnpm/yarn), Python (venv/.pytest_cache), Go
- [ ] Dry-run mode (`--dry-run`)
- [ ] JSON output for scripting (`--json`)

### Phase 3 — Observability

- [ ] `status`: per-project and total disk usage on HD vs SSD
- [ ] `doctor`: detect broken symlinks, missing HD, stale entries
- [ ] `clean --older <days>`: remove artifacts not used recently

### Phase 4 — Automation

- [ ] Git hook / watcher to auto-move on `git clone`? (optional)
- [ ] Shell completions (bash/zsh/fish)
- [ ] Packaging (cargo install, AUR, GitHub releases)

## Design notes

- Prefer symlinks over bind mounts (simpler, reversible).
- Keep package caches global by default; isolate only project build dirs.
- Never move source directories.
- All paths stored as absolute; validate HD mount before operations.
- Keep operations idempotent and safe (no data loss).

## Open questions

- Should `node_modules` move for pnpm too, or rely on global store?
- How to handle monorepos (per-package targets)?
- Should cleanup consider atime or explicit markers?
EOF