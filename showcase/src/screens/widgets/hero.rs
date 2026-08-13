//! `Hero` — shared-element transition support: `.hero_tag("id")` wraps a
//! widget so it morphs into a same-tagged `Hero` on the destination screen
//! during a navigation transition (see `ScreenTransitionView`). Outside an
//! active transition it is a total pass-through — same paint output as not
//! wrapping at all, which is exactly what this static page shows; the
//! morph itself only appears mid-navigation, not on a page that IS one.

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

pub fn hero_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Tagged widget — a pass-through outside a transition",
                Container::new()
                    .background(Color::rgb(90, 120, 200))
                    .radius(12.0)
                    .size(120.0, 80.0)
                    .hero_tag("showcase-hero-demo"),
            ))
            .child(labeled(
                "How it's used",
                Text::new(
                    "Give the SAME tag to a widget on this screen and one on the \
                     destination screen (e.g. an Avatar on a list row and a larger \
                     Avatar on its detail page). When ScreenNav pushes between them, \
                     the two captured pictures fly and resize between their two rects \
                     instead of one fading out while the other fades in.",
                )
                .color(Color::rgb(120, 120, 120)),
            )),
    )
}
