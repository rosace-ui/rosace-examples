//! `Autocomplete` — a text field that filters a list as you type and drops
//! the matches into an overlay below it.

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

fn countries() -> Vec<&'static str> {
    vec![
        "Argentina", "Australia", "Austria", "Belgium", "Brazil", "Canada",
        "Denmark", "Egypt", "Finland", "France", "Germany", "Greece", "India",
        "Indonesia", "Ireland", "Italy", "Japan", "Kenya", "Mexico",
        "Netherlands", "New Zealand", "Norway", "Peru", "Poland", "Portugal",
        "Singapore", "Spain", "Sweden", "Switzerland", "Thailand", "Turkey",
    ]
}

pub fn autocomplete_detail(
    value: &Atom<String>,
    open: &Atom<bool>,
    limited_value: &Atom<String>,
    limited_open: &Atom<bool>,
    fb: &Feedback,
) -> impl Widget {
    let (v1, f1) = (value.clone(), fb.clone());
    let (v2, f2) = (limited_value.clone(), fb.clone());

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Type to filter — try \"an\"",
                Autocomplete::new(countries(), open.get())
                    .value(value.get())
                    .placeholder("Country")
                    .on_change(move |t| v1.set(t))
                    .on_select(move |picked| f1.say(format!("Selected {picked}"))),
            ))
            .child(labeled(
                "max_visible(3) — the overlay caps its height",
                Autocomplete::new(countries(), limited_open.get())
                    .value(limited_value.get())
                    .placeholder("At most three suggestions")
                    .max_visible(3)
                    .width(280.0)
                    .on_change(move |t| v2.set(t))
                    .on_select(move |picked| f2.say(format!("Picked {picked}"))),
            ))
            .child(Text::new(
                "The suggestion list is an overlay, so it floats above whatever \
                 follows it instead of pushing the page down.",
            )),
    )
}
