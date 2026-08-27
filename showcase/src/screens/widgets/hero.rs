//! `Hero` — a shared element that morphs between two screens.
//!
//! `.hero_tag("id")` on a widget in the source screen and the same tag on the
//! destination: during the navigation transition the framework matches them by
//! tag and flies a single element between their two rects, interpolating
//! position AND size.
//!
//! **A hero only exists mid-navigation.** Outside a transition `.hero_tag(..)`
//! is a total pass-through, painting exactly as if it were not there — so a
//! static page cannot show one. That is why this demo is two screens with a
//! real `nav.push`: tap a thumbnail and watch it grow into the detail image,
//! then go back and watch it shrink into the grid.

use rosace::prelude::*;

use crate::app::Screen;

/// The four demo tiles. Index doubles as the hero tag and the route payload,
/// so tapping tile 2 flies tile 2 — matching by tag is what makes a hero a
/// SHARED element rather than a coincidence of geometry.
const SWATCHES: [(u8, u8, u8); 4] = [
    (86, 118, 220),
    (214, 106, 92),
    (94, 176, 128),
    (198, 150, 74),
];

fn tag(i: usize) -> String {
    format!("hero-swatch-{i}")
}

fn swatch(i: usize) -> Container {
    let (r, g, b) = SWATCHES[i];
    Container::new().background(Color::rgb(r, g, b)).radius(12.0)
}

/// Source screen: a grid of small tiles, each pushing its own detail route.
pub fn hero_detail(nav: &ScreenNav<Screen>) -> impl Widget {
    let mut grid = Row::new().spacing(12.0);
    for i in 0..SWATCHES.len() {
        let nav = nav.clone();
        grid = grid.child(
            swatch(i)
                .width(64.0)
                .height(64.0)
                .on_press(move || nav.push(Screen::HeroDetail(i)))
                .hero_tag(tag(i)),
        );
    }

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new("Tap a tile — it flies and grows into the next screen.")
                .color(Color::rgb(120, 120, 120)))
            .child(grid)
            .child(
                Text::caption(
                    "Each tile carries `.hero_tag(\"hero-swatch-N\")`. The destination \
                     screen tags a large version with the SAME id, and the transition \
                     interpolates one element between the two rects.",
                )
                .color(Color::rgb(120, 120, 120)),
            ),
    )
}

/// Destination screen: the same swatch, large, under the same tag.
pub fn hero_destination(i: usize, nav: &ScreenNav<Screen>) -> impl Widget {
    let nav_back = nav.clone();
    let i = i.min(SWATCHES.len() - 1);

    Scaffold::new(
        ScrollView::new(
            Column::new()
                .padding(EdgeInsets::all(16.0))
                .spacing(16.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                // Same tag as the tile that was tapped — this is the other
                // end of the flight.
                .child(swatch(i).width(280.0).height(200.0).hero_tag(tag(i)))
                .child(Text::title(format!("Swatch {i}")))
                .child(
                    Text::caption(
                        "Going back flies it in reverse. The element is matched by TAG, \
                         not by position — tapping a different tile flies that one.",
                    )
                    .color(Color::rgb(120, 120, 120)),
                )
                .child(Button::new("Back").on_press(move || { nav_back.pop(); })),
        ),
    )
    .app_bar(AppBar::new("Hero").back_button(nav))
}
