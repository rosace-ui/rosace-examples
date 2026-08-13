//! `Drawer` — a slide-in side panel. Emitted as a build-time overlay
//! (`Drawer::emit`, same per-frame-while-open convention as
//! `Dialog::emit`/`Snackbar::emit`); a real app calls this from `Scaffold`.

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

fn panel(open: &Atom<bool>, label: &'static str) -> impl Fn() -> BoxedWidget + Send + Sync + 'static {
    let o = open.clone();
    move || {
        let o = o.clone();
        Box::new(
            Column::new()
                .padding(EdgeInsets::all(16.0))
                .spacing(12.0)
                .child(Text::title(label))
                .child(Button::new("Close").on_press(move || o.set(false))),
        )
    }
}

pub fn drawer_detail(
    side_open: &Atom<bool>, full_open: &Atom<bool>, styled_open: &Atom<bool>,
) -> impl Widget {
    // Build-time emitters — push the overlay if their atom is open, same
    // convention as `Dialog::emit`/`Snackbar::emit` (called every rebuild).
    Drawer::new(side_open.clone(), panel(side_open, "Side drawer")).emit();
    Drawer::new(full_open.clone(), panel(full_open, "Full-screen drawer")).full_screen().emit();
    Drawer::new(
        styled_open.clone(),
        panel(styled_open, "Styled drawer"),
    )
    .width(220.0)
    .background(Color::rgb(30, 30, 40))
    .scrim_color(Color::rgba(90, 40, 160, 140))
    .emit();

    let a = side_open.clone();
    let b = full_open.clone();
    let c = styled_open.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Default (280px side panel)",
                Button::new("Open drawer").on_press(move || a.set(true)),
            ))
            .child(labeled(
                "Full screen (mobile nav-page style)",
                Button::new("Open full-screen drawer").on_press(move || b.set(true)),
            ))
            .child(labeled(
                "Custom width, background, and scrim color",
                Button::new("Open styled drawer").on_press(move || c.set(true)),
            )),
    )
}
