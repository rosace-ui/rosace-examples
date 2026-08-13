//! `Grid` — a fixed-column grid with three placement modes: uniform
//! (default), staggered (masonry), and bento (a fixed lattice with
//! multi-cell spans).

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

fn tile(color: Color, h: f32) -> BoxedWidget {
    Box::new(Container::new().background(color).radius(8.0).height(h))
}

/// Bento cells are sized by the grid's fixed lattice, not the child — no
/// `.height()` here so `Container` just fills whatever cell it's given.
fn bento_tile(color: Color) -> BoxedWidget {
    Box::new(Container::new().background(color).radius(8.0))
}

pub fn grid_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Uniform (default) — 3 columns",
                Grid::new(3)
                    .child(tile(Color::rgb(220, 80, 60), 60.0))
                    .child(tile(Color::rgb(80, 130, 220), 60.0))
                    .child(tile(Color::rgb(80, 180, 120), 60.0))
                    .child(tile(Color::rgb(180, 120, 200), 60.0)),
            ))
            .child(labeled(
                "Custom spacing/run_spacing",
                Grid::new(2)
                    .spacing(20.0)
                    .run_spacing(4.0)
                    .child(tile(Color::rgb(220, 160, 60), 50.0))
                    .child(tile(Color::rgb(60, 160, 220), 50.0)),
            ))
            .child(labeled(
                "Staggered (masonry)",
                Grid::new(3)
                    .staggered()
                    .child(tile(Color::rgb(220, 80, 60), 40.0))
                    .child(tile(Color::rgb(80, 130, 220), 80.0))
                    .child(tile(Color::rgb(80, 180, 120), 60.0))
                    .child(tile(Color::rgb(180, 120, 200), 100.0))
                    .child(tile(Color::rgb(220, 160, 60), 50.0)),
            ))
            .child(labeled(
                "Bento (fixed lattice with multi-cell spans)",
                Grid::new(3)
                    .row_height(50.0)
                    .child_span(bento_tile(Color::rgb(220, 80, 60)), 2, 1)
                    .child_span(bento_tile(Color::rgb(80, 130, 220)), 1, 2)
                    .child_span(bento_tile(Color::rgb(80, 180, 120)), 1, 1)
                    .child_span(bento_tile(Color::rgb(180, 120, 200)), 1, 1),
            )),
    )
}
