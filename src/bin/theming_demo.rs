//! Companion demo for Phase 23 Steps 1-3 (D105) — proves the SAME `AppBar`
//! renders platform-appropriate chrome purely from theme data, with zero
//! platform branch inside the widget. Exercises the real end-to-end path:
//! a `Themes` bundle (Step 2) resolved against a forced platform override
//! (Step 1), landing on `ThemeData::app_bar` (Step 3) that `AppBar` reads.
//!
//! ```bash
//! cargo run --bin theming_demo               # desktop -> falls back to light_theme()
//! RSC_PREVIEW=ios cargo run --bin theming_demo       # forced Platform::Ios -> cupertino()
//! RSC_PREVIEW=android cargo run --bin theming_demo   # forced Platform::Android -> material()
//! ```

use rosace::prelude::*;

struct ThemingDemo;

impl Component for ThemingDemo {
    fn build(&self, _ctx: &mut Context) -> Element {
        Scaffold::new(
            Column::new()
                .padding(EdgeInsets::all(24.0))
                .child(Text::new(
                    "Same AppBar widget, zero platform branch inside it — \
                     the theme alone decides title alignment, height, and \
                     whether the separating edge is drawn.",
                )),
        )
        .app_bar(
            AppBar::new("Phase 23 Proof")
                .leading(Button::new("Back").variant(ButtonVariant::Ghost).width(70.0))
                .action(Button::new("Edit").variant(ButtonVariant::Ghost).width(60.0)),
        )
        .into_element()
    }
}

fn main() {
    let preview = std::env::var("RSC_PREVIEW").unwrap_or_default();
    // One design system (D133): `cupertino()` was withdrawn — it only ever
    // restyled the accent and the app bar, which is half a platform look
    // rather than a real one. iOS now uses the base theme; Android keeps
    // Material's structural bar. Third-party skins plug in through this
    // same `Themes` bundle.
    let themes = Themes::new(light_theme())
        .platform(Platform::Android, material());

    let mut app = App::new().title("Theming Demo").size(420, 300).themes(themes);
    app = match preview.as_str() {
        "ios" => app.platform(Platform::Ios),
        "android" => app.platform(Platform::Android),
        _ => app, // no override -> real detected platform (macOS in this environment) -> falls back to light_theme()
    };
    app.launch(ThemingDemo);
}
