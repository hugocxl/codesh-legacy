# Single tab per project

Codesh enables `muxy_app_core::codesh::SINGLE_TAB`. Each project, including a
worktree with `parent_id`, has zero or one tab. That tab contains the pane layout.
New Terminal (the existing shortcut is unchanged), adjacent-terminal requests,
extension tab types, browser opens, and Remote Servers open panes in that
container. Singleton webviews and Remote Servers reuse their existing pane.

New content splits to the right of the focused pane, falling back to the last
layout leaf. Background browser opens preserve window selection and focus.
Typed pane insertion shares `AppState::insert_split_pane` with terminal and
browser splits: apply split equalization hooks immediately after its
`tab.layout.split` call in `state.rs`.

Load, native restore, legacy import, and project-layout application merge old
tabs in order. The first tab survives; each later layout subtree is appended
on the right intact. Pane IDs, content, terminal sessions, and startup commands
survive (native backup restore still remaps IDs as before). Merging clears tab
groups and zoom, repairs selected tab IDs, and retains valid pane focus. Loading
an existing single container preserves its zoom. Only the
surviving tab's title, pin, and color remain. Normalization runs on lifecycle
boundaries, never during rendering.

Closing the last pane leaves an empty project. The next New Terminal creates a
fresh container; closing never silently starts another terminal session.
Custom commands and agent resumes title their new pane. Extension terminal
commands target the newly focused pane. Extension APIs still return the
container tab ID; callers identifying individual documents should retain pane
IDs where the existing API supports them.

Tab-group drops are disabled. Group strips and the project titlebar `+` are
suppressed; menus and keyboard shortcuts remain available. The separate visual
change owns workspace header/tab-strip removal and sidebar styling.

No protocol/server changes or dependencies are needed. On a build machine run:

```sh
cargo fmt --all --check
cargo check -p muxy-app-core
cargo test -p muxy-app-core
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps
```
