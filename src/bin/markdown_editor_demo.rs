//! D116/Phase 28 capstone demo — a real, usable markdown editor built
//! ONLY from public `TextArea` APIs: a toy `**bold**`/`# heading`/
//! `` `code` `` `SpanSource` (Step 5) does live syntax highlighting, a
//! toolbar's Bold/Italic buttons wrap the live keyboard selection via
//! `EditController` (Step 2), and a second `TextArea` mirrors the same
//! atom with the same highlighting as a live "preview" pane.
//!
//! The point this proves: `rosace` never learned what markdown is. The
//! app supplied the tokenizer (`markdown_spans` below) and the toolbar;
//! the framework only supplied the editing/selection/styling seams
//! (`Transaction`/`Selection`/`EditController`/`SpanSource`) every other
//! text-shaped widget already uses.

use rosace::prelude::*;
use std::sync::Arc;

/// A deliberately toy markdown tokenizer — real enough to prove the
/// `SpanSource` seam works, not a spec-complete markdown parser (that's
/// explicitly the app's job, never the framework's, per D116).
fn markdown_spans(s: &str, _changed: Option<(usize, usize)>) -> Vec<Span> {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    let mut spans = Vec::new();

    // Headings: any line starting with "# ".
    let mut line_start = 0usize;
    for i in 0..=n {
        if i == n || chars[i] == '\n' {
            if i > line_start + 1 && chars[line_start] == '#' && chars[line_start + 1] == ' ' {
                spans.push(Span::new((line_start, i))
                    .color(Color::rgb(140, 180, 255))
                    .weight(FontWeight::Bold));
            }
            line_start = i + 1;
        }
    }

    // Bold: **...**
    let mut i = 0;
    while i + 1 < n {
        if chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(rel) = chars[i + 2..].windows(2).position(|w| w == ['*', '*']) {
                let end = i + 2 + rel + 2;
                spans.push(Span::new((i, end)).weight(FontWeight::Bold));
                i = end;
                continue;
            }
        }
        i += 1;
    }

    // Inline code: `...`
    let mut i = 0;
    while i < n {
        if chars[i] == '`' {
            if let Some(rel) = chars[i + 1..].iter().position(|&c| c == '`') {
                let end = i + 1 + rel + 1;
                spans.push(Span::new((i, end)).color(Color::rgb(255, 180, 120)));
                i = end;
                continue;
            }
        }
        i += 1;
    }

    spans
}

/// Phase 32 Step 3's motivating case: the preview pane above was honestly
/// just a second SOURCE-styled `TextArea` — markers (`**`, `#`, `` ` ``)
/// still visible, only colored. This renders REAL formatted markdown
/// (markers hidden, real bold/heading) via `Text::rich` + `RichText` —
/// wiring Phase 32 Step 3's actual deliverable, not a rewrite of the toy
/// tokenizer above (which stays exactly as-is for the editable source pane).
///
/// `RichText`'s wrap algorithm treats `\n` as plain whitespace (no forced
/// break — see `rosace-text`'s `TextLayout::layout_with_measure`, which
/// only knows about word-wrapping, not markdown's block structure), so a
/// naive single `RichText` would flow every source line into one paragraph.
/// One `RichText` PER SOURCE LINE (stacked in a `Column`) gets real hard
/// line breaks for free while still getting real inline mixed styles
/// WITHIN each line — a block-splitting responsibility that belongs in
/// the app's tokenizer, same as everything else markdown-shaped here.
fn markdown_preview_color(r: u8, g: u8, b: u8) -> rosace::theme::Color {
    rosace::theme::Color { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a: 1.0 }
}

fn markdown_preview_lines(source: &str) -> Vec<RichText> {
    source.split('\n').map(|line| {
        if let Some(heading) = line.strip_prefix("# ") {
            return RichText::new().bold(heading.to_string(), 22.0, markdown_preview_color(140, 180, 255));
        }
        parse_inline_markdown(line)
    }).collect()
}

