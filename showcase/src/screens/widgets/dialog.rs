//! `Dialog` — a title/message/actions surface, presented as an overlay two
//! ways: co-located `.dialog()` (always modal), or `Dialog::emit` (honors
//! `.modal()`/`.non_modal()`/`.full_page()`).

use rosace::prelude::*;
use crate::present::PresentExt;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn dialog_detail(
    modal_open: &Atom<bool>, non_modal_open: &Atom<bool>, full_page_open: &Atom<bool>, styled_open: &Atom<bool>,
) -> impl Widget {
    let modal_trigger = modal_open.clone();
    let modal_trigger2 = modal_open.clone();
    let non_modal_trigger = non_modal_open.clone();
    let full_page_trigger = full_page_open.clone();
    let styled_trigger = styled_open.clone();
    // Non-modal and full-page are not reachable through `.dialog()`, which is
    // always modal — they go through `Dialog::emit`, which needs a paint
    // context. `present` supplies one.
    let (nm, fp) = (non_modal_open.clone(), full_page_open.clone());

    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Modal — co-located .dialog() (default presentation)",
                // `.dialog()` only DECLARES the overlay — it never opens it
                // itself (found live: the trigger absorbed clicks and did
                // nothing).
                Button::new("Open modal dialog")
                    .on_press(move || modal_trigger.set(true))
                    .dialog(modal_open.get(), move || {
                        let o = modal_trigger2.clone();
                        std::sync::Arc::new(
                            Dialog::new("Delete item?")
                                .message("This cannot be undone.")
                                .action("Cancel", { let o = o.clone(); move || o.set(false) })
                                .destructive_action("Delete", move || o.set(false)),
                        )
                    }),
            ))
            .child(labeled(
                "Non-modal — Dialog::emit, content behind stays interactive",
                Button::new("Open non-modal dialog").on_press(move || non_modal_trigger.set(true)),
            ))
            .child(labeled(
                "Full page — Dialog::emit, fills the window",
                Button::new("Open full-page dialog").on_press(move || full_page_trigger.set(true)),
            ))
            .child(labeled(
                "Custom width, radius, and colors",
                Button::new("Open styled dialog")
                    .on_press(move || styled_trigger.set(true))
                    .dialog(styled_open.get(), {
                        let o = styled_open.clone();
                        move || std::sync::Arc::new(
                            Dialog::new("Styled")
                                .width(280.0)
                                .radius(20.0)
                                .background(Color::rgb(30, 30, 40))
                                .color(Color::WHITE)
                                .primary_action("OK", { let o = o.clone(); move || o.set(false) }),
                        )
                    }),
            )),
    )
    .present(move |ctx| {
        let o = nm.clone();
        Dialog::new("Non-modal")
            .message("The content behind this stays interactive.")
            .non_modal()
            .action("Close", { let o = o.clone(); move || o.set(false) })
            .emit(ctx, nm.get(), move || o.set(false));

        let o = fp.clone();
        Dialog::new("Full page")
            .message("Fills the entire window, like a pushed page.")
            .full_page()
            .action("Close", { let o = o.clone(); move || o.set(false) })
            .emit(ctx, fp.get(), move || o.set(false));
    })
}
