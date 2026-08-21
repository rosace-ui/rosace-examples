//! `NavRail` — a vertical navigation sidebar: section headers, items
//! (leading icon, badge, active state), separators, and custom widgets.

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

pub fn nav_rail_detail(selected: &Atom<usize>) -> impl Widget {
    let s1 = selected.clone();
    let s2 = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it — sections, items, badges, separator",
                Container::new().height(260.0).clip().child(
                    NavRail::new()
                        .section("Main")
                        .item(NavItem::new("Home").active_if(selected.get() == 0).on_press({
                            let s = s1.clone();
                            move || s.set(0)
                        }))
                        .item(
                            NavItem::new("Inbox")
                                .badge(4)
                                .active_if(selected.get() == 1)
                                .on_press(move || s1.set(1)),
                        )
                        .separator()
                        .section("Other")
                        .item(NavItem::new("Settings").active_if(selected.get() == 2).on_press(move || s2.set(2))),
                ),
            ))
            .child(labeled(
                "Custom width and background",
                Container::new().height(140.0).clip().child(
                    NavRail::new()
                        .width(160.0)
                        .background(Color::rgb(20, 20, 30))
                        .item(NavItem::new("A").active())
                        .item(NavItem::new("B")),
                ),
            ))
            .child(labeled(
                "Item with a leading widget and custom height",
                Container::new().height(100.0).clip().child(
                    NavRail::new().item(
                        NavItem::new("Profile")
                            .leading(Avatar::new("GJ").size(20.0))
                            .height(44.0),
                    ),
                ),
            )),
    )
}
