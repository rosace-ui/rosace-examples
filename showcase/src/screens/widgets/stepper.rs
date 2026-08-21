//! `Stepper` — the numeric −/+ control: `[−] value [+]`.

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

pub fn stepper_detail(
    value: &Atom<i64>,
    bounded: &Atom<i64>,
    step: &Atom<i64>,
    sized: &Atom<i64>,
    styled: &Atom<i64>,
) -> impl Widget {
    let v = value.clone();
    let b = bounded.clone();
    let s = step.clone();
    let sz = sized.clone();
    let st = styled.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — {}", value.get()),
                Stepper::new(value.get()).on_change(move |x| v.set(x)),
            ))
            .child(labeled(
                &format!("Bounded range (0\u{2013}10) — {}", bounded.get()),
                Stepper::new(bounded.get()).min(0).max(10).on_change(move |x| b.set(x)),
            ))
            .child(labeled(
                &format!("Custom step (by 5) — {}", step.get()),
                Stepper::new(step.get()).step(5).on_change(move |x| s.set(x)),
            ))
            .child(labeled(
                &format!("Custom height and font size — {}", sized.get()),
                Stepper::new(sized.get()).height(40.0).font_size(16.0).on_change(move |x| sz.set(x)),
            ))
            .child(labeled(
                &format!("Custom colors and border — {}", styled.get()),
                Stepper::new(styled.get())
                    .background(Color::rgb(30, 30, 40))
                    .color(Color::WHITE)
                    .border(Color::rgb(220, 80, 60), 1.5)
                    .radius(10.0)
                    .on_change(move |x| st.set(x)),
            )),
    )
}
