# Built-in extensions

> **Retired.** Native Files and Source Control panels, editor and diff panes
> replace these extensions. `BUILTINS` is empty, so nothing is seeded or forced
> on, and folders an earlier version seeded (those carrying `.codesh-builtin`)
> are skipped when extensions load. The bundle and the extension system remain
> until they are removed; the rest of this page describes the earlier setup.

Codesh ships the upstream [`files`](https://github.com/muxy-app/extensions/tree/main/extensions/files)
and [`git`](https://github.com/muxy-app/extensions/tree/main/extensions/git) extensions as
built-in features. They run on the normal extension system; only how they are installed,
enabled and trusted differs.

## How it works

- **Source.** The build output of each extension (the folder holding its dist `package.json`)
  is committed under `crates/muxy-app/builtin-extensions/<name>/`. `UPSTREAM.toml` records the
  upstream commit and versions. Marketplace screenshots are left out.
- **Bundling.** `crates/muxy-app/build.rs` embeds every file below `builtin-extensions/*/`
  (except dot files and `logs/`) into the binary as `BUILTIN_EXTENSIONS`, plus
  `BUILTIN_EXTENSIONS_HASH`, a hash of all paths and bytes. Cargo does not run node or npm.
- **Install.** At boot, right after pending backup restores and 1.x imports are applied,
  `muxy_app_core::extensions::builtin::seed` writes each extension to
  `<profile>/extensions/.codesh-stage-<name>`, copies across the previous
  `logs/output.log`, then swaps it into `<profile>/extensions/<name>`. The folder is stamped
  with the bundle hash in `.codesh-builtin`; a matching stamp skips the copy, so this is a
  no-op on most launches and a full replace whenever the bundle changes. Any leftover stage
  from an interrupted attempt is discarded. A failure is logged and boot continues. Extension
  storage and settings live outside the folder and survive replacement.
- **Always enabled.** `Registry::enabled`/`active` treat `files` and `git` as enabled whatever
  `extension-enabled.json` says (so a restored backup cannot turn them off), and
  `set_enabled(false)`/`remove` refuse them. A "Load Unpacked" folder reusing one of these
  names gets none of this.
- **Pre-granted.** `AppModel::consent` allows every gated call from a built-in without a
  prompt. The audit log still records each one, with rule `builtin`.
- **Hidden UI.** The Extensions settings category (and so the marketplace and Load Unpacked),
  the "Manage Extensions" palette command, the "More languages" row and "Update all" for
  built-ins are hidden or removed. Terminal file links default to the `files:editor` opener.

## Refreshing from upstream

```sh
scripts/update-builtin-extensions.sh            # latest main
scripts/update-builtin-extensions.sh <tag|sha>  # a specific ref
```

The script needs git, node (20+), npm and rsync. It clones the upstream repository, builds
both extensions with the upstream `scripts/build.mjs` (`npm ci --ignore-scripts`, then each
extension's `build`), syncs each `dist/` into place and rewrites `UPSTREAM.toml`. Review the
diff (especially permission changes in the two `package.json` files), then rebuild the app:
the new bundle hash makes the next launch replace the installed copies.
