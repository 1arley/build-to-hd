# Usage — build-to-hd

## Install helper

The helper lives at `~/.local/bin/build-to-hd`. Ensure `~/.local/bin` is in `PATH`.

## Move artifacts

```bash
# Auto-detect ecosystem
build-to-hd ~/Projects/my-project auto

# Force Rust
build-to-hd ~/Projects/my-crate rust

# Force Node
build-to-hd ~/Projects/my-app node
```

What happens:
- Rust: `target/` moved to `/mnt/linux-data/<name>/target`; symlink left in project; `.cargo/config.toml` written.
- Node: `node_modules/` moved to `/mnt/linux-data/<name>/node_modules`; symlink left in project.

## Remove a project from HD

```bash
build-to-hd remove ~/Projects/my-project
```

Removes symlinks and the per-project HD directory. Source code is untouched.

## Verify

```bash
ls -l ~/Projects/my-project/target
readlink -f ~/Projects/my-project/target
du -sh /mnt/linux-data/<name>
```

## Notes

- Package caches remain global (`~/.npm`, pnpm store, cargo registry).
- If HD is unmounted, helper aborts.
- Re-running is safe (idempotent).
EOF