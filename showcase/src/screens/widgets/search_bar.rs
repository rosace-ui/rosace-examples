//! `SearchBar` — a `TextInput` preset with a leading search icon (and an
//! optional trailing clear ×), one pill rather than icon-beside-field.

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

pub fn search_bar_detail(value: &Atom<String>, fb: &Feedback) -> impl Widget {
    let v = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — \"{}\"", value.get()),
                SearchBar::new().value(value.get()).on_change(move |s| v.set(s)),
            ))
            .child(labeled("Custom placeholder", SearchBar::new().placeholder("Search widgets\u{2026}")))
            .child(labeled("Custom width and height", SearchBar::new().width(220.0).height(44.0)))
            .child(labeled(
                "With a clear (×) button — shown once the value is non-empty",
                SearchBar::new().value("rosace").on_clear(fb.tap("Cleared")),
            )),
    )
}
