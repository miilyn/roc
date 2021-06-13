fn main() {
    println!("""
        To run benchmarks:
            - Install cargo criterion: cargo install --git https://github.com/Anton-4/cargo-criterion --branch main
            - Necessary to get cache misses...: sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid'
            - run: cargo criterion
    """);
}
