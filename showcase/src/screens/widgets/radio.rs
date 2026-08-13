//! `Radio` — a grouped choice where only one option is selected at a time.

use rosace::prelude::*;

use crate::feedback::Feedback;

pub fn radio_detail(selected: &Atom<u8>, fb: &Feedback) -> impl Widget {
    let mut group = Column::new()
        .spacing(6.0)
        .cross_axis_alignment(CrossAxisAlignment::Start);

    for (i, label) in ["Option A", "Option B", "Option C"].into_iter().enumerate() {
        let i = i as u8;
        let s = selected.clone();
        group = group.child(
            Radio::new(selected.get() == i)
                .label(label)
                .on_select(move || s.set(i)),
        );
    }

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new("Try it — a real group, one selection").color(Color::rgb(120, 120, 120)))
            .child(group)
            .child(Text::new("Disabled").color(Color::rgb(120, 120, 120)))
            .child(
                Column::new()
                    .spacing(6.0)
                    .cross_axis_alignment(CrossAxisAlignment::Start)
                    .child(Radio::new(true).label("Locked, selected").disabled().on_select(fb.tap("This should never appear — the radio is disabled")))
                    .child(Radio::new(false).label("Locked, unselected").disabled().on_select(fb.tap("This should never appear — the radio is disabled"))),
            )
            .child(Text::new("Custom color").color(Color::rgb(120, 120, 120)))
            .child(Radio::new(true).label("Custom color").color(Color::rgb(220, 80, 60)).on_select(fb.tap("Custom-colour radio selected"))),
    )
}
