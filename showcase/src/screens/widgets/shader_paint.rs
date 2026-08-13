//! `ShaderPaint` — fills its rect with a registered custom shader
//! `ShaderMaterial`. Uses the starter material library (`gradient`/`noise`/
//! `glow`), registered once at app startup (see `app_init` in `lib.rs`).

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

pub fn shader_paint_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Gradient material",
                ShaderPaint::new(materials::gradient(Color::rgb(220, 80, 60), Color::rgb(90, 40, 160), 0.6, 0.0))
                    .size(160.0, 90.0),
            ))
            .child(labeled(
                "Animated (flowing gradient)",
                ShaderPaint::new(materials::gradient(Color::rgb(60, 160, 220), Color::rgb(30, 200, 140), 0.3, 0.4))
                    .size(160.0, 90.0)
                    .animated(),
            ))
            .child(labeled(
                "Noise material",
                ShaderPaint::new(materials::noise(Color::rgb(40, 40, 60), 0.5)).size(160.0, 90.0),
            ))
            .child(labeled(
                "Glow material, animated",
                ShaderPaint::new(materials::glow(Color::rgb(220, 80, 200), 40.0, 0.5))
                    .size(160.0, 90.0)
                    .animated(),
            )),
    )
}
