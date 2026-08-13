//! `AppBar` — a top bar with title, leading, and trailing action slots.
//! Platform-adaptive (D105): height/traffic-lights/title alignment default
//! from the active theme's `app_bar` style; every one of THIS app's own
//! screens already uses one — see `app.rs`.

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

pub fn app_bar_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("Default", AppBar::new("Title")))
            .child(labeled(
                "Leading + actions",
                AppBar::new("Inbox")
                    .leading(Icon::new(IconKind::Menu))
                    .action(Icon::new(IconKind::Search))
                    .action(Icon::new(IconKind::Settings)),
            ))
            .child(labeled(
                "Custom height, background, foreground, title size",
                AppBar::new("Styled")
                    .height(56.0)
                    .background(Color::rgb(30, 30, 40))
                    .foreground(Color::WHITE)
                    .title_size(18.0),
            ))
            .child(labeled("Traffic lights (mockup-only decoration)", AppBar::new("macOS-style").traffic_lights()))
            .child(labeled("No traffic lights", AppBar::new("No dots").no_traffic_lights()))
            .child(labeled(
                "Shader material fill",
                AppBar::new("Material").material(materials::gradient(
                    Color::rgb(220, 80, 60), Color::rgb(90, 40, 160), 0.6, 0.0,
                )),
            )),
    )
}
