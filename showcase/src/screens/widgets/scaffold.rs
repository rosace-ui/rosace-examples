//! `Scaffold` — full-page layout: optional AppBar + optional NavRail
//! sidebar + body + optional FAB + optional bottom bar + optional right
//! sidebar. This IS the root widget of this very app's screens — shown
//! here nested (with a fixed height) so it doesn't take over the page.

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

fn body(label: &str) -> BoxedWidget {
    std::sync::Arc::new(Container::new().align(Alignment::Center).child(Text::new(label)))
}

pub fn scaffold_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "App bar + body",
                Container::new().height(180.0).clip().radius(8.0).child(
                    Scaffold::new(body("Body content")).app_bar(AppBar::new("Title")),
                ),
            ))
            .child(labeled(
                "Nav rail + body",
                Container::new().height(180.0).clip().radius(8.0).child(
                    Scaffold::new(body("Body content"))
                        .nav_rail(NavRail::new().item(NavItem::new("Home").active()).item(NavItem::new("Settings"))),
                ),
            ))
            .child(labeled(
                "FAB + bottom bar",
                Container::new().height(180.0).clip().radius(8.0).child(
                    Scaffold::new(body("Body content"))
                        .fab(FloatingActionButton::new())
                        .bottom_bar(
                            BottomNavigationBar::new()
                                .item(BottomNavItem::new("Home").active())
                                .item(BottomNavItem::new("Search")),
                        ),
                ),
            ))
            .child(labeled(
                "Custom background + right sidebar",
                Container::new().height(180.0).clip().radius(8.0).child(
                    Scaffold::new(body("Body content"))
                        .background(Color::rgb(30, 30, 40))
                        .sidebar_right(Container::new().width(120.0).background(Color::rgb(20, 20, 30))),
                ),
            )),
    )
}
