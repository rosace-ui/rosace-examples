//! `Accordion` — a collapsible section: clickable header row (title +
//! chevron) with a body that shows only while expanded, animated.

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

pub fn accordion_detail(expanded: &Atom<bool>, styled: &Atom<bool>) -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — tap the header",
                Accordion::new("Section title", expanded.get(), Text::new("The body content, revealed and animated.")),
            ))
            .child(labeled(
                "Custom background, border, radius, elevation, title size",
                Accordion::new("Styled section", styled.get(), Text::new("Styled body content."))
                    .background(Color::rgb(30, 30, 40))
                    .border(Color::rgb(90, 90, 100), 1.0)
                    .radius(16.0)
                    .elevation(6.0)
                    .title_size(18.0),
            )),
    )
}
