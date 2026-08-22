//! `Sheet` — a bottom sheet surface: full-width panel with rounded top
//! corners and a grab handle, presented via the co-located `.sheet()`
//! overlay API.

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

fn sheet_body(label: &str) -> BoxedWidget {
    std::sync::Arc::new(Column::new().padding(EdgeInsets::all(16.0)).spacing(8.0).child(Text::new(label)))
}

pub fn sheet_detail(
    default_open: &Atom<bool>, detent_open: &Atom<bool>, full_open: &Atom<bool>, styled_open: &Atom<bool>,
) -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            // `.sheet()` only DECLARES the overlay — it never opens it
            // itself (found live: every trigger absorbed clicks and did
            // nothing), same gap as `.dialog()`/`.dropdown()`.
            .child(labeled(
                "Default (natural content height, grab handle)",
                {
                    let o = default_open.clone();
                    Button::new("Open sheet")
                        .on_press(move || o.set(true))
                        .sheet_bound(&default_open, || {
                            std::sync::Arc::new(Sheet::new(sheet_body("Default sheet content")))
                        })
                },
            ))
            .child(labeled(
                "Detent (fraction of window height)",
                {
                    let o = detent_open.clone();
                    Button::new("Open half-height sheet")
                        .on_press(move || o.set(true))
                        .sheet_bound(&detent_open, || {
                            std::sync::Arc::new(Sheet::new(sheet_body("Half the window")).detent(0.5))
                        })
                },
            ))
            .child(labeled(
                "Full screen + scrollable content",
                {
                    let o = full_open.clone();
                    Button::new("Open full-screen sheet")
                        .on_press(move || o.set(true))
                        .sheet_bound(&full_open, || {
                            let mut col = Column::new().padding(EdgeInsets::all(16.0)).spacing(8.0);
                            for i in 0..30 {
                                col = col.child(Text::new(format!("Row {i}")));
                            }
                            std::sync::Arc::new(Sheet::new(col).full_screen().scrollable())
                        })
                },
            ))
            .child(labeled(
                "No handle, custom radius/background/padding",
                {
                    let o = styled_open.clone();
                    Button::new("Open styled sheet")
                        .on_press(move || o.set(true))
                        .sheet_bound(&styled_open, || {
                            std::sync::Arc::new(
                                Sheet::new(sheet_body("Styled sheet"))
                                    .no_handle()
                                    .radius(28.0)
                                    .padding(EdgeInsets::all(24.0))
                                    .background(Color::rgb(30, 30, 40))
                                    .handle_color(Color::rgb(90, 90, 100)),
                            )
                        })
                },
            )),
    )
}
