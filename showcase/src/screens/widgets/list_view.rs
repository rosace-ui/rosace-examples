//! `ListView` — a VIRTUALIZED list: only the rows inside the viewport are
//! ever built, so 10,000 rows cost the same as 20.
//!
//! That is the whole point of the widget, and it is also its one constraint:
//! every row must be `item_extent` tall, because the scroll position is
//! computed arithmetically instead of by measuring rows that do not exist.

use rosace::prelude::*;
use rosace::scroll::ScrollController;

use crate::feedback::Feedback;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

/// A fixed height so the virtualized list has a viewport to virtualize
/// against — inside a `ScrollView` the vertical axis is unbounded, and a
/// list that is "as tall as its content" cannot virtualize anything.
fn viewport(child: impl Widget + 'static) -> impl Widget {
    Container::new().height(260.0).radius(10.0).child(child)
}

/// A scrubber wired to a list in BOTH directions.
///
/// Dragging the slider scrolls the list; scrolling the list moves the bar.
/// Neither side measures anything: `scroll_to_index` puts a row in view from
/// its fixed extent, and `on_scroll` reports the position back.
///
/// The classic version of this in other toolkits is a pile of arithmetic in
/// app code — item height, content height, viewport height, clamp. None of
/// that is here.
fn scrubbed_list(progress: &Atom<f32>, ctrl: &ScrollController, fb: &Feedback) -> impl Widget {
    const ROWS: usize = 500;
    const EXTENT: f32 = 44.0;

    let (ctrl, progress) = (ctrl.clone(), progress.clone());

    // list -> bar
    ctrl.on_scroll({
        let progress = progress.clone();
        move |[_, y]| {
            let max = (ROWS as f32 * EXTENT - 260.0).max(1.0);
            progress.set((y / max).clamp(0.0, 1.0));
        }
    });

    let f = fb.clone();
    Column::new()
        .spacing(8.0)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        // bar -> list
        .child(
            Slider::new(progress.get())
                .on_change({
                    let ctrl = ctrl.clone();
                    move |v: f32| {
                        let row = ((v * (ROWS - 1) as f32).round() as usize).min(ROWS - 1);
                        ListView::scroll_to_index(
                            &ctrl, ROWS, EXTENT, row, rosace::scroll::ScrollAlign::Start,
                        );
                    }
                }),
        )
        .child(Text::caption(format!("Row {}", (progress.get() * (ROWS - 1) as f32) as usize)))
        .child(viewport(
            ListView::builder(ROWS, EXTENT, move |i| {
                let f = f.clone();
                std::sync::Arc::new(
                    ListTile::new(format!("Row {i}"))
                        .on_press(move || f.say(format!("Row {i} tapped"))),
                )
            })
            .controller(ctrl),
        ))
}

/// A horizontally scrolling strip inside each row of a vertically scrolling
/// list.
///
/// The interesting part is which one takes the wheel: scroll routing is
/// axis-aware, so a vertical gesture over a horizontal strip still scrolls the
/// page rather than being swallowed by the strip.
fn nested_scroll(fb: &Feedback) -> impl Widget {
    let f = fb.clone();
    viewport(ListView::builder(60, 92.0, move |row| {
        let f = f.clone();
        let mut strip = Row::new().spacing(8.0);
        for col in 0..12 {
            let f = f.clone();
            strip = strip.child(
                Container::new()
                    .width(90.0)
                    .height(64.0)
                    .radius(8.0)
                    .background(Color::rgb(40 + (col * 8) as u8, 44, 70))
                    .child(Text::caption(format!("{row}.{col}")))
                    .on_press(move || f.say(format!("Cell {row}.{col}"))),
            );
        }
        std::sync::Arc::new(
            Column::new()
                .spacing(4.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .child(Text::caption(format!("Row {row}")))
                .child(ScrollView::new(strip).axis(ScrollAxis::Horizontal)),
        )
    }))
}

pub fn list_view_detail(demo: &crate::app::WidgetDemoState, fb: &Feedback) -> impl Widget {
    let f1 = fb.clone();
    let f2 = fb.clone();
    let f3 = fb.clone();

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "10,000 rows — only the visible ones exist",
                viewport(ListView::builder(10_000, 56.0, move |i| {
                    let f = f1.clone();
                    std::sync::Arc::new(
                        ListTile::new(format!("Row {i}"))
                            .subtitle("Tap to confirm this row is real")
                            .on_press(move || f.say(format!("Row {i} tapped"))),
                    )
                })),
            ))
            .child(labeled(
                "Compact rows (item_extent 36)",
                viewport(ListView::builder(500, 36.0, move |i| {
                    let f = f2.clone();
                    std::sync::Arc::new(
                        ListTile::new(format!("Compact item {i}"))
                            .no_divider()
                            .on_press(move || f.say(format!("Compact item {i} tapped"))),
                    )
                })),
            ))
            .child(labeled(
                "No scrollbar",
                viewport(
                    ListView::builder(200, 44.0, move |i| {
                        let f = f3.clone();
                        std::sync::Arc::new(
                            ListTile::new(format!("Clean row {i}"))
                                .on_press(move || f.say(format!("Clean row {i} tapped"))),
                        )
                    })
                    .no_scrollbar(),
                ),
            ))
            .child(labeled(
                "Scrubber — drag the slider to scroll, scroll to move the slider",
                scrubbed_list(&demo.list_scrub, &demo.list_ctrl, fb),
            ))
            .child(labeled(
                "Nested: a horizontal strip inside every row of a vertical list",
                nested_scroll(fb),
            ))
            .child(labeled(
                "Custom scrollbar colour",
                viewport(
                    ListView::builder(200, 44.0, |i| {
                        std::sync::Arc::new(ListTile::new(format!("Row {i}")))
                    })
                    .scrollbar_color(Color::rgb(230, 120, 60)),
                ),
            )),
    )
}
