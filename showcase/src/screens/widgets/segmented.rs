//! `SegmentedControl` — mutually-exclusive choice shown as one connected bar.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    Box::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn segmented_detail(selected: &Atom<usize>) -> impl Widget {
    let s = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it",
                SegmentedControl::new(vec!["Day", "Week", "Month"], selected.get())
                    .on_change(move |i| s.set(i)),
            ))
            .child(labeled("Disabled", SegmentedControl::new(vec!["A", "B"], 0).disabled()))
            .child(labeled(
                "Custom colors",
                SegmentedControl::new(vec!["Low", "High"], 1)
                    .track_color(Color::rgb(240, 225, 222))
                    .selected_color(Color::rgb(220, 80, 60))
                    .selected_text_color(Color::WHITE),
            )),
    )
}
