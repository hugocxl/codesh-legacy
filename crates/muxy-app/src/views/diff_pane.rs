// The coordinator wires this entity into the native pane renderer.
#![allow(dead_code)]

use std::ops::Range;

use gpui::prelude::*;
use gpui::{
    App, Context, Div, FocusHandle, Focusable, Hsla, ListHorizontalSizingBehavior, MouseButton,
    SharedString, UniformListScrollHandle, Window, div, px, uniform_list,
};
use muxy_protocol::{GitDiff, GitDiffKind, GitDiffRow};
use muxy_ui::controls::{self, Choice};
use muxy_ui::theme::{Metrics, Theme};

use super::codesh::style;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct SplitRow {
    left: Option<usize>,
    right: Option<usize>,
}

fn split_rows(rows: &[GitDiffRow]) -> Vec<SplitRow> {
    let mut layout = Vec::with_capacity(rows.len());
    let mut index = 0;
    while index < rows.len() {
        if matches!(
            rows[index].kind,
            GitDiffKind::Addition | GitDiffKind::Deletion
        ) {
            let start = index;
            while index < rows.len()
                && matches!(
                    rows[index].kind,
                    GitDiffKind::Addition | GitDiffKind::Deletion
                )
            {
                index += 1;
            }
            let mut deleted = (start..index).filter(|&i| rows[i].kind == GitDiffKind::Deletion);
            let mut added = (start..index).filter(|&i| rows[i].kind == GitDiffKind::Addition);
            loop {
                let left = deleted.next();
                let right = added.next();
                if left.is_none() && right.is_none() {
                    break;
                }
                layout.push(SplitRow { left, right });
            }
        } else {
            layout.push(SplitRow {
                left: Some(index),
                right: Some(index),
            });
            index += 1;
        }
    }
    layout
}

// GPUI computes the pixel viewport; bound its requested rows after a diff refresh.
fn visible_range(requested: Range<usize>, count: usize) -> Range<usize> {
    let start = requested.start.min(count);
    start..requested.end.clamp(start, count)
}

fn content(row: &GitDiffRow, old: bool) -> &str {
    let text = if old { &row.old_text } else { &row.new_text };
    text.as_deref().unwrap_or_else(|| {
        if row.kind == GitDiffKind::Hunk {
            &row.text
        } else {
            row.text.get(1..).unwrap_or(&row.text)
        }
    })
}

fn columns(text: &str) -> usize {
    text.chars().fold(0_usize, |column, ch| {
        column.saturating_add(if ch == '\t' {
            4
        } else if ch.is_ascii() {
            1
        } else {
            2
        })
    })
}

#[derive(Debug)]
pub(crate) struct DiffView {
    diff: GitDiff,
    path: SharedString,
    theme: Theme,
    metrics: Metrics,
    focus: FocusHandle,
    scroll: UniformListScrollHandle,
    split: bool,
    staged: bool,
    layout: Vec<SplitRow>,
    max_columns: usize,
}

impl DiffView {
    pub(crate) fn new(
        diff: GitDiff,
        path: &str,
        theme: &Theme,
        metrics: Metrics,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut view = Self {
            diff,
            path: path.to_owned().into(),
            theme: theme.clone(),
            metrics,
            focus: cx.focus_handle(),
            scroll: UniformListScrollHandle::new(),
            split: false,
            staged: false,
            layout: Vec::new(),
            max_columns: 0,
        };
        view.rebuild();
        view
    }

    pub(crate) fn set_diff(&mut self, diff: GitDiff, cx: &mut Context<Self>) {
        self.diff = diff;
        self.rebuild();
        cx.notify();
    }

    pub(crate) fn set_theme(&mut self, theme: &Theme, cx: &mut Context<Self>) {
        self.theme = theme.clone();
        cx.notify();
    }

    pub(crate) fn set_staged(&mut self, staged: bool, cx: &mut Context<Self>) {
        self.staged = staged;
        cx.notify();
    }

    fn rebuild(&mut self) {
        self.layout = split_rows(&self.diff.rows);
        self.max_columns = self
            .diff
            .rows
            .iter()
            .map(|row| columns(content(row, true)).max(columns(content(row, false))))
            .max()
            .unwrap_or(0);
    }

