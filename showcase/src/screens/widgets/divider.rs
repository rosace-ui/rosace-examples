//! `Divider` — a thin separating line, horizontal or vertical.

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

pub fn divider_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Horizontal (default)",
                Column::new()
                    .spacing(8.0)
                    .child(Text::new("Above"))
                    .child(Divider::new())
                    .child(Text::new("Below")),
            ))
            .child(labeled("Custom thickness + color", Divider::new().thickness(3.0).color(Color::rgb(220, 80, 60))))
            .child(labeled("Indented (e.g. to align past a leading icon)", Divider::new().indent(32.0)))
            .child(labeled(
                "Vertical",
                // A vertical divider fills the height it is GIVEN, so it needs
                // a bounded one. This page sits in a vertical `ScrollView`,
                // which offers infinite height — so the extent is set here
                // explicitly rather than left to the parent.
                Row::new()
                    .cross_axis_alignment(CrossAxisAlignment::Center)
                    .spacing(8.0)
                    .child(Text::new("Left"))
                    .child(Container::new().height(20.0).child(Divider::vertical()))
                    .child(Text::new("Right")),
            )),
    )
}
