//! The showcase's real screens have URL paths.
//!
//! A routing API that nothing uses is how the last one ended up half-built —
//! `#[routes]` was locked in D026 and only the enum half shipped. This drives
//! the macro against an app with every awkward shape in it: unit variants, a
//! `usize` payload, and an enum-typed parameter with fifty-odd values whose
//! slug is derived from its display name rather than listed twice.

use rosace::nav::RoutePath;
use showcase::{Screen, WidgetKind};

#[test]
fn every_screen_survives_a_round_trip() {
    let mut all = vec![
        Screen::Welcome,
        Screen::Home,
        Screen::Widgets,
        Screen::PlatformChannel,
        Screen::HeroDetail(2),
        Screen::HeroFar(1),
    ];
    all.extend(WidgetKind::ALL.iter().copied().map(Screen::WidgetDetail));

    for screen in all {
        let path = screen.to_path();
        assert_eq!(
            Screen::from_path(&path),
            Some(screen),
            "{screen:?} formatted to {path:?} and did not parse back"
        );
    }
}

#[test]
fn paths_are_the_ones_a_person_would_type() {
    assert_eq!(Screen::Home.to_path(), "/");
    assert_eq!(Screen::Widgets.to_path(), "/widgets");
    assert_eq!(Screen::WidgetDetail(WidgetKind::Slider).to_path(), "/widget/slider");
    assert_eq!(Screen::HeroFar(1).to_path(), "/hero-far/1");
}

/// The slug is derived from `name()`, so a multi-word widget has to come out
/// hyphenated and go back in again.
#[test]
fn multi_word_widgets_slug_and_unslug() {
    let path = Screen::WidgetDetail(WidgetKind::TextInput).to_path();
    assert_eq!(path, "/widget/text-input");
    assert_eq!(Screen::from_path(&path), Some(Screen::WidgetDetail(WidgetKind::TextInput)));
}

/// Two widgets must never share a slug, or one of them is unreachable by
/// link and the other silently answers for it.
#[test]
fn every_widget_slug_is_unique() {
    let mut seen = std::collections::HashMap::new();
    for k in WidgetKind::ALL {
        if let Some(prev) = seen.insert(k.slug(), *k) {
            panic!("`{prev:?}` and `{k:?}` both slug to {:?}", k.slug());
        }
    }
    assert_eq!(seen.len(), WidgetKind::ALL.len());
}

#[test]
fn a_link_to_a_widget_that_does_not_exist_is_refused() {
    assert_eq!(Screen::from_path("/widget/not-a-widget"), None);
    assert_eq!(Screen::from_path("/hero/abc"), None, "the index is a usize");
    assert_eq!(Screen::from_path("/nowhere"), None);
}
