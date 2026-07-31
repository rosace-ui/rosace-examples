//! The home screen — an index of the app's routes.

use rosace::prelude::*;

use crate::app::Screen;

pub fn home_screen(nav: &ScreenNav<Screen>) -> impl Widget {
    let nav_counter = nav.clone();
    let nav_platform_channel = nav.clone();
    Column::new()
        .padding(EdgeInsets::all(16.0))
        .child(
            ListTile::new("Counter")
                .subtitle("A simple counter with + / \u{2212}")
                .on_press(move || {
                    nav_counter.push(Screen::Counter);
                }),
        )
        .child(
            ListTile::new("Platform Channel")
                .subtitle("Talk to native code: device info, camera permission, sync dispatch")
                .on_press(move || {
                    nav_platform_channel.push(Screen::PlatformChannel);
                }),
        )
}
