//! `Chip` — a small toggleable pill, usually for filters/tags.

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

pub fn chip_detail(selected: &Atom<bool>) -> impl Widget {
    let s = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — tap to toggle",
                Chip::new("Interactive").selected_if(selected.get()).on_toggle(move |v| s.set(v)),
            ))
            .child(labeled("Selected", Chip::new("Selected").selected()))
            .child(labeled("Unselected", Chip::new("Unselected")))
            .child(labeled("Disabled", Chip::new("Disabled").disabled()))
            .child(labeled("Custom color", Chip::new("Custom").selected().color(Color::rgb(220, 80, 60)))),
    )
}
