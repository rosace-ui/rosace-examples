//! App theme. Edit these to customize colors, or build a `ThemeData` from
//! scratch — the built-ins are just a convenient starting point.

use rosace::prelude::ThemeData;

/// The app's dark theme.
pub fn dark() -> ThemeData {
    rosace::prelude::dark_theme()
}

/// The app's light theme.
pub fn light() -> ThemeData {
    rosace::prelude::light_theme()
}

/// One design system, not per-platform chrome (D133, superseding D105's
/// Cupertino half): Android keeps Material's structural bar, everything
/// else uses the base theme. Third-party themes plug in through this same
/// `Themes` bundle. Passed to `App::themes(..)` in `lib.rs`.
pub fn themes() -> rosace::prelude::Themes {
    rosace::prelude::Themes::new(light())
        .platform(rosace::prelude::Platform::Android, rosace::prelude::material())
}
