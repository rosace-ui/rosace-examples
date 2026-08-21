//! `Card` — a bordered, elevated content container.

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

pub fn card_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Default",
                Card::new(Text::new("A card is just a themed, padded container.")),
            ))
            .child(labeled(
                "Elevated",
                Card::new(Text::new("Higher elevation reads as more prominent.")).elevation(4.0),
            ))
            .child(labeled(
                "No border",
                Card::new(Text::new("Background + elevation only, no outline.")).no_border(),
            ))
            .child(labeled(
                "Custom color + radius",
                Card::new(Text::new("Background, border, and radius are all overridable."))
                    .background(Color::rgb(250, 240, 238))
                    .border(Color::rgb(220, 80, 60))
                    .radius(18.0),
            )),
    )
}
