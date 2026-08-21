//! `Menu` — a vertical list of pressable rows, the standard dropdown
//! content, presented via the co-located `.dropdown()` overlay API.

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

pub fn menu_detail(open: &Atom<bool>, styled_open: &Atom<bool>, fb: &Feedback) -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Default",
                {
                    let o = open.clone();
                    Button::new("File")
                        // `.dropdown()` only DECLARES the overlay — it never
                        // toggles `open` itself (found live: the trigger
                        // absorbed clicks and did nothing, same class of gap
                        // as Dropdown's own pre-toggle-fix bug).
                        .on_press(move || o.set(!o.get()))
                        .dropdown(open.clone(), {
                            let o = open.clone();
                            let fb = fb.clone();
                            move || {
                                let (o1, f1) = (o.clone(), fb.clone());
                                let (o2, f2) = (o.clone(), fb.clone());
                                std::sync::Arc::new(
                                    Menu::new()
                                        .item("New", move || { o1.set(false); f1.say("New"); })
                                        .item("Open", move || { o2.set(false); f2.say("Open"); }),
                                )
                            }
                        })
                },
            ))
            .child(labeled(
                "Custom width, row height, radius, and colors",
                {
                    let o = styled_open.clone();
                    // The overlay builder is `'static`, so it cannot borrow
                    // `fb` — it owns a clone (all `Atom`s, so this is cheap
                    // and shares the same toast channel).
                    let fb = fb.clone();
                    Button::new("Options")
                        .on_press(move || o.set(!o.get()))
                        .dropdown(styled_open.clone(), move || {
                            std::sync::Arc::new(
                                Menu::new()
                                    .min_width(220.0)
                                    .row_height(40.0)
                                    .radius(6.0)
                                    .background(Color::rgb(30, 30, 40))
                                    .color(Color::WHITE)
                                    .item("Cut", fb.tap("Cut"))
                                    .item("Copy", fb.tap("Copy"))
                                    .item("Paste", fb.tap("Paste")),
                            )
                        })
                },
            )),
    )
}
