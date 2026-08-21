//! `DatePicker` — a month-grid calendar: single-date or range selection,
//! min/max bounds, horizontal or vertical month transitions.

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

pub fn date_picker_detail(
    single: &Atom<SimpleDate>,
    range: &Atom<(SimpleDate, Option<SimpleDate>)>,
    bounded: &Atom<SimpleDate>,
    accented: &Atom<SimpleDate>,
    vertical: &Atom<SimpleDate>,
    month: &Atom<SimpleDate>,
) -> impl Widget {
    let today = SimpleDate::new(2026, 8, 1);
    let s = single.clone();
    let rg = range.clone();
    let b = bounded.clone();
    let a = accented.clone();
    let v = vertical.clone();
    let m = month.clone();
    let (range_start, range_end) = range.get();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Single-date selection — {:?}", single.get()),
                DatePicker::new(today).selected(single.get()).today(today).on_select(move |d, _| s.set(d)),
            ))
            .child(labeled(
                &format!("Range selection — {:?}", range.get()),
                DatePicker::new(today)
                    .mode(SelectionMode::Range)
                    .range(range_start, range_end)
                    .range_color(Color::rgb(220, 80, 60))
                    .on_select(move |start, end| rg.set((start, end))),
            ))
            .child(labeled(
                &format!("Min/max bounds (outside dates dim and stop absorbing) — {:?}", bounded.get()),
                DatePicker::new(today)
                    .min_date(SimpleDate::new(2026, 8, 3))
                    .max_date(SimpleDate::new(2026, 8, 20))
                    .selected(bounded.get())
                    .on_select(move |d, _| b.set(d)),
            ))
            .child(labeled(
                &format!("Custom accent color — {:?}", accented.get()),
                DatePicker::new(today).selected(accented.get()).accent(Color::rgb(90, 40, 160)).on_select(move |d, _| a.set(d)),
            ))
            .child(labeled(
                &format!("Vertical month transitions — {:?}", vertical.get()),
                DatePicker::new(today).axis(PageAxis::Vertical).selected(vertical.get()).on_select(move |d, _| v.set(d)),
            ))
            .child(labeled(
                &format!("on_month_change callback — visible month {:?}", month.get()),
                DatePicker::new(month.get()).on_month_change(move |d| m.set(d)),
            )),
    )
}
