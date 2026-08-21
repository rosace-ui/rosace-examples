//! `Tabs` — an interactive tab bar over switchable content. Selection is
//! external (`usize` + `on_change`), matching `SegmentedControl`.

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

pub fn tabs_detail(selected: &Atom<usize>) -> impl Widget {
    let s = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it",
                Container::new().height(140.0).clip().child(
                    Tabs::new(selected.get(), move |i| s.set(i))
                        .tab("One", Container::new().align(Alignment::Center).child(Text::new("Content one")))
                        .tab("Two", Container::new().align(Alignment::Center).child(Text::new("Content two")))
                        .tab("Three", Container::new().align(Alignment::Center).child(Text::new("Content three"))),
                ),
            ))
            .child(labeled(
                "Readonly (no on_change — bar still absorbs taps)",
                Container::new().height(100.0).clip().child(
                    Tabs::readonly(0)
                        .tab("A", Text::new("A"))
                        .tab("B", Text::new("B")),
                ),
            ))
            .child(labeled(
                "Custom bar height, colors, divider, font size, no animation",
                Container::new().height(120.0).clip().child(
                    Tabs::readonly(1)
                        .tab("Alpha", Text::new("Alpha content"))
                        .tab("Beta", Text::new("Beta content"))
                        .bar_height(48.0)
                        .background(Color::rgb(30, 30, 40))
                        .active_color(Color::rgb(220, 80, 60))
                        .inactive_color(Color::rgb(150, 150, 150))
                        .indicator_color(Color::rgb(220, 80, 60))
                        .border_color(Color::rgb(70, 70, 80))
                        .font_size(15.0)
                        .animated(false),
                ),
            ))
            .child(labeled(
                "Scrollable — natural-width tabs in a horizontal ScrollView",
                Container::new().height(140.0).clip().child(
                    Tabs::readonly(2)
                        .scrollable(true)
                        .tab("Overview", Container::new().align(Alignment::Center).child(Text::new("Overview content")))
                        .tab("Detailed Analytics", Container::new().align(Alignment::Center).child(Text::new("Analytics content")))
                        .tab("Settings", Container::new().align(Alignment::Center).child(Text::new("Settings content")))
                        .tab("Notifications", Container::new().align(Alignment::Center).child(Text::new("Notifications content")))
                        .tab("Advanced Configuration", Container::new().align(Alignment::Center).child(Text::new("Advanced content")))
                        .tab("Help", Container::new().align(Alignment::Center).child(Text::new("Help content"))),
                ),
            )),
    )
}
