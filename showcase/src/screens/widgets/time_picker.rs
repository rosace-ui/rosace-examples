//! `TimePicker` — a Material clock-dial time picker: hour/minute editing,
//! 12h/24h display, fully customizable colors.

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

pub fn time_picker_detail(
    default_time: &Atom<SimpleTime>,
    h24_time: &Atom<SimpleTime>,
    minute_time: &Atom<SimpleTime>,
    step_time: &Atom<SimpleTime>,
    styled_time: &Atom<SimpleTime>,
    unit: &Atom<TimeUnit>,
) -> impl Widget {
    let v1 = default_time.clone();
    let v2 = h24_time.clone();
    let v3 = minute_time.clone();
    let v4 = step_time.clone();
    let v5 = styled_time.clone();
    let u = unit.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Default (12h, editing the hour) — {:?}", default_time.get()),
                TimePicker::new(default_time.get()).on_change(move |t| v1.set(t)),
            ))
            .child(labeled(
                &format!("24h display — {:?}", h24_time.get()),
                TimePicker::new(h24_time.get()).use_24h().on_change(move |t| v2.set(t)),
            ))
            .child(labeled(
                &format!("Editing the minute — {:?}", minute_time.get()),
                TimePicker::new(minute_time.get()).editing(TimeUnit::Minute).on_change(move |t| v3.set(t)),
            ))
            .child(labeled(
                &format!("Custom minute step — {:?}", step_time.get()),
                TimePicker::new(step_time.get()).editing(TimeUnit::Minute).minute_step(5).on_change(move |t| v4.set(t)),
            ))
            .child(labeled(
                &format!("Custom colors — {:?}", styled_time.get()),
                TimePicker::new(styled_time.get())
                    .accent(Color::rgb(220, 80, 60))
                    .dial_color(Color::rgb(30, 30, 40))
                    .hand_color(Color::rgb(220, 80, 60))
                    .thumb_color(Color::rgb(220, 80, 60))
                    .number_color(Color::rgb(200, 200, 200))
                    .selected_number_color(Color::WHITE)
                    .on_change(move |t| v5.set(t)),
            ))
            .child(labeled(
                &format!("on_unit_change (hour \u{2194} minute) — editing {:?}", unit.get()),
                TimePicker::new(SimpleTime::new(9, 30)).editing(unit.get()).on_unit_change(move |u2| u.set(u2)),
            )),
    )
}
