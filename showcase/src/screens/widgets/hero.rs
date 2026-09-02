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

/// A second set of tags, for the travel demo. Distinct from the grid's so
/// the two demos can never pair with each other.
fn far_tag(i: usize) -> String {
    format!("hero-far-{i}")
}

/// The travel demo's own colours. Deliberately NOT reused from `SWATCHES`:
/// sharing one would make the two demos indistinguishable on screen, and a
/// colour that appears twice defeats any test that identifies a hero by it.
const FAR: [(u8, u8, u8); 2] = [
    (72, 160, 168),
    (206, 138, 60),
];

fn far_swatch(n: usize) -> Container {
    let (r, g, b) = FAR[n];
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
            )
            .child(Text::new("Travelling across the screen")
                .color(Color::rgb(120, 120, 120)))
            .child(far_row(nav))
            .child(
                Text::caption(
                    "The pair above sits at the two ENDS of the row and lands centred \
                     near the bottom of the next screen, so the flight is a real \
                     journey rather than a widget growing in place. Nothing about the \
                     two screens knows where the other one puts it — the element is \
                     matched by tag and the framework interpolates whatever rects the \
                     two layouts happen to produce.",
                )
                .color(Color::rgb(120, 120, 120)),
            ),
    )
}

/// Two tiles pushed to opposite ends of the row, so each has a visibly
/// different journey to the same destination.
fn far_row(nav: &ScreenNav<Screen>) -> impl Widget {
    let mut row = Row::new().main_axis_alignment(MainAxisAlignment::SpaceBetween);
    for n in 0..FAR.len() {
        let nav = nav.clone();
        row = row.child(
            far_swatch(n)
                .width(48.0)
                .height(48.0)
                .on_press(move || nav.push(Screen::HeroFar(n)))
                .hero_tag(far_tag(n)),
        );
    }
    row
}

/// Destination for the travel demo: the same swatch, larger, low on the page
/// and centred — a different position AND a different size from its source.
pub fn hero_far_destination(n: usize, nav: &ScreenNav<Screen>) -> impl Widget {
    let nav_back = nav.clone();
    let n = n.min(FAR.len() - 1);

    Scaffold::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(16.0)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .child(Text::caption(
                "It flew from the end of a row near the top to here — centred, lower, \
                 and larger. Go back and it retraces the same path in reverse.",
            ).color(Color::rgb(120, 120, 120)))
            .child(Spacer::new(1.0))
            .child(far_swatch(n).width(180.0).height(180.0).hero_tag(far_tag(n)))
            .child(Spacer::new(1.0))
            .child(Button::new("Back").on_press(move || { nav_back.pop(); })),
    )
    .app_bar(AppBar::new("Hero — travel").back_button(nav))
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
