//! `Slider` — a draggable value in a range, live and in every state.

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

pub fn slider_detail(value: &Atom<f32>) -> impl Widget {
    let v = value.clone();
    let v2 = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — {:.0}%", value.get() * 100.0),
                Slider::new(value.get()).on_change(move |x| v.set(x)),
            ))
            .child(labeled(
                "Custom range (0–200)",
                Slider::new(50.0).range(0.0, 200.0, 50.0).on_change(move |x| v2.set(x / 200.0)),
            ))
            .child(labeled("Disabled", Slider::new(0.3).disabled().on_change(|_| {})))
            .child(labeled(
                "Custom colors",
                Slider::new(0.7)
                    .track_color(Color::rgb(230, 230, 235))
                    .fill_color(Color::rgb(220, 80, 60))
                    .thumb_color(Color::rgb(220, 80, 60))
                    .on_change(|_| {}),
            )),
    )
}
