# Contributing to Codesh

Issues and pull requests are welcome at [hugocxl/codesh](https://github.com/hugocxl/codesh).
Keep changes focused. Read the [fork rules](AGENTS.md) and
[Codesh notes](docs/codesh/README.md), then the inherited
[product](docs/product/README.md) and [technical](docs/tech/README.md) guides.
Anything submitted upstream must follow Muxy's humans-only policy in AGENTS.md.

## Prerequisites

- macOS 14+ for the desktop app, or Linux for the CLI and server.
- [Rust](https://rustup.rs), using the version pinned in [rust-toolchain.toml](rust-toolchain.toml).
- [Zig](https://ziglang.org) 0.15.2 to build the Ghostty terminal library.

On macOS 26/27, Zig 0.15.2 cannot link natively against the affected SDK.
Use the [SDK workaround and Metal toolchain setup](docs/codesh/building-on-macos-27.md).

## Setup

```bash
git clone https://github.com/hugocxl/codesh.git
cd codesh
git checkout codesh
git switch -c task/my-change
```

On macOS, build with `cargo build --workspace`. On Linux, build the CLI and
server with `cargo build -p muxy-cli -p muxy-server`.
For packaging, see the [README](README.md#install).

## Running

```bash
cargo run -p muxy-app     # desktop app
cargo run -p muxy-cli     # muxy terminal UI
```

Build as in [Setup](#setup) first so `muxy-server` sits next to the binary you
run. Development builds use a separate `Codesh Dev` profile (`codesh-dev` on
Linux); see [identity](docs/codesh/identity.md).

## Checks

Run before submitting changes:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```

On Linux, `scripts/check-headless.py` checks the CLI, server and dependencies
and requires glibc 2.35. Elsewhere, select the packages listed by
`python3 scripts/check-headless.py --packages` for Clippy and tests.
`scripts/check-desktop-linux.py` checks the whole workspace, including the
[experimental Linux desktop](docs/tech/constraints.md), on Ubuntu 24.04.

## Code standards

Keep code self-explanatory without comments, prefer early returns, fix root
causes, follow existing patterns, and prevent security vulnerabilities. Test
real risks and watch GPUI's CPU and memory use. New dependencies need owner
approval; see [AGENTS.md](AGENTS.md).

## License

Contributions are licensed under the [MIT License](LICENSE).
