//! `Tooltip` — a hover label that wraps any widget; also the everyday
//! `.tooltip("...")` builder sugar via `WidgetExt`.

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

pub fn tooltip_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Hover to see it — Tooltip::new",
                Tooltip::new("A helpful tip", Button::new("Hover me")),
            ))
            .child(labeled(
                "Everyday sugar — .tooltip(\"...\") on any widget",
                Button::new("Also hover me").tooltip("Attached via WidgetExt"),
            ))
            .child(labeled(
                "Custom font size",
                Tooltip::new("Bigger label", Button::new("Hover")).font_size(16.0),
            ))
            .child(labeled(
                "Custom style (background, color, radius, font size)",
                Tooltip::new("Styled tooltip", Button::new("Hover")).style(TooltipStyle {
                    background: Color::rgb(220, 80, 60),
                    text_color: Color::WHITE,
                    radius: 10.0,
                    font_size: 14.0,
                    ..TooltipStyle::default()
                }),
            )),
    )
}
