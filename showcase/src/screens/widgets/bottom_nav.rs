//! `BottomNavigationBar` — the horizontal counterpart to `NavRail`: 3-5
//! top-level destinations pinned to the bottom edge. Drop it in
//! `Scaffold::bottom_bar`.

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

/// `BottomNavItem` has no `.active_if()` (unlike `NavItem`) — conditionally
/// apply `.active()` here instead.
fn nav_item(label: &str, active: bool) -> BottomNavItem {
    let item = BottomNavItem::new(label);
    if active { item.active() } else { item }
}

pub fn bottom_nav_detail(selected: &Atom<usize>) -> impl Widget {
    let s0 = selected.clone();
    let s1 = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Try it",
                BottomNavigationBar::new()
                    .item(nav_item("Home", selected.get() == 0).on_press(move || s0.set(0)))
                    .item(nav_item("Search", selected.get() == 1).on_press(move || s1.set(1)))
                    .item(BottomNavItem::new("Inbox").badge(3)),
            ))
            .child(labeled(
                "With leading icons",
                BottomNavigationBar::new()
                    .item(BottomNavItem::new("Home").icon(Icon::new(IconKind::Home)).active())
                    .item(BottomNavItem::new("Settings").icon(Icon::new(IconKind::Settings))),
            ))
            .child(labeled(
                "Custom height, radius, no divider",
                BottomNavigationBar::new()
                    .height(64.0)
                    .radius(16.0)
                    .no_divider()
                    .item(BottomNavItem::new("A").active())
                    .item(BottomNavItem::new("B")),
            ))
            .child(labeled(
                "Custom colors and font size",
                BottomNavigationBar::new()
                    .background(Color::rgb(30, 30, 40))
                    .active_color(Color::rgb(220, 80, 60))
                    .inactive_color(Color::rgb(150, 150, 150))
                    .font_size(12.0)
                    .item(BottomNavItem::new("One").active())
                    .item(BottomNavItem::new("Two")),
            )),
    )
}
