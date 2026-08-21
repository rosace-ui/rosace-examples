//! Tap feedback shared by every widget detail screen.
//!
//! Every interactive element in the catalog reports what it did, so a demo
//! is verifiable by touch rather than by reading its source: press a button
//! and a toast says which button. That matters most on a phone, where there
//! is no console to print to and no cursor to hover with — before this, a
//! handler wired to `|| {}` and a handler that was genuinely broken looked
//! exactly the same on device.
//!
//! One toast is shared across the whole catalog rather than one per demo.
//! Two reasons: a screen with eight controls would otherwise need eight
//! `Atom<bool>`s that can never be visible at once, and overlapping toasts
//! from rapid taps stack into an unreadable pile. A single message atom
//! means the newest tap simply replaces the message.

use rosace::prelude::*;

/// How long a feedback toast stays up. Short — it is an acknowledgement,
/// not a notification, and a demo screen invites rapid tapping.
const DWELL_SECS: f32 = 1.6;

/// The shared toast channel. Cloned into every demo screen.
#[derive(Clone)]
pub struct Feedback {
    open: Atom<bool>,
    message: Atom<String>,
}

impl Feedback {
    pub fn new(open: Atom<bool>, message: Atom<String>) -> Self {
        Self { open, message }
    }

    /// Fire the toast directly — for handlers that already own a closure.
    pub fn say(&self, text: impl Into<String>) {
        self.message.set(text.into());
        Toast::show(&self.open, DWELL_SECS);
    }

    /// An `on_press`-shaped handler that reports `text`.
    ///
    /// ```rust,ignore
    /// Button::new("Save").on_press(fb.tap("Save pressed"))
    /// ```
    pub fn tap(&self, text: impl Into<String>) -> impl Fn() + Send + Sync + 'static {
        let (open, message, text) = (self.open.clone(), self.message.clone(), text.into());
        move || {
            message.set(text.clone());
            Toast::show(&open, DWELL_SECS);
        }
    }

    /// A handler for controls that report a VALUE (`Slider`, `Stepper`,
    /// `Switch`): `label` names the control, the value is appended.
    pub fn tap_with<T: std::fmt::Display>(
        &self,
        label: impl Into<String>,
    ) -> impl Fn(T) + Send + Sync + 'static {
        let (open, message, label) = (self.open.clone(), self.message.clone(), label.into());
        move |v: T| {
            message.set(format!("{label}: {v}"));
            Toast::show(&open, DWELL_SECS);
        }
    }

    /// Attach the toast overlay. Called ONCE, around the whole detail screen,
    /// so individual demos never have to think about it.
    pub fn attach(&self, child: impl Widget + 'static) -> impl Widget {
        let message = self.message.clone();
        child.toast(self.open.clone(), move || {
            std::sync::Arc::new(Toast::info(message.get()))
        })
    }
}