    fn row_count(&self) -> usize {
        if self.split {
            self.layout.len()
        } else {
            self.diff.rows.len()
        }
    }

    fn header(&self, cx: &Context<Self>) -> Div {
        div()
            .flex()
            .flex_none()
            .items_center()
            .h(px(style::HEADER))
            .gap(style::spacing())
            .px(style::spacing())
            .text_size(self.metrics.font_emphasis())
            .font_weight(gpui::FontWeight::SEMIBOLD)
            .child(div().flex_1().min_w_0().truncate().child(self.path.clone()))
            .child(
                div()
                    .text_color(self.theme.diff_add)
                    .child(format!("+{}", self.diff.additions)),
            )
            .child(
                div()
                    .text_color(self.theme.diff_remove)
                    .child(format!("−{}", self.diff.deletions)),
            )
            .child(div().text_color(self.theme.fg_muted).child(if self.staged {
                "Staged"
            } else {
                "Unstaged"
            }))
            .child(controls::segmented(
                controls::Style {
                    theme: &self.theme,
                    metrics: &self.metrics,
                },
                "diff-mode",
                &[
                    Choice::new("unified", "Unified"),
                    Choice::new("split", "Split"),
                ],
                if self.split { "split" } else { "unified" },
                cx.listener(|view, value: &SharedString, _, cx| {
                    let split = value.as_ref() == "split";
                    if view.split != split {
                        view.split = split;
                        view.scroll = UniformListScrollHandle::new();
                        cx.notify();
                    }
                }),
            ))
    }

    fn tint(&self, kind: GitDiffKind) -> Hsla {
        let mut color = match kind {
            GitDiffKind::Addition => self.theme.diff_add,
            GitDiffKind::Deletion => self.theme.diff_remove,
            GitDiffKind::Hunk => self.theme.diff_hunk,
            _ => return gpui::transparent_black(),
        };
        color.a *= 0.12;
        color
    }

    fn number(&self, number: Option<u64>) -> Div {
        div()
            .flex_none()
            .w(self.metrics.scaled(64.0))
            .pr(self.metrics.spacing2())
            .text_right()
            .text_color(self.theme.fg_dim)
            .child(number.map_or_else(String::new, |value| value.to_string()))
    }

    fn cell(&self, row: Option<&GitDiffRow>, old: bool, unified: bool) -> Div {
        let mut cell = div()
            .flex()
            .items_center()
            .h_full()
            .min_w_0()
            .flex_1()
            .overflow_hidden();
        let Some(row) = row else { return cell };
        cell = cell.bg(self.tint(row.kind));
        if unified || old {
            cell = cell.child(self.number(row.old_line_number));
        }
        if unified || !old {
            cell = cell.child(self.number(row.new_line_number));
        }
        let marker = match row.kind {
            GitDiffKind::Addition => "+",
            GitDiffKind::Deletion => "-",
            _ => " ",
        };
        let color = if row.kind == GitDiffKind::Hunk {
            self.theme.diff_hunk
        } else {
            self.theme.fg
        };
        cell.child(
            div()
                .flex_none()
                .w(self.metrics.scaled(20.0))
                .text_color(color)
                .child(marker),
        )
        .child(
            div()
                .flex_none()
                .whitespace_nowrap()
                .text_color(color)
                .child(content(row, old).replace('\t', "    ")),
        )
    }

    fn row(&self, index: usize, width: gpui::Pixels) -> Div {
        let row = div()
            .flex()
            .w(width)
            .min_w_full()
            .h(self.metrics.scaled(20.0))
            .line_height(self.metrics.scaled(20.0));
        if self.split {
            let pair = self.layout[index];
            row.child(self.cell(pair.left.map(|i| &self.diff.rows[i]), true, false))
                .child(self.cell(pair.right.map(|i| &self.diff.rows[i]), false, false))
        } else {
            let source = &self.diff.rows[index];
            row.child(self.cell(Some(source), source.kind == GitDiffKind::Deletion, true))
        }
    }
}

