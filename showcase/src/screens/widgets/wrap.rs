//! `Wrap` — a flow layout: children lay left-to-right, wrapping onto a new
//! run when they don't fit the available width.

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

fn pill(label: &str) -> BoxedWidget {
    Box::new(Chip::new(label))
}

pub fn wrap_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Default spacing",
                Wrap::new()
                    .children(vec![
                        pill("Rust"), pill("UI"), pill("Widgets"), pill("Layout"),
                        pill("Flow"), pill("Wrap"), pill("Tags"), pill("Filters"),
                    ]),
            ))
            .child(labeled(
                "Custom spacing + run_spacing",
                Wrap::new()
                    .spacing(20.0)
                    .run_spacing(2.0)
                    .child(pill("A"))
                    .child(pill("B"))
                    .child(pill("C")),
            )),
    )
}
