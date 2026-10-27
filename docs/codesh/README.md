# Codesh notes

Codesh tracks [Muxy](https://github.com/muxy-app/muxy) while keeping its core,
protocol and crate/binary names. These pages describe the fork's changes and
maintenance. See [AGENTS.md](../../AGENTS.md) for engineering and AI policies.

## Desktop identity

[identity.md](identity.md) covers Codesh app and bundle names, separate profiles,
release feeds, signing and notarization. Internal `muxy` names and protocol
compatibility remain intact to keep upstream merges manageable.

## Built-in extensions

[builtin-extensions.md](builtin-extensions.md) explains how Files and Git are
bundled, installed, always enabled and pre-granted permissions. It also covers
hidden extension management UI and refreshing the bundles from upstream.

## Single tab and equal-parts splits

[single-tab.md](single-tab.md) describes one tab per project: every “new tab”
creates a split pane inside that project's one container. Each split or close
re-equalizes the affected row or column to `1/n` shares.

## Desktop style

[style.md](style.md) describes card panes with a 10px radius, 1px border, 12px
gutters and per-card split/zoom/close headers. The opaque sidebar has no border,
uses project section headers and worktree pill rows, and the status bar is
hidden by default.

## Building on macOS 26/27

[building-on-macos-27.md](building-on-macos-27.md) documents the Zig 0.15.2 SDK
linker workaround and GPUI's Metal toolchain requirement. The fake SDK is scoped
to Zig so Cargo, Clang and bindgen keep using the real SDK.

## Upstream sync

`main` is an upstream mirror; never edit it directly. Fork work lands on `codesh`.
Keep changes in `crates/*/src/codesh*` and `docs/codesh/` where possible, using
few-line hooks into upstream code. Never change `muxy-protocol` or rename the
`muxy` crates/binaries.

With a clean checkout, enable rerere and replace `<upstream tag>` with the
chosen upstream release tag:

```bash
git config rerere.enabled true
git fetch upstream && git checkout main && git merge --ff-only upstream/main && git push origin main && git checkout codesh && git rebase <upstream tag>
```

Resolve conflicts, run all checks in [AGENTS.md](../../AGENTS.md), rebuild, and
re-screenshot the desktop to verify the fork's behavior and style. AI agents may
commit and push for the owner when asked in this fork; upstream submissions
remain subject to Muxy's humans-only policy.
