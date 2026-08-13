//! `TextArea` — a multi-line editable text field with scrolling.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    Box::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn text_area_detail(value: &Atom<String>) -> impl Widget {
    let v = value.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — bound to app state",
                TextArea::new().value(value.get()).on_change(move |s| v.set(s)),
            ))
            .child(labeled("Placeholder", TextArea::new().placeholder("Write something\u{2026}")))
            .child(labeled("Focused on mount", TextArea::new().value("Starts focused").focused()))
            .child(labeled("Custom width and height", TextArea::new().width(260.0).height(80.0)))
            .child(labeled(
                "Custom colors",
                TextArea::new()
                    .value("Styled text area")
                    .background(Color::rgb(30, 30, 40))
                    .border(Color::rgb(90, 90, 100))
                    .focus_color(Color::rgb(220, 80, 60)),
            ))
            .child(labeled("No scrollbar", TextArea::new().value("No scrollbar shown here").no_scrollbar()))
            .child(labeled(
                "Custom scrollbar color",
                TextArea::new().value("Custom scrollbar color").scrollbar_color(Color::rgb(220, 80, 60)),
            )),
    )
}
