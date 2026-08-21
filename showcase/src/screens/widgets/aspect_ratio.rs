//! `AspectRatio` — sizes its child to a fixed width:height ratio, fitting
//! within the available space.

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

fn block(color: Color) -> BoxedWidget {
    std::sync::Arc::new(Container::new().background(color).radius(8.0))
}

pub fn aspect_ratio_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("16:9 (video)", AspectRatio::new(16.0 / 9.0, block(Color::rgb(90, 120, 200)))))
            .child(labeled("1:1 (square)", AspectRatio::new(1.0, block(Color::rgb(220, 80, 60)))))
            .child(labeled("4:3", AspectRatio::new(4.0 / 3.0, block(Color::rgb(80, 180, 120))))),
    )
}
