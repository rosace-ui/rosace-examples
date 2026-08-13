//! `InteractiveViewer` — pan and zoom any child. Trackpad pinch, scroll to
//! pan, drag to pan, plus the built-in +/- controls.

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

/// Something with enough detail that zooming is visibly doing something.
fn target() -> impl Widget {
    let mut grid = Column::new().spacing(4.0);
    for row in 0..8 {
        let mut r = Row::new().spacing(4.0);
        for col in 0..8 {
            let shade = 40 + ((row + col) % 2) * 60;
            r = r.child(
                Container::new()
                    .width(48.0)
                    .height(48.0)
                    .radius(6.0)
                    .background(Color::rgb(shade as u8, (shade + 30) as u8, 120))
                    .child(Text::new(format!("{row}{col}"))),
            );
        }
        grid = grid.child(r);
    }
    grid
}

fn stage(child: impl Widget + 'static) -> impl Widget {
    Container::new().height(260.0).radius(10.0).child(child)
}

pub fn interactive_viewer_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Default — pan and zoom, controls bottom-right",
                stage(InteractiveViewer::new(target())),
            ))
            .child(labeled(
                "min_scale(1.0) / max_scale(2.0) — zoom is clamped",
                stage(InteractiveViewer::new(target()).min_scale(1.0).max_scale(2.0)),
            ))
            .child(labeled(
                "no_zoom_controls() — gesture only",
                stage(InteractiveViewer::new(target()).no_zoom_controls()),
            ))
            .child(labeled(
                "unconstrained() — pan freely past the content edges",
                stage(InteractiveViewer::new(target()).unconstrained()),
            )),
    )
}
