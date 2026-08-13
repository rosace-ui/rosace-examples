//! `ProgressBar` + `CircularProgress` — determinate and indeterminate
//! progress indicators, linear and circular.
//!
//! Progress vs [`Slider`]: Progress is a display-only OUTPUT (no
//! `on_change`, no drag/tap handling at all — it just paints whatever value
//! you feed it, driven by app state like a download percentage). Slider is
//! an interactive INPUT the user drags to set a value themselves. The last
//! example on this page drives a ProgressBar with a real Slider so the
//! difference is visible side by side.

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

pub fn progress_detail(value: &Atom<f32>) -> impl Widget {
    let v = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Linear — {:.0}%", value.get() * 100.0),
                ProgressBar::new(value.get()),
            ))
            .child(labeled(
                "Custom color",
                ProgressBar::new(0.6).color(Color::rgb(220, 80, 60)).track_color(Color::rgb(240, 225, 222)),
            ))
            .child(labeled(
                &format!("Circular — {:.0}%", value.get() * 100.0),
                CircularProgress::new(value.get()),
            ))
            .child(labeled("Circular — indeterminate spinner", CircularProgress::spinner()))
            .child(labeled(
                "Drive the value with the slider from the Slider page — try it here too",
                Slider::new(value.get()).on_change(move |x| v.set(x)),
            )),
    )
}