impl Focusable for DiffView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl Render for DiffView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.row_count();
        // Width is cached from text columns, never by shaping the entire diff.
        #[allow(clippy::cast_precision_loss)]
        let columns = self.max_columns as f32;
        let width = self.metrics.scaled(if self.split {
            2.0 * (columns * 8.0 + 84.0)
        } else {
            columns * 8.0 + 148.0
        });
        let mut body = div()
            .id("native-diff")
            .track_focus(&self.focus)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|view, _, window, _| view.focus.focus(window)),
            )
            .flex()
            .flex_col()
            .size_full()
            .min_w_0()
            .min_h_0()
            .overflow_hidden()
            .bg(self.theme.bg)
            .text_color(self.theme.fg)
            .child(self.header(cx));
        if self.diff.binary {
            body = body.child(
                div()
                    .p(style::spacing())
                    .text_color(self.theme.fg_muted)
                    .child("Binary file — no text diff available."),
            );
        } else if count == 0 {
            body = body.child(
                div()
                    .p(style::spacing())
                    .text_color(self.theme.fg_muted)
                    .child("No changes to display."),
            );
        } else {
            body = body.child(
                uniform_list(
                    "diff-rows",
                    count,
                    cx.processor(move |view, range, _, _| {
                        visible_range(range, view.row_count())
                            .map(|i| view.row(i, width))
                            .collect()
                    }),
                )
                .track_scroll(self.scroll.clone())
                .with_horizontal_sizing_behavior(ListHorizontalSizingBehavior::Unconstrained)
                .flex_1()
                .min_h_0()
                .font_family(muxy_ui::font::monospace())
                .text_size(self.metrics.font_body())
                .whitespace_nowrap(),
            );
        }
        if self.diff.truncated {
            body = body.child(
                div()
                    .flex_none()
                    .p(style::spacing())
                    .text_color(self.theme.fg_muted)
                    .child("Diff truncated — additional changes are not shown."),
            );
        }
        body
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rows(kinds: &[GitDiffKind]) -> Vec<GitDiffRow> {
        kinds
            .iter()
            .map(|&kind| GitDiffRow {
                kind,
                old_line_number: None,
                new_line_number: None,
                old_text: None,
                new_text: None,
                text: String::new(),
            })
            .collect()
    }

    #[test]
    fn split_pairs_replacements_and_keeps_unmatched_lines() {
        use GitDiffKind::{Addition as A, Context as C, Deletion as D, Hunk as H};
        let layout = split_rows(&rows(&[H, D, D, A, C, A, A, D, H, D, A, A]));
        let pairs: Vec<_> = layout.iter().map(|row| (row.left, row.right)).collect();
        assert_eq!(
            pairs,
            vec![
                (Some(0), Some(0)),
                (Some(1), Some(3)),
                (Some(2), None),
                (Some(4), Some(4)),
                (Some(7), Some(5)),
                (None, Some(6)),
                (Some(8), Some(8)),
                (Some(9), Some(10)),
                (None, Some(11)),
            ]
        );
    }

    #[test]
    fn split_never_pairs_across_hunks_or_context() {
        use GitDiffKind::{Addition as A, Context as C, Deletion as D, Hunk as H};
        let layout = split_rows(&rows(&[D, H, A, C, D, C, A]));
        assert_eq!(layout.len(), 7);
        assert_eq!(
            layout[0],
            SplitRow {
                left: Some(0),
                right: None
            }
        );
        assert_eq!(
            layout[2],
            SplitRow {
                left: None,
                right: Some(2)
            }
        );
        assert_eq!(
            layout[4],
            SplitRow {
                left: Some(4),
                right: None
            }
        );
        assert_eq!(
            layout[6],
            SplitRow {
                left: None,
                right: Some(6)
            }
        );
    }

    #[test]
    fn visible_range_stays_bounded_for_large_and_replaced_diffs() {
        assert_eq!(visible_range(25_000..25_040, 50_000), 25_000..25_040);
        assert_eq!(visible_range(49_980..50_020, 50_000), 49_980..50_000);
        assert_eq!(visible_range(49_980..50_020, 12), 12..12);
        assert_eq!(visible_range(0..40, 0), 0..0);
    }
}
