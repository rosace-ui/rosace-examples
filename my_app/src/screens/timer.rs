//! Timer screen — the Material clock-dial `TimePicker`. Exercises the
//! sweeping-hand animation (continuous per-frame) on mobile.

use rosace::prelude::*;
use rosace::widgets::{SimpleTime, TimePicker, TimeUnit};

pub fn timer_screen(time: &Atom<SimpleTime>, unit: &Atom<TimeUnit>) -> impl Widget {
    let (tc, uc) = (time.clone(), unit.clone());
    let clock = TimePicker::new(time.get())
        .editing(unit.get())
        .on_change(move |v| tc.set(v))
        .on_unit_change(move |u| uc.set(u));

    let (h12, pm) = time.get().hour_12();
    let label = format!("{h12:02}:{:02} {}", time.get().minute, if pm { "PM" } else { "AM" });

    Column::new()
        .spacing(16.0)
        .padding(EdgeInsets::all(20.0))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .child(Text::display(label).align(TextAlign::Center))
        .child(Text::new("Tap the header to switch hour/minute; drag the hand").align(TextAlign::Center))
        .child(clock)
}
