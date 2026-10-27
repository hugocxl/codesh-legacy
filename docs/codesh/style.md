# Codesh desktop style

The window and opaque sidebar share `Theme::bg`. The bundled Muxy dark theme
uses `#111111` and warm `#d5cec7` text. Fresh profiles use ProjectFocused layout,
an expanded 270px sidebar, hidden tips/status, and no sidebar vibrancy. Saved
appearance choices still apply, except the sidebar always stays opaque.

## Tokens and geometry

`views/codesh/style.rs` owns 12px grid inset/gutter, 10px card radius, 1px border,
36px header, 12px body padding and 16px sidebar padding. Headers use 13px semibold
text, 16px glyphs and 8px control gaps. Cards blend 2.5% theme foreground over
background (about `#161616`); borders blend 95% theme border over background
(about `#282827`). Pills use theme surface/hover. No shadows or permanent split
or sidebar separators are painted.

For extent `E`, ratio `r` and display scale `s`, the pure gutter helper computes:

```text
g = min(max(E, 0), 12)
A = max(E, 0) - g
first = clamp(round(A * r * s) / s, 0, A)
second_origin = first + g
second_size = max(E, 0) - second_origin
```

Only the shared edge is snapped, including for extents smaller than the gutter.
Resize divides pointer delta by `max(E - 12, 1)`. The 10px grip is centred in the
gutter, which shows a subtle 1px accent line on hover/drag. Both pane-drop
walkers reuse `split_bounds()`; nested ratios remain owned by the layout model.
The whole grid receives one 12px inset, inside auxiliary panel composition.

## Hooks

- `splits::pane_element()` wraps every pane kind exactly once in `pane_card`;
  zoom adds no extra frame. Header buttons focus their own PaneId before acting;
  close uses `AppModel::close_pane()`. Headers focus but never initiate drags.
- The body shrinks below the header with real layout padding. Workspace terminal
  padding adds only `max(configured - 2, 0)` per edge: the upstream default 2px
  is absorbed into the 12px card inset. Quick terminal padding is unchanged.
  Palette construction and the opaque workspace terminal root share the card
  blend; explicit terminal background overrides remain user overrides. Inner
  terminal focus borders and zoom-only rounding are removed.
- Native browser/extension canvases measure their frames inside the body below
  the header and sync the shared 10px radius through existing host code.
- `workspace::render()` hides both main header variants and their separator;
  group strips are also hidden. Legacy renderers remain for the separate tab
  policy work. Sidebar chrome reserves native titlebar space and retains window
  dragging; a collapsed sidebar reserves top space for the toggle.
- Sidebar project headers retain selection, disclosure, menus and drag ordering.
  Worktrees/leaf projects use 28px pills, activity glyphs and PRIMARY badges.
  The top row shares the native traffic-light row: sidebar toggle on the left,
  then the Files and Source Control extensions' own topbar items on the right.
  Missing/hidden extension items are omitted; commands and popover anchors use
  the shared extension toolbar renderer.
- The footer has 16px side padding and 8px vertical padding: host selector,
  28px Filter pill, Add Project, New Browser. Collapsed sidebars stack the icon
  controls and hide the text input. Host choice persists in Appearance (All,
  Local, Server); the name query lives only in AppModel and Escape clears it.
  Case-insensitive name filtering keeps parent headers with matching worktrees
  and reveals matching rows without changing saved disclosure choices. The
  host menu also opens the existing remote management popover.
- Agents are hidden by default. The existing shortcut opens the left placement
  as a popup; right and detached placements retain their existing behavior.
  Saved visibility preferences are retained, without docking into the sidebar.

Terminal `PaneView::element()` and sidebar caching remain intact. Titles/icons
use cached state; there are no native title/process queries, animations, new
dependencies or layout feedback loops. The terminal card-padding flag changes
once when a terminal joins the workspace.

## Validation

The sidebar iteration is validated with core tests, GPUI sidebar/filter tests,
Clippy with warnings denied, and documentation with rustdoc warnings denied.
Workspace integration tests also exercise sockets, subprocesses and SSH; those
need a session that permits those operations. The debug app is built for manual
visual review. The local validation commands are:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
RUSTDOCFLAGS=-Dwarnings cargo doc --workspace --no-deps
cargo run -p muxy-app
```

Compare one pane, nested splits and zoom to the references. Check inactive-card
controls, confirmations, find, selection, IME, native scrolling/webviews, resizing
and pane drops, collapsed/narrow sidebars, alternate themes and Linux controls.
Measure idle CPU/memory and resize responsiveness with several active panes.
