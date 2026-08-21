//! `ListView` — a VIRTUALIZED list: only the rows inside the viewport are
//! ever built, so 10,000 rows cost the same as 20.
//!
//! That is the whole point of the widget, and it is also its one constraint:
//! every row must be `item_extent` tall, because the scroll position is
//! computed arithmetically instead of by measuring rows that do not exist.

use rosace::prelude::*;

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

pub fn list_view_detail(fb: &Feedback) -> impl Widget {
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
