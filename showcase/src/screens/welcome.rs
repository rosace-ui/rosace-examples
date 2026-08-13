//! The welcome screen — a full-bleed, animated intro. Plays once on first
//! build (the animation's own progress atom persists across re-renders, so
//! navigating away and back would resume rather than replay — not a concern
//! here since there's no way back to Welcome once you've left it).

use rosace::animate::{AnimCtrl, Progress};
use rosace::prelude::*;

use crate::app::Screen;

pub fn welcome_screen(progress: &Progress, ctrl: &AnimCtrl, nav: &ScreenNav<Screen>) -> impl Widget {
    if progress.get() == 0.0 {
        ctrl.play();
    }
    let p = progress.get();

    // A simple staggered reveal: the title fades in over the first 60% of
    // the animation, the subtitle over the back half, and the button only
    // once both are fully visible — no separate widget or timer needed,
    // just three windows carved out of the one Progress value.
    let title_alpha = (p.min(0.6) / 0.6 * 255.0) as u8;
    let subtitle_p = ((p - 0.3) / 0.7).clamp(0.0, 1.0);
    let subtitle_alpha = (subtitle_p * 255.0) as u8;
    let button_visible = p >= 0.999;

    // Theme-derived, not hardcoded (found live, 2026-08-03): a literal
    // near-black `Color::rgba(30, 30, 30, ..)` reads fine on the light
    // theme this screen originally shipped against, but is nearly
    // invisible on the dark theme — a pre-existing bug that stayed hidden
    // until system-brightness-following (D127) started actually landing
    // apps on the dark theme by default instead of always launching light.
    // `rosace_theme::Color` is f32 [0,1] (design tokens); widgets paint
    // with `rosace_render::Color`, u8 [0,255] — convert once here.
    let on_bg = rosace::theme::use_theme().colors.on_background;
    let title_color = Color::rgb(
        (on_bg.r * 255.0).round() as u8,
        (on_bg.g * 255.0).round() as u8,
        (on_bg.b * 255.0).round() as u8,
    );
    // Muted caption gray — the same fixed tone every other widget-demo
    // screen in this app already uses for secondary text.
    let subtitle_color = Color::rgb(120, 120, 120);

    let nav = nav.clone();
    let mut column = Column::new()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .padding(EdgeInsets::all(32.0))
        .spacing(16.0)
        .child(
            Text::display("Welcome to ROSACE")
                .align(TextAlign::Center)
                .color(Color { a: title_alpha, ..title_color }),
        )
        .child(
            Text::new("A tour of what you can build — widgets, platform channels, and more.")
                .align(TextAlign::Center)
                .color(Color { a: subtitle_alpha, ..subtitle_color }),
        )
        .child(Spacer::gap(0.0, 32.0));

    if button_visible {
        column = column.child(Button::new("Get Started").on_press(move || nav.push(Screen::Home)));
    }

    column
}
