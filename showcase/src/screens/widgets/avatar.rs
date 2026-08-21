//! `Avatar` — a circular initials badge.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn avatar_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("Default", Avatar::new("GJ")))
            .child(labeled("Custom size", Avatar::new("RS").size(56.0)))
            .child(labeled(
                "Custom colors",
                Avatar::new("AI").color(Color::rgb(220, 80, 60)).text_color(Color::WHITE),
            ))
            .child(labeled(
                "A row of avatars",
                Row::new()
                    .spacing(8.0)
                    .child(Avatar::new("A"))
                    .child(Avatar::new("B"))
                    .child(Avatar::new("C")),
            )),
    )
}
