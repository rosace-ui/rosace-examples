//! `RatingBar` — a row of tappable stars showing (and optionally setting)
//! a rating.

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

pub fn rating_bar_detail(value: &Atom<f32>) -> impl Widget {
    let v = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — {:.0}/5", value.get()),
                RatingBar::new(value.get()).on_change(move |r| v.set(r)),
            ))
            .child(labeled("Read-only (no on_change)", RatingBar::new(3.0)))
            .child(labeled("Disabled", RatingBar::new(2.0).disabled()))
            .child(labeled("Custom count (10 stars)", RatingBar::new(7.0).count(10)))
            .child(labeled("Custom size and spacing", RatingBar::new(3.0).size(32.0).spacing(10.0)))
            .child(labeled(
                "Custom colors",
                RatingBar::new(4.0).color(Color::rgb(220, 80, 60)).empty_color(Color::rgb(230, 230, 230)),
            )),
    )
}
