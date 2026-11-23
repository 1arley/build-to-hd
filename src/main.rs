// build-to-hd: move heavy build artifacts to a secondary disk, leave symlinks.
//
// This is a study skeleton. The real implementation will be built incrementally
// following docs/PLAN.md.

fn main() {
    println!("build-to-hd 0.1.0 (skeleton)");
    println!();
    println!("Planned commands:");
    println!("  build-to-hd add <project>      # register + move artifacts");
    println!("  build-to-hd run <project>      # run with HD env vars");
    println!("  build-to-hd list               # list registered projects");
    println!("  build-to-hd status             # show disk usage");
    println!("  build-to-hd clean [--older N]  # remove stale artifacts");
    println!();
    println!("See docs/PLAN.md for the roadmap.");
}
EOF