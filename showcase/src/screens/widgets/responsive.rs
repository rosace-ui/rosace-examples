//! `Responsive` — build a different tree depending on how much room there
//! is. Resize the window (or rotate the device) and these rebuild live.

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

fn swatch(c: Color, label: &str) -> impl Widget {
    Container::new().height(64.0).radius(8.0).background(c)
        .child(Text::new(label.to_string()))
}

pub fn responsive_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Reports the width it was given",
                Responsive::new(|space| {
                    std::sync::Arc::new(Text::new(format!("{:.0} px wide", space.width)))
                }),
            ))
            .child(labeled(
                "Column on a narrow window, Row on a wide one",
                Responsive::new(|space| {
                    let a = swatch(Color::rgb(70, 110, 190), "A");
                    let b = swatch(Color::rgb(190, 110, 70), "B");
                    if space.width >= breakpoint::COMPACT {
                        std::sync::Arc::new(Row::new().spacing(8.0)
                            .child(Expanded::new(a))
                            .child(Expanded::new(b)))
                    } else {
                        std::sync::Arc::new(Column::new().spacing(8.0).child(a).child(b))
                    }
                }),
            ))
            .child(labeled(
                "Column count follows the breakpoints",
                Responsive::new(|space| {
                    let cols = if space.width >= breakpoint::EXPANDED { 4 }
                               else if space.width >= breakpoint::COMPACT { 2 }
                               else { 1 };
                    let mut row = Row::new().spacing(8.0);
                    for i in 0..cols {
                        row = row.child(Expanded::new(swatch(
                            Color::rgb(60 + i as u8 * 30, 90, 170),
                            &format!("{}/{}", i + 1, cols),
                        )));
                    }
                    std::sync::Arc::new(Column::new().spacing(6.0)
                        .child(Text::new(format!("{cols} column(s)")))
                        .child(row))
                }),
            ))
            .child(Text::new(
                "An unbounded axis reports 0, not infinity — so a width check \
                 inside a ScrollView does not silently take the widest branch.",
            )),
    )
}
