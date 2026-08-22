//! `Drawer` — a slide-in side panel. Emitted as a build-time overlay
//! (`Drawer::emit`, same per-frame-while-open convention as
//! `Dialog::emit`/`Snackbar::emit`); a real app calls this from `Scaffold`.

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

fn panel(open: &Atom<bool>, label: &'static str) -> impl Fn() -> BoxedWidget + Send + Sync + 'static {
    let o = open.clone();
    move || {
        let o = o.clone();
        std::sync::Arc::new(
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
    // A drawer promotes into the root layer, which is declared during PAINT —
    // so it needs a paint context. `present` supplies one at the end of this
    // screen's own paint.
    let (d1, d2, d3) = (side_open.clone(), full_open.clone(), styled_open.clone());
    // `Arc`ed so the paint closure (which runs every frame) can hand each
    // drawer its panel builder without consuming it.
    let p1 = std::sync::Arc::new(panel(side_open, "Side drawer"));
    let p2 = std::sync::Arc::new(panel(full_open, "Full-screen drawer"));
    let p3 = std::sync::Arc::new(panel(styled_open, "Styled drawer"));

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
    .present(move |ctx| {
        Drawer::new(d1.get(), { let p = std::sync::Arc::clone(&p1); move || p() })
            .on_open_change({ let d = d1.clone(); move |v| d.set(v) })
            .emit(ctx);
        Drawer::new(d2.get(), { let p = std::sync::Arc::clone(&p2); move || p() })
            .full_screen()
            .on_open_change({ let d = d2.clone(); move |v| d.set(v) })
            .emit(ctx);
        Drawer::new(d3.get(), { let p = std::sync::Arc::clone(&p3); move || p() })
            .width(220.0)
            .background(Color::rgb(30, 30, 40))
            .scrim_color(Color::rgba(90, 40, 160, 140))
            .on_open_change({ let d = d3.clone(); move |v| d.set(v) })
            .emit(ctx);
    })
}
