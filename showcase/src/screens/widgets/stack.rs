//! `Stack` + `Positioned` — overlap children instead of arranging them in a
//! line. Later children paint on top.

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

fn block(w: f32, h: f32, c: Color, label: &str) -> impl Widget {
    Container::new().width(w).height(h).radius(8.0).background(c)
        .child(Text::new(label.to_string()))
}

pub fn stack_detail(fb: &Feedback) -> impl Widget {
    let f = fb.clone();

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Overlapping children — last one wins",
                Stack::new()
                    .child(block(200.0, 120.0, Color::rgb(60, 70, 130), "back"))
                    .child(block(140.0, 80.0, Color::rgb(90, 110, 190), "middle"))
                    .child(block(80.0, 44.0, Color::rgb(150, 170, 240), "front")),
            ))
            .child(labeled(
                "Positioned — anchored to the stack's edges",
                Container::new().height(160.0).radius(10.0).child(
                    Stack::new()
                        .child(block(300.0, 160.0, Color::rgb(45, 50, 90), ""))
                        .child(Positioned::new(block(64.0, 32.0, Color::rgb(220, 90, 90), "TL")).top(8.0).left(8.0))
                        .child(Positioned::new(block(64.0, 32.0, Color::rgb(90, 180, 120), "TR")).top(8.0).right(8.0))
                        .child(Positioned::new(block(64.0, 32.0, Color::rgb(220, 170, 60), "BL")).bottom(8.0).left(8.0))
                        .child(Positioned::new(block(64.0, 32.0, Color::rgb(150, 120, 230), "BR")).bottom(8.0).right(8.0)),
                ),
            ))
            .child(labeled(
                "A badge pinned to a corner — the everyday use",
                Stack::new()
                    .child(
                        Button::new("Inbox")
                            .on_press(f.tap("Inbox opened")),
                    )
                    .child(Positioned::new(Badge::label("9")).top(-4.0).right(-6.0)),
            ))
            .child(Text::new(
                "A Stack sizes itself to its largest child unless you give it \
                 StackFit::Expand, which makes every child fill it instead.",
            )),
    )
}
