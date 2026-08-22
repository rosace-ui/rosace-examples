//! Presenting an overlay that needs a `PaintCtx`.
//!
//! `Dialog::emit` and `Drawer::emit` promote into the root layer, and a
//! promotion is declared DURING the paint walk — so they need a `PaintCtx`,
//! which `Component::build` has not got.
//!
//! For most overlays that is a non-issue: the co-located `.dialog(open, ..)` /
//! `.sheet(..)` / `.toast(..)` builders declare the overlay on the tree and the
//! framework promotes it during paint. But those are always MODAL, so a
//! non-modal or full-page `Dialog`, and any `Drawer`, still need somewhere to
//! run paint-time code.
//!
//! [`Present`] is that somewhere: a transparent wrapper that paints its child
//! and then hands its own `PaintCtx` to a closure.

use std::sync::Arc;

use rosace::widgets::tree::{Children, LayoutCtx, PaintCtx};
use rosace::{Size, Widget};

/// Wraps a widget and runs `emit` with the live [`PaintCtx`] after painting it.
pub struct Present<W: Widget + 'static> {
    child: W,
    emit: Arc<dyn Fn(&mut PaintCtx) + Send + Sync>,
}

impl<W: Widget + 'static> Present<W> {
    pub fn new(child: W, emit: impl Fn(&mut PaintCtx) + Send + Sync + 'static) -> Self {
        Self { child, emit: Arc::new(emit) }
    }
}

impl<W: Widget + Send + Sync + 'static> Widget for Present<W> {
    fn children(&self) -> Children<'_> {
        Children::One(&self.child)
    }

    fn layout(&self, ctx: &LayoutCtx) -> Size {
        ctx.layout_child(ctx.constraints, &self.child)
    }

    fn paint(&self, ctx: &mut PaintCtx) {
        let rect = ctx.rect;
        ctx.paint_child(rect, &self.child);
        // After the child, so anything promoted here is declared later and
        // therefore composites above it.
        (self.emit)(ctx);
    }
}

/// `widget.present(|ctx| ..)` — sugar for [`Present::new`].
pub trait PresentExt: Widget + Sized + 'static {
    fn present(self, emit: impl Fn(&mut PaintCtx) + Send + Sync + 'static) -> Present<Self> {
        Present::new(self, emit)
    }
}

impl<W: Widget + Sized + 'static> PresentExt for W {}

// ── Binding an overlay to app state ───────────────────────────────────────────

/// The co-located overlay builders take a VALUE and report changes through
/// `on_open_change` — the framework never owns your state.
///
/// That is the right shape, but it means every call site repeats the same two
/// lines. These wrappers bind an `Atom` to both halves at once, which is what
/// an app with atom-shaped state actually wants:
///
/// ```rust,ignore
/// Button::new("Open").on_press(..).dialog_bound(&open, || Arc::new(body()))
/// ```
pub trait BindOverlay: rosace::widgets::tree::OverlayApi + Sized {
    fn dialog_bound(
        self,
        open: &rosace::state::Atom<bool>,
        content: impl Fn() -> rosace::widgets::tree::BoxedWidget + Send + Sync + 'static,
    ) -> rosace::widgets::tree::WithOverlay<Self> {
        let o = open.clone();
        self.dialog(open.get(), content).on_open_change(move |v| o.set(v))
    }

    fn sheet_bound(
        self,
        open: &rosace::state::Atom<bool>,
        content: impl Fn() -> rosace::widgets::tree::BoxedWidget + Send + Sync + 'static,
    ) -> rosace::widgets::tree::WithOverlay<Self> {
        let o = open.clone();
        self.sheet(open.get(), content).on_open_change(move |v| o.set(v))
    }

    fn dropdown_bound(
        self,
        open: &rosace::state::Atom<bool>,
        content: impl Fn() -> rosace::widgets::tree::BoxedWidget + Send + Sync + 'static,
    ) -> rosace::widgets::tree::WithOverlay<Self> {
        let o = open.clone();
        self.dropdown(open.get(), content).on_open_change(move |v| o.set(v))
    }

    fn toast_bound(
        self,
        open: &rosace::state::Atom<bool>,
        content: impl Fn() -> rosace::widgets::tree::BoxedWidget + Send + Sync + 'static,
    ) -> rosace::widgets::tree::WithOverlay<Self> {
        let o = open.clone();
        self.toast(open.get(), content).on_open_change(move |v| o.set(v))
    }
}

impl<W: rosace::widgets::tree::OverlayApi + Sized> BindOverlay for W {}
