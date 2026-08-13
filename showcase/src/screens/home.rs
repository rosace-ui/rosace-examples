//! The hub — a card list of top-level sections. Add a section by adding
//! one `ListTile` here and (if it's a real destination, not "Upcoming") a
//! `Screen` variant + match arm in `app.rs`.

use rosace::prelude::*;

use crate::app::Screen;

/// A dimmed, unpressable tile for a section that isn't built yet — visible
/// in the list (so the shape of the app is obvious) without pretending to
/// be a working destination.
fn upcoming_tile(title: &str) -> ListTile {
    ListTile::new(title)
        .subtitle("Upcoming")
        .title_color(Color::rgb(150, 150, 150))
}

pub fn home_screen(nav: &ScreenNav<Screen>) -> impl Widget {
    let nav_widgets = nav.clone();
    let nav_platform_channel = nav.clone();
    Column::new()
        .padding(EdgeInsets::all(16.0))
        .child(
            ListTile::new("Widgets")
                .subtitle("Every widget, one dedicated page each")
                .on_press(move || nav_widgets.push(Screen::Widgets)),
        )
        .child(
            ListTile::new("Platform Channel")
                .subtitle("Talk to native code: device info, permissions, sync calls")
                .on_press(move || nav_platform_channel.push(Screen::PlatformChannel)),
        )
        .child(upcoming_tile("Persistence"))
        .child(upcoming_tile("Network"))
}
