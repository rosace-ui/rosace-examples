//! `PullToRefresh` — drag down past the trigger distance and release to fire
//! `on_refresh`; the arc fills as you pull, then spins while `refreshing`.

use rosace::prelude::*;

use crate::feedback::Feedback;

pub fn pull_to_refresh_detail(
    count: &Atom<i32>,
    busy: &Atom<bool>,
    fb: &Feedback,
) -> impl Widget {
    let (c, b, f) = (count.clone(), busy.clone(), fb.clone());
    let n = count.get();

    let mut list = Column::new()
        .padding(EdgeInsets::all(16.0))
        .spacing(12.0)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .child(Text::new("Pull down from the top").color(Color::rgb(120, 120, 120)))
        .child(Text::new(format!("Refreshed {n} times")));

    for i in 0..12 {
        list = list.child(
            Card::new(Text::new(format!("Item {} — batch {}", i + 1, n))),
        );
    }

    PullToRefresh::new(ScrollView::new(list))
        .refreshing(busy.get())
        // The spinner is driven by `refreshing`, which the APP owns — the
        // widget never flips it. A real app clears it when its network call
        // returns; here the next frame clears it, so the spinner blinks
        // rather than hanging forever.
        .on_refresh(move || {
            c.set(c.get() + 1);
            b.set(false);
            f.say("Refreshed");
        })
        .color(Color::rgb(120, 90, 220))
}
