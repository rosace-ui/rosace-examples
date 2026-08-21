//! `Container` — the fundamental box: shape, background/gradient, border,
//! shadow, radius, padding/margin, size, alignment, clipping. Everything
//! box-shaped is a `Container` (D095) — there is no separate ColoredBox /
//! CircleBox / GradientBox.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn container_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Background + radius",
                Container::new().background(Color::rgb(90, 120, 200)).radius(12.0).size(120.0, 60.0),
            ))
            .child(labeled(
                "Border",
                Container::new().border(Color::rgb(220, 80, 60), 2.0).radius(8.0).size(120.0, 60.0),
            ))
            .child(labeled(
                "Shadow",
                Container::new()
                    .background(Color::rgb(30, 30, 40))
                    .radius(12.0)
                    .shadow(Color::rgba(0, 0, 0, 120), 16.0)
                    .size(120.0, 60.0),
            ))
            .child(labeled(
                "Elevation (shortcut for a soft drop shadow)",
                Container::new().background(Color::rgb(245, 245, 245)).radius(12.0).elevation(8.0).size(120.0, 60.0),
            ))
            .child(labeled(
                "Vertical gradient",
                Container::new().gradient(Color::rgb(220, 80, 60), Color::rgb(90, 40, 160)).radius(12.0).size(160.0, 60.0),
            ))
            .child(labeled(
                "Horizontal gradient",
                Container::new().gradient_h(Color::rgb(80, 180, 120), Color::rgb(40, 100, 200)).radius(12.0).size(160.0, 60.0),
            ))
            .child(labeled(
                "Circle shape",
                Container::new().background(Color::rgb(220, 160, 60)).circle().size(60.0, 60.0),
            ))
            .child(labeled(
                "Stadium (pill) shape",
                Container::new().background(Color::rgb(60, 160, 220)).stadium().size(140.0, 44.0),
            ))
            .child(labeled(
                "Padding + margin around a child",
                Container::new()
                    .background(Color::rgb(240, 240, 240))
                    .padding(EdgeInsets::all(12.0))
                    .margin(EdgeInsets::all(8.0))
                    .radius(8.0)
                    .child(Text::new("Padded content")),
            ))
            .child(labeled(
                "Min size (grows to fit a larger child)",
                Container::new()
                    .background(Color::rgb(240, 240, 240))
                    .min_size(160.0, 40.0)
                    .radius(8.0)
                    .child(Text::new("Small")),
            ))
            .child(labeled(
                "Alignment of a child within a fixed box",
                Container::new()
                    .background(Color::rgb(240, 240, 240))
                    .size(160.0, 80.0)
                    .radius(8.0)
                    .align(Alignment::BottomRight)
                    .child(Text::new("Bottom-right")),
            ))
            .child(labeled(
                "Clip (child larger than the box gets clipped)",
                Container::new()
                    .background(Color::rgb(240, 240, 240))
                    .size(120.0, 50.0)
                    .radius(8.0)
                    .clip()
                    .child(Container::new().background(Color::rgb(220, 80, 60)).size(200.0, 100.0)),
            ))
            .child(labeled(
                "Shader material fill (replaces the background)",
                Container::new()
                    .radius(12.0)
                    .size(160.0, 60.0)
                    .material(materials::gradient(
                        Color::rgb(220, 80, 60), Color::rgb(90, 40, 160), 0.6, 0.0,
                    )),
            )),
    )
}
