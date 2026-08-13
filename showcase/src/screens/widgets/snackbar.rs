//! `Snackbar` — `Toast`'s action-bearing sibling: a bottom-anchored message
//! WITH an action button. The app owns an `Atom<bool>`; `Snackbar::show`
//! opens it with an auto-dismiss timer, and the caller guards `.emit()`
//! with that atom every build while it should be visible.

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

pub fn snackbar_detail(open: &Atom<bool>, styled_open: &Atom<bool>) -> impl Widget {
    if open.get() {
        let o = open.clone();
        Snackbar::new("Item archived").action("UNDO", move || o.set(false)).emit();
    }
    if styled_open.get() {
        Snackbar::new("Saved!")
            .height(52.0)
            .radius(10.0)
            .background(Color::rgb(30, 30, 40))
            .color(Color::WHITE)
            .action_color(Color::rgb(220, 80, 60))
            .font_size(14.0)
            .emit();
    }

    let a = open.clone();
    let b = styled_open.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — with an action button, auto-dismiss after 3s",
                Button::new("Archive item").on_press(move || Snackbar::show(&a, 3.0)),
            ))
            .child(labeled(
                "Custom height, radius, and colors",
                Button::new("Show styled snackbar").on_press(move || Snackbar::show(&b, 3.0)),
            )),
    )
}