/// Inline `**bold**` / `` `code` `` within one line — real spans, markers
/// stripped, everything else plain. Same toy scope as `markdown_spans`
/// above (not a spec-complete parser); mirrors its exact bold/code
/// detection logic since both views must agree on what's "bold"/"code".
fn parse_inline_markdown(line: &str) -> RichText {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();
    let plain = markdown_preview_color(230, 230, 235);
    let code_color = markdown_preview_color(255, 180, 120);

    let mut rt = RichText::new();
    let mut i = 0;
    let mut plain_start = 0;
    let flush_plain = |rt: RichText, text: &str| -> RichText {
        if text.is_empty() { rt } else { rt.text(text.to_string(), 16.0, plain) }
    };

    while i < n {
        if i + 1 < n && chars[i] == '*' && chars[i + 1] == '*' {
            if let Some(rel) = chars[i + 2..].windows(2).position(|w| w == ['*', '*']) {
                let end = i + 2 + rel;
                rt = flush_plain(rt, &chars[plain_start..i].iter().collect::<String>());
                let inner: String = chars[i + 2..end].iter().collect();
                rt = rt.bold(inner, 16.0, plain);
                i = end + 2;
                plain_start = i;
                continue;
            }
        }
        if chars[i] == '`' {
            if let Some(rel) = chars[i + 1..].iter().position(|&c| c == '`') {
                let end = i + 1 + rel;
                rt = flush_plain(rt, &chars[plain_start..i].iter().collect::<String>());
                let inner: String = chars[i + 1..end].iter().collect();
                rt = rt.text(inner, 15.0, code_color);
                i = end + 1;
                plain_start = i;
                continue;
            }
        }
        i += 1;
    }
    rt = flush_plain(rt, &chars[plain_start..n].iter().collect::<String>());
    if rt.is_empty() { rt.text(" ", 16.0, plain) } else { rt }
}

struct MarkdownEditorDemo;

impl Component for MarkdownEditorDemo {
    fn build(&self, ctx: &mut Context) -> BoxedWidget {
        let body: Atom<String> = ctx.state(String::from(
            "# Markdown demo\n\nType **bold** text, `inline code`, or a heading.\n\nThe toolbar wraps your selection.\n\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12\nLine 13\nLine 14\nLine 15\nLine 16\nLine 17\nLine 18\nLine 19\nLine 20"
        ));
        let controller: EditController = ctx.state(EditController::new()).get();

        let toolbar = Row::new()
            .child(Button::new("Bold").on_press({
                let controller = controller.clone();
                move || {
                    let value = controller.value();
                    let (start, end) = controller.selection().primary_range();
                    if start < end {
                        let word = value[start..end].to_string();
                        controller.replace_range(start, end, format!("**{word}**"));
                    }
                }
            }))
            .child(Spacer::gap(8.0, 0.0))
            .child(Button::new("Italic").on_press({
                let controller = controller.clone();
                move || {
                    let value = controller.value();
                    let (start, end) = controller.selection().primary_range();
                    if start < end {
                        let word = value[start..end].to_string();
                        controller.replace_range(start, end, format!("*{word}*"));
                    }
                }
            }));

        Scaffold::new(
            Column::new()
                .child(Spacer::gap(0.0, 16.0))
                .child(toolbar)
                .child(Spacer::gap(0.0, 12.0))
                .child(
                    Row::new()
                        .child(
                            TextArea::new()
                                .placeholder("Write markdown...")
                                .value(body.get())
                                .width(280.0)
                                .height(260.0)
                                .controller(controller.clone())
                                .spans(markdown_spans)
                                .on_change({
                                    let body = body.clone();
                                    move |v| body.set(v)
                                }),
                        )
                        .child(Spacer::gap(16.0, 0.0))
                        // Live preview pane — REAL rendered markdown (Phase 32
                        // Step 3): markers hidden, real bold/heading, not a
                        // second source-styled editor. Same `body` atom, so
                        // it updates as you type in the source pane.
                        .child(
                            Container::new()
                                .width(280.0)
                                .height(260.0)
                                .child(ScrollView::new(
                                    Column::new()
                                        .spacing(2.0)
                                        .children(
                                            markdown_preview_lines(&body.get())
                                                .into_iter()
                                                .map(|rt| Arc::new(Text::rich(rt)) as BoxedWidget)
                                                .collect(),
                                        ),
                                )),
                        ),
                ),
        )
        .app_bar(AppBar::new("markdown_editor_demo"))
        .boxed()
    }
}

fn main() {
    env_logger::init();
    App::new().title("markdown_editor_demo").size(620, 420).launch(MarkdownEditorDemo);
}
