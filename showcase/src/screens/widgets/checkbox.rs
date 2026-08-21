//! `Checkbox` — every state in one place: try one live, then see checked,
//! unchecked, indeterminate, and disabled side by side.

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

pub fn checkbox_detail(checked: &Atom<bool>) -> impl Widget {
    let c = checked.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — tap to toggle",
                Checkbox::new(checked.get())
                    .label("Interactive")
                    .on_change(move |v| c.set(v)),
            ))
            .child(labeled("Checked", Checkbox::new(true).label("Checked").on_change(|_| {})))
            .child(labeled("Unchecked", Checkbox::new(false).label("Unchecked").on_change(|_| {})))
            .child(labeled(
                "Indeterminate",
                Checkbox::new(false).label("Indeterminate").indeterminate().on_change(|_| {}),
            ))
            .child(labeled(
                "Disabled",
                Checkbox::new(true).label("Disabled, checked").disabled().on_change(|_| {}),
            ))
            .child(labeled(
                "Custom color",
                Checkbox::new(true).label("Custom color").color(Color::rgb(220, 80, 60)).on_change(|_| {}),
            )),
    )
}
