# defendertest

Hopefully this will give some inside on timings for file read-write on a clean system vs defender.

## Usage

1. Install rustup, which gives you access to the `cargo` build tool:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Install flamegraph using cargo:

```bash
cargo install flamegraph
```

3. Build the tool using cargo:

```bash
cargo build --release
```

4. Run the tool under flamegraph:

```bash
sudo flamegraph -- target/release/defendertest --path /tmp/ --total-inodes 1000
```

5. Let's compare our timing information and flamegraph.
