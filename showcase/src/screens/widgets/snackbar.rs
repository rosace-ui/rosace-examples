//! `Snackbar` — `Toast`'s action-bearing sibling: a bottom-anchored message
//! WITH an action button. The app owns an `Atom<bool>`; `Snackbar::show`
//! opens it with an auto-dismiss timer, and the caller guards `.emit()`
//! with that atom every build while it should be visible.

use rosace::prelude::*;
use crate::present::PresentExt;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn snackbar_detail(open: &Atom<bool>, styled_open: &Atom<bool>) -> impl Widget {
    // A snackbar promotes into the root layer, declared during PAINT — so it
    // needs a paint context, which `build` has not got. `present` supplies one.
    let (s1, s2) = (open.clone(), styled_open.clone());

    let a = open.clone();
    let b = styled_open.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — with an action button, auto-dismiss after 3s",
                Button::new("Archive item").on_press(move || { a.set(true); let o = a.clone(); Snackbar::dismiss_after(3.0, move || o.set(false)); }),
            ))
            .child(labeled(
                "Custom height, radius, and colors",
                Button::new("Show styled snackbar").on_press(move || { b.set(true); let o = b.clone(); Snackbar::dismiss_after(3.0, move || o.set(false)); }),
            )),
    )
    .present(move |ctx| {
        if s1.get() {
            let o = s1.clone();
            Snackbar::new("Item archived")
                .action("UNDO", move || o.set(false))
                .emit(ctx);
        }
        if s2.get() {
            Snackbar::new("Saved!")
                .height(52.0)
                .radius(10.0)
                .background(Color::rgb(30, 30, 40))
                .color(Color::WHITE)
                .action_color(Color::rgb(220, 80, 60))
                .font_size(14.0)
                .emit(ctx);
        }
    })
}
