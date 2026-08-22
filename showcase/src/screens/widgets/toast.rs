//! `Toast` — a transient notification pill, floated above the bottom edge
//! via the co-located `.toast()` overlay API and opened with auto-dismiss
//! via `Toast::show`.

use rosace::prelude::*;
use crate::present::BindOverlay;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn toast_detail(
    info_open: &Atom<bool>, success_open: &Atom<bool>, error_open: &Atom<bool>, styled_open: &Atom<bool>,
) -> impl Widget {
    let i = info_open.clone();
    let s = success_open.clone();
    let e = error_open.clone();
    let c = styled_open.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Info",
                Button::new("Show info toast")
                    .on_press(move || { i.set(true); let o = i.clone(); Toast::dismiss_after(2.5, move || o.set(false)); })
                    .toast_bound(&info_open, || std::sync::Arc::new(Toast::info("Heads up!"))),
            ))
            .child(labeled(
                "Success",
                Button::new("Show success toast")
                    .on_press(move || { s.set(true); let o = s.clone(); Toast::dismiss_after(2.5, move || o.set(false)); })
                    .toast_bound(&success_open, || std::sync::Arc::new(Toast::success("Saved!"))),
            ))
            .child(labeled(
                "Error",
                Button::new("Show error toast")
                    .on_press(move || { e.set(true); let o = e.clone(); Toast::dismiss_after(2.5, move || o.set(false)); })
                    .toast_bound(&error_open, || std::sync::Arc::new(Toast::error("Something went wrong"))),
            ))
            .child(labeled(
                "Custom background, color, accent, and radius",
                Button::new("Show styled toast")
                    .on_press(move || { c.set(true); let o = c.clone(); Toast::dismiss_after(2.5, move || o.set(false)); })
                    .toast_bound(&styled_open, || {
                        std::sync::Arc::new(
                            Toast::info("Styled")
                                .background(Color::rgb(30, 30, 40))
                                .color(Color::WHITE)
                                .accent(Color::rgb(220, 80, 60))
                                .radius(4.0),
                        )
                    }),
            )),
    )
}
