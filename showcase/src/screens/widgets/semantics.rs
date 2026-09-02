//! `Semantics` — the accessibility escape hatch. Annotate a subtree that
//! cannot describe itself, or hide one that should not be announced at all.
//!
//! Nothing on this page looks different. That IS the point: `Semantics`
//! changes only what a screen reader reports. Turn on VoiceOver (macOS/iOS)
//! or TalkBack (Android) and move through the examples to hear the effect.

use rosace::prelude::*;

use crate::feedback::Feedback;

fn labeled(title: &str, note: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child)
            .child(Text::new(note.to_string()).size(13.0).color(Color::rgb(140, 140, 140))),
    )
}

pub fn semantics_detail(fb: &Feedback) -> impl Widget {
    let f = fb.clone();

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(22.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Announcing something with no widget",
                "Nothing on screen changes. With VoiceOver on you hear the message; \
                 without it, nothing happens at all.",
                Row::new()
                    .spacing(8.0)
                    .child(Button::new("Announce (polite)").on_press(|| {
                        rosace::a11y::announce(
                            "Copied to clipboard",
                            rosace::a11y::Politeness::Polite,
                        )
                    }))
                    .child(Button::new("Announce (assertive)").on_press(|| {
                        rosace::a11y::announce(
                            "Upload failed",
                            rosace::a11y::Politeness::Assertive,
                        )
                    })),
            ))
            .child(labeled(
                "Naming custom-painted content",
                "Announced as \"Sales trend, image\" instead of being skipped entirely.",
                Semantics::new(
                    CustomPaint::new(|cx, size| {
                        // A sparkline: meaningful to look at, invisible to
                        // assistive tech without an explicit label.
                        let pts = [0.35_f32, 0.6, 0.45, 0.8, 0.55, 0.9];
                        let step = size.width / (pts.len() as f32 - 1.0);
                        let (ox, oy) = (cx.rect.origin.x, cx.rect.origin.y);
                        for (i, v) in pts.iter().enumerate() {
                            let x = ox + step * i as f32;
                            let y = oy + size.height * (1.0 - v);
                            cx.fill_circle(Point { x, y }, 4.0, Color::rgb(120, 190, 255));
                        }
                    })
                    .height(80.0),
                )
                .role(Role::Image)
                .label("Sales trend"),
            ))
            .child(labeled(
                "Marking a heading",
                "Screen readers can jump between headings; a styled Text is not one.",
                Semantics::new(Text::new("Quarterly results").size(22.0))
                    .role(Role::Heading)
                    .heading_level(2),
            ))
            .child(labeled(
                "merge() — one announcement instead of three",
                "Without it a reader stutters through icon, title and count separately.",
                Semantics::new(
                    Row::new()
                        .spacing(8.0)
                        .child(Icon::new(IconKind::Star))
                        .child(Text::new("Favourites"))
                        .child(Badge::label("12")),
                )
                .merge()
                .label("Favourites, 12 items"),
            ))
            .child(labeled(
                "exclude() — hidden from assistive tech",
                "Purely decorative, and announcing it would only add noise.",
                Semantics::new(
                    Row::new()
                        .spacing(6.0)
                        .child(Icon::new(IconKind::Star))
                        .child(Icon::new(IconKind::Star))
                        .child(Icon::new(IconKind::Star)),
                )
                .exclude(),
            ))
            .child(labeled(
                "A control that needs a better name",
                "The visible label is \"×\"; assistive tech hears \"Close dialog\".",
                Semantics::new(Button::new("×").on_press(f.tap("Close pressed")))
                    .role(Role::Button)
                    .label("Close dialog"),
            )),
    )
}
