//! Tier 1 hot-reload demo — a STANDALONE app (own workspace), the reference
//! for how a `view!`/hot-reload app is structured.
//!
//! Run from this directory:
//!   rsc dev                       (recommended — turns on rsc-hot)
//!   cargo run --features rosace/rsc-hot
//!
//! Then edit the `spacing:` value inside the `view!` below and save — the
//! running app swaps the descriptor live (watch the `[hot-reload] ⚡ swapped …`
//! line) with no recompile. Changing a literal/structure is a data swap;
//! adding a `{expr}` hole or a new `view!` prints a "needs a recompile" note.

use rosace::prelude::*;

struct Demo;

impl Component for Demo {
    fn build(&self, _ctx: &mut Context) -> Element {
        Scaffold::new(
            Column::new()
                .padding(EdgeInsets::all(24.0))
                .child(Text::new("Hot reload demo — edit the view! spacing and save"))
                // The hot-swap target: a `view!` site (Column is a registered,
                // inflatable widget). Edit `spacing:` and save.
                .child(view! {
                    Column {
                        spacing: 20.0
                    }
                }),
        )
        .app_bar(AppBar::new("Hot Reload"))
        .into_element()
    }
}

fn main() {
    App::new().title("Hot Reload Demo").size(560, 420).launch(Demo);
}
