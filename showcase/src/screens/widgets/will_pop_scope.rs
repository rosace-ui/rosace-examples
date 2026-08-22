//! `WillPopScope` — stop a screen leaving while there is unsaved work.
//!
//! This page is itself pushed on the nav stack, so the guard below governs
//! the REAL exits from it. Try all three: the `‹ Back` button in the app
//! bar, the Android back button or gesture, and the iOS left-edge swipe.
//! Every one asks first, because the guard sits inside `ScreenNav::pop`
//! rather than in any one control's handler.

use rosace::prelude::*;
use crate::present::BindOverlay;

use crate::feedback::Feedback;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn will_pop_scope_detail(
    draft: &Atom<String>,
    saved: &Atom<String>,
    confirm_open: &Atom<bool>,
    nav: &ScreenNav<crate::app::Screen>,
    fb: &Feedback,
) -> impl Widget {
    // "Unsaved work" is just the draft differing from what was last saved —
    // the same shape a real editor uses, and it means the guard needs no
    // extra bookkeeping of its own.
    let dirty = draft.get() != saved.get();

    let body = Column::new()
        .padding(EdgeInsets::all(16.0))
        .spacing(20.0)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .child(Text::new(
            "Type something, then try to leave — with the app bar's Back \
             button, the Android back gesture, or the iOS edge swipe. All \
             three are stopped by the same guard.",
        ))
        .child(labeled(
            "Draft",
            TextInput::new()
                .placeholder("Unsaved work goes here")
                .value(draft.get())
                .on_change({
                    let d = draft.clone();
                    move |v| d.set(v)
                }),
        ))
        .child(
            Row::new()
                .spacing(8.0)
                .child(
                    Button::new(if dirty { "Save" } else { "Saved" })
                        .disabled_if(!dirty)
                        .on_press({
                            let (d, s, f) = (draft.clone(), saved.clone(), fb.clone());
                            move || {
                                s.set(d.get());
                                f.say("Saved — leaving is allowed now");
                            }
                        }),
                )
                .child(
                    Button::new("Revert")
                        .variant(ButtonVariant::Ghost)
                        .disabled_if(!dirty)
                        .on_press({
                            let (d, s) = (draft.clone(), saved.clone());
                            move || d.set(s.get())
                        }),
                ),
        )
        .child(Text::new(if dirty {
            "Unsaved changes — leaving will ask first."
        } else {
            "No unsaved changes — leaving is allowed."
        }))
        .child(Divider::new())
        .child(Text::new(
            "The guard lives inside ScreenNav::pop, not in the back handler, \
             so no control can bypass it. A blocked pop still CONSUMES the \
             Android back intent — otherwise the activity would finish \
             underneath this dialog.",
        ).size(13.0).color(Color::rgb(140, 140, 140)));

    // The confirmation the guard opens. "Discard" clears the dirty state and
    // pops — the same `nav.pop()` the guard just blocked, which now passes
    // because the state it reads has changed. That is why there is no
    // force-pop API.
    let dialog_open = confirm_open.clone();
    let discard = {
        let (d, s, n, c, f) = (draft.clone(), saved.clone(), nav.clone(),
                               confirm_open.clone(), fb.clone());
        move || {
            d.set(s.get());   // throw the edits away
            c.set(false);
            f.say("Discarded");
            n.pop();          // allowed now — the guard reads `dirty`
        }
    };
    let keep = {
        let c = confirm_open.clone();
        move || c.set(false)
    };

    WillPopScope::new(
        ScrollView::new(body).dialog_bound(&dialog_open, move || {
            std::sync::Arc::new(
                Dialog::new("Discard changes?")
                    .message("Your draft has not been saved. Leaving now will lose it.")
                    .action("Keep editing", keep.clone())
                    .action("Discard", discard.clone()),
            )
        }),
    )
    .on_will_pop({
        let (draft, saved, confirm) = (draft.clone(), saved.clone(), confirm_open.clone());
        move || {
            if draft.get() == saved.get() {
                return true; // nothing to lose
            }
            confirm.set(true); // ask, and stop this pop
            false
        }
    })
}
