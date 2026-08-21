//! `Dismissible` — swipe a row away. Drag past the threshold and it commits;
//! release short of it and it springs back.

use rosace::prelude::*;

use crate::feedback::Feedback;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn dismissible_detail(removed: &Atom<i32>, fb: &Feedback) -> impl Widget {
    let mut rows = Column::new().spacing(2.0);
    for i in 1..=4 {
        let (f, r) = (fb.clone(), removed.clone());
        rows = rows.child(
            Dismissible::new(
                ListTile::new(format!("Message {i}"))
                    .subtitle("Swipe left or right to dismiss"),
            )
            .semantic_label(format!("Message {i}"))
            .on_dismissed(move || {
                r.set(r.get() + 1);
                f.say(format!("Message {i} dismissed"));
            }),
        );
    }

    let (f2, f3) = (fb.clone(), fb.clone());

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled("Swipe either way", rows))
            .child(Text::new(format!("Dismissed so far: {}", removed.get())))
            .child(labeled(
                "Custom background — shown as the row slides away",
                Dismissible::new(ListTile::new("Archive me").subtitle("Amber background"))
                    .background(Container::new().background(Color::rgb(200, 140, 40)))
                    .on_dismissed(move || f2.say("Archived")),
            ))
            .child(labeled(
                "threshold(0.6) — needs a longer swipe to commit",
                Dismissible::new(
                    ListTile::new("Stubborn row").subtitle("Short swipes spring back"),
                )
                .threshold(0.6)
                .on_dismissed(move || f3.say("Stubborn row finally dismissed")),
            )),
    )
}
