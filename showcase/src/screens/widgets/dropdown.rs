//! `Dropdown` — a single selection from an opened list of options.

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

pub fn dropdown_detail(
    selected: &Atom<usize>, open: &Atom<bool>, open2: &Atom<bool>, open3: &Atom<bool>,
    styled_open: &Atom<bool>, styled_selected: &Atom<usize>,
) -> impl Widget {
    let s = selected.clone();
    let options = vec!["Small", "Medium", "Large"];
    let sel2 = styled_selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                &format!("Try it — selected: {}", options[selected.get().min(options.len() - 1)]),
                Dropdown::new(options.clone(), selected.get(), open.get())
                    .on_change(move |i| s.set(i)),
            ))
            .child(labeled(
                "Disabled",
                Dropdown::new(vec!["Only option"], 0, open2.get()).disabled(),
            ))
            .child(labeled(
                "Custom width + radius",
                Dropdown::new(vec!["A", "B", "C"], 0, open3.get()).width(220.0).radius(12.0),
            ))
            .child(labeled(
                &format!(
                    "Custom background, border, shape — selected: {}",
                    vec!["Alpha", "Beta", "Gamma"][styled_selected.get().min(2)]
                ),
                Dropdown::new(vec!["Alpha", "Beta", "Gamma"], styled_selected.get(), styled_open.get())
                    .background(Color::rgb(30, 30, 40))
                    .color(Color::WHITE)
                    .border(Color::rgb(220, 80, 60), 1.5)
                    .radius(20.0)
                    .on_change(move |i| sel2.set(i)),
            )),
    )
}
