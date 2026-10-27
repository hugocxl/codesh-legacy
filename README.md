# Codesh

Codesh is [hugocxl's fork](https://github.com/hugocxl/codesh) of
[Muxy](https://github.com/muxy-app/muxy), a terminal multiplexer built with Rust,
GPUI and libghostty. It tracks upstream Muxy and keeps its core: background and
remote sessions, split panes, git worktrees, extensions, AI agent integrations,
the `muxy` terminal UI, and themes.

The fork adds five customizations:

1. **Desktop identity.** Codesh app names, separate profiles, and releases from
   this repository. See [identity](docs/codesh/identity.md).
2. **Built-in extensions.** Files and Git ship enabled and trusted; extension
   management UI is hidden. See [built-in extensions](docs/codesh/builtin-extensions.md).
3. **One tab per project.** Every “new tab” becomes a split pane inside the
   project's one container. See [single-tab behavior](docs/codesh/single-tab.md).
4. **Restyled desktop.** Card panes with a 10px radius, 1px border, 12px gutters,
   and a header with split, zoom and close controls; an opaque, borderless sidebar
   with project section headers and worktree pill rows; a status bar hidden by
   default. See [style](docs/codesh/style.md).
5. **Equal-parts splits.** Every split or close re-equalizes the affected row or
   column to equal `1/n` shares. See [single-tab behavior](docs/codesh/single-tab.md).

## Install

For now, build from source. On macOS 14+ (Apple Silicon or Intel):

```bash
python3 scripts/build-install.py --build-only
```

This builds and signs without installing. It needs an installed Apple Development
or Developer ID Application certificate; use `--sign-identity` to select one if
needed. Alternatively, package an ad-hoc signed release (replace `X.Y.Z` with a
version, choose `arm64` or `x86_64`, and set the build number):

```bash
bash scripts/build-release.sh --version X.Y.Z --sign-identity - --arch arm64 --build-number 1
```

Releases will live at [hugocxl/codesh/releases](https://github.com/hugocxl/codesh/releases).
See [Contributing](CONTRIBUTING.md) for prerequisites and local build/run commands,
and the [macOS 26/27 workaround](docs/codesh/building-on-macos-27.md) for Zig and Metal setup.

## Usage and upstream documentation

The `muxy` CLI and server still run on macOS and Linux. Crate names and the
`muxy`, `muxy-server` and `muxy-app` binary names are unchanged. Muxy's mobile
companion apps remain upstream products.

The inherited [Getting started guide](docs/user-guide/getting-started.md) covers
usage, Linux formats, upgrades from 1.x and beta builds. Its Homebrew commands,
installers and download links refer to upstream Muxy distributions.

See the [local documentation](docs/README.md), [Codesh maintenance notes](docs/codesh/README.md),
and [upstream documentation](https://muxy.app/docs).

## Upstream screenshot

This is an upstream Muxy screenshot; it does not show the Codesh customizations.

<img height="1592" alt="Upstream Muxy desktop" src="https://github.com/user-attachments/assets/eca8fb92-b2f4-4a53-a25f-6f75316222af" />

## Credit and license

Codesh is derived from [Muxy](https://github.com/muxy-app/muxy).
Copyright (c) 2026 Muxy. The upstream copyright and permission notice are
preserved in [LICENSE](LICENSE). Codesh is distributed under the same MIT license.
