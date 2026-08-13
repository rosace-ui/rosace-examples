//! `Image` — blits a PNG (file/asset/bytes) onto the canvas; `placeholder`
//! paints a solid color box without touching the filesystem, used here so
//! this page has no asset dependency.

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

pub fn image_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Placeholder (default fit)",
                Image::placeholder(Color::rgb(90, 120, 200)).width(160.0).height(90.0),
            ))
            .child(labeled(
                "Fit — Cover",
                Image::placeholder(Color::rgb(220, 80, 60)).width(160.0).height(90.0).fit(ImageFit::Cover),
            ))
            .child(labeled(
                "Fit — Contain",
                Image::placeholder(Color::rgb(80, 180, 120)).width(160.0).height(90.0).fit(ImageFit::Contain),
            ))
            .child(labeled(
                "Fit — Fill",
                Image::placeholder(Color::rgb(180, 120, 200)).width(160.0).height(90.0).fit(ImageFit::Fill),
            ))
            .child(labeled(
                "Alt text (accessibility label)",
                Image::placeholder(Color::rgb(120, 120, 120)).width(160.0).height(90.0).alt("A placeholder image"),
            ))
            .child(labeled(
                "Broken (file/asset/bytes that fails to decode) — distinct \
                 red X, no longer identical to an intentional placeholder",
                Image::file("this-file-does-not-exist.png").width(160.0).height(90.0),
            )),
    )
}
// No "loading" state is demoed: Image decoding is synchronous (a cache
// lookup + PNG decode right inside `paint()`, not an async fetch), so there
// is no in-progress period to show a spinner for today — naming that
// honestly rather than faking a loading state that can't actually occur.
