//! `Button` — every variant side by side, plus a live press counter proving
//! `on_press` actually fires.

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

pub fn button_detail(presses: &Atom<i32>, fb: &Feedback) -> impl Widget {
    let p = presses.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — press count",
                Button::new(format!("Pressed {} times", presses.get()))
                    .on_press(move || p.set(p.get() + 1)),
            ))
            .child(labeled("Primary", Button::new("Primary").variant(ButtonVariant::Primary).on_press(fb.tap("Primary pressed"))))
            .child(labeled(
                "Secondary",
                Button::new("Secondary").variant(ButtonVariant::Secondary).on_press(fb.tap("Secondary pressed")),
            ))
            .child(labeled("Ghost", Button::new("Ghost").variant(ButtonVariant::Ghost).on_press(fb.tap("Ghost pressed"))))
            .child(labeled("Link", Button::new("Link").variant(ButtonVariant::Link).on_press(fb.tap("Link pressed"))))
            .child(labeled("Success", Button::new("Success").variant(ButtonVariant::Success).on_press(fb.tap("Success pressed"))))
            .child(labeled("Danger", Button::new("Danger").variant(ButtonVariant::Danger).on_press(fb.tap("Danger pressed"))))
            .child(labeled(
                "Disabled",
                Button::new("Disabled").variant(ButtonVariant::Primary).disabled()
                    .on_press(fb.tap("This should never appear — the button is disabled")),
            )),
    )
}
