//! `Skeleton` — a shimmering loading placeholder: a rounded block with a
//! self-animating highlight band sweeping across it.

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

pub fn skeleton_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("Default (fills available width)", Skeleton::new()))
            .child(labeled("Custom width and height", Skeleton::new().width(160.0).height(24.0)))
            .child(labeled("Custom radius", Skeleton::new().width(160.0).radius(2.0)))
            .child(labeled("Custom shimmer color", Skeleton::new().width(160.0).color(Color::rgb(220, 80, 60))))
            .child(labeled("Vertical sweep", Skeleton::new().width(160.0).height(60.0).vertical(true)))
            .child(labeled(
                "Circular (avatar-sized) — pulses instead of sweeping, so the \
                 highlight never paints past the circle's curve",
                Skeleton::circle(48.0),
            ))
            .child(labeled(
                "A typical loading row",
                Row::new()
                    .spacing(10.0)
                    .cross_axis_alignment(CrossAxisAlignment::Center)
                    .child(Skeleton::circle(36.0))
                    .child(
                        Column::new()
                            .spacing(6.0)
                            .child(Skeleton::new().width(140.0))
                            .child(Skeleton::new().width(90.0).height(12.0)),
                    ),
            )),
    )
}
