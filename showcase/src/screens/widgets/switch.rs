//! `Switch` — on/off with an animated thumb, plus disabled and custom-color
//! variants.

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

pub fn switch_detail(on: &Atom<bool>) -> impl Widget {
    let s = on.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — tap to toggle",
                Switch::new(on.get()).label("Interactive").on_change(move |v| s.set(v)),
            ))
            .child(labeled("On", Switch::new(true).label("On").on_change(|_| {})))
            .child(labeled("Off", Switch::new(false).label("Off").on_change(|_| {})))
            .child(labeled(
                "Disabled",
                Switch::new(true).label("Locked, on").disabled().on_change(|_| {}),
            ))
            .child(labeled(
                "Custom colors",
                Switch::new(true)
                    .label("Custom colors")
                    .on_color(Color::rgb(220, 80, 60))
                    .thumb_color(Color::WHITE)
                    .on_change(|_| {}),
            )),
    )
}
