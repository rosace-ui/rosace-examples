//! `TextInput` — a live bound field, plus placeholder and disabled states.

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

pub fn text_input_detail(value: &Atom<String>) -> impl Widget {
    let v = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — bound to app state",
                TextInput::new()
                    .value(value.get())
                    .placeholder("Type something…")
                    .on_change(move |s| v.set(s)),
            ))
            .child(Text::new(format!("Current value: \"{}\"", value.get())).color(Color::rgb(120, 120, 120)))
            .child(labeled("Placeholder only", TextInput::new().placeholder("Placeholder text")))
            .child(labeled("Obscured (password-style)", TextInput::new().value("secret").obscure())),
    )
}
