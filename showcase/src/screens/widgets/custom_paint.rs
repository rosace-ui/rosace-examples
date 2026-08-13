//! `CustomPaint` — a leaf widget that draws with a closure, recording
//! DrawCommands through the standard `PaintCtx` (so caching, replay,
//! clipping, and HiDPI scaling all apply, same as any built-in widget).

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

pub fn custom_paint_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "A hand-painted circle",
                CustomPaint::new(|cx, size| {
                    cx.fill_circle(
                        Point { x: cx.rect.origin.x + size.width / 2.0, y: cx.rect.origin.y + size.height / 2.0 },
                        size.width.min(size.height) / 2.0,
                        Color::rgb(220, 80, 60),
                    );
                })
                .size(80.0, 80.0),
            ))
            .child(labeled(
                "Concentric rings (arbitrary drawing logic)",
                CustomPaint::new(|cx, size| {
                    let center = Point { x: cx.rect.origin.x + size.width / 2.0, y: cx.rect.origin.y + size.height / 2.0 };
                    let max_r = size.width.min(size.height) / 2.0;
                    for i in 0..4 {
                        let t = i as f32 / 4.0;
                        cx.fill_circle(center, max_r * (1.0 - t), Color::rgba(90, 120, 200, 255 - (i * 50) as u8));
                    }
                })
                .size(120.0, 120.0),
            ))
            .child(labeled(
                "Width/height only (no square constraint)",
                CustomPaint::new(|cx, size| {
                    cx.fill_rect(cx.rect, Color::rgb(40, 40, 40));
                    let bar_w = size.width * 0.6;
                    cx.fill_rect(
                        Rect {
                            origin: Point { x: cx.rect.origin.x + 8.0, y: cx.rect.origin.y + size.height / 2.0 - 4.0 },
                            size: Size { width: bar_w, height: 8.0 },
                        },
                        Color::rgb(80, 180, 120),
                    );
                })
                .width(200.0)
                .height(40.0),
            )),
    )
}
