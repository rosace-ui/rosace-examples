//! Calendar screen — a single-select `DatePicker`. Exercises the day-grid
//! month-slide animation (the heaviest picker animation) on mobile.

use rosace::prelude::*;
use rosace::widgets::{DatePicker, SimpleDate};

pub fn calendar_screen(month: &Atom<SimpleDate>, sel: &Atom<Option<SimpleDate>>) -> impl Widget {
    let (mc, sc) = (month.clone(), sel.clone());
    let mut dp = DatePicker::new(month.get()).today(SimpleDate::new(2026, 7, 29));
    if let Some(d) = sel.get() {
        dp = dp.selected(d);
    }
    let dp = dp
        .on_select(move |d, _| sc.set(Some(d)))
        .on_month_change(move |m| mc.set(m));

    Column::new()
        .spacing(16.0)
        .padding(EdgeInsets::all(20.0))
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .child(Text::new("Swipe/chevron months to see the slide animation").align(TextAlign::Center))
        .child(dp)
}
