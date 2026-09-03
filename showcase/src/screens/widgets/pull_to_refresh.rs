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
        // widget never flips it. That is the whole contract, and it is why
        // this demo has to do real work to show anything: it sets the flag
        // TRUE here, and clears it when the "request" finishes.
        //
        // Previously it only ever set the flag false, so `refreshing` was
        // false on every frame and the spinner could not appear at all — the
        // widget looked broken when the demo was.
        .on_refresh(move || {
            if b.get() {
                return; // already refreshing — ignore a second pull
            }
            b.set(true);
            f.say("Refreshing…");

            // A real refresh takes time. Standing in for a network call with
            // a background thread, which is also the honest demonstration
            // that an `Atom` written from ANOTHER thread reaches the UI: the
            // write is routed to the subscriber's thread rather than dirtying
            // the writer's own.
            let (c, b, f) = (c.clone(), b.clone(), f.clone());
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(1200));
                c.set(c.get() + 1);
                b.set(false);
                f.say("Refreshed");
            });
        })
        .color(Color::rgb(120, 90, 220))
}
