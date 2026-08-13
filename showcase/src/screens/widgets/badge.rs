//! `Badge` — a small count/label/dot marker, usually pinned to a corner.

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

pub fn badge_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("Count", Badge::count(3)))
            .child(labeled("Count, capped display", Badge::count(120)))
            .child(labeled("Label", Badge::label("NEW")))
            .child(labeled("Dot (no text)", Badge::dot()))
            .child(labeled("Custom color", Badge::count(5).color(Color::rgb(220, 80, 60)).text_color(Color::WHITE))),
    )
}
