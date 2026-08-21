//! `Carousel` (alias `PageView`) — a swipeable page container with dot
//! indicators.

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

fn page(label: &str, color: Color) -> BoxedWidget {
    std::sync::Arc::new(
        Container::new()
            .background(color)
            .radius(12.0)
            .child(
                Container::new()
                    .align(Alignment::Center)
                    .child(Text::new(label).color(Color::WHITE)),
            ),
    )
}

pub fn carousel_detail(page_index: &Atom<usize>) -> impl Widget {
    let set_page = page_index.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — swipe, page {}", page_index.get() + 1),
                Carousel::new()
                    .page(page_index.get())
                    .on_page_change(move |i| set_page.set(i))
                    .child(page("Page 1", Color::rgb(220, 80, 60)))
                    .child(page("Page 2", Color::rgb(80, 130, 220)))
                    .child(page("Page 3", Color::rgb(80, 180, 120))),
            ))
            .child(labeled(
                "Custom height",
                Carousel::new()
                    .height(100.0)
                    .child(page("A", Color::rgb(180, 100, 200)))
                    .child(page("B", Color::rgb(200, 160, 60))),
            ))
            .child(labeled(
                "No indicator",
                Carousel::new()
                    .no_indicator()
                    .child(page("X", Color::rgb(90, 90, 90)))
                    .child(page("Y", Color::rgb(60, 60, 60))),
            ))
            .child(labeled(
                "Custom indicator color",
                Carousel::new()
                    .indicator_color(Color::rgb(220, 80, 60))
                    .child(page("1", Color::rgb(40, 40, 60)))
                    .child(page("2", Color::rgb(50, 50, 80)))
                    .child(page("3", Color::rgb(60, 60, 100))),
            )),
    )
}
