//! A4 live-verify: a fully interactive counter written ENTIRELY in `view!` —
//! a working button INSIDE the `view!`. Before A4 this would panic under
//! `rsc dev` (the closure couldn't inflate); now it runs and hot-reloads.
//!
//! Run: `cargo run --bin hot_reload_button_demo --features rosace/rsc-hot`
//! (or `rsc dev --bin hot_reload_button_demo`). Click the button — it counts.
//! Edit the `spacing:` or the `Text(..)` and save — the running app hot-swaps.

use rosace::prelude::*;

struct Demo;

impl Component for Demo {
    fn build(&self, ctx: &mut Context) -> Element {
        let count = ctx.state(0i32);
        let display = count.clone();

        Scaffold::new(view! {
            Column {
                spacing: 20.0
                Text("Interactive hot-reload — click, then edit & save")
                Text(format!("Count: {}", display.get()))
                Button("Increment") { on_press: move || count.update(|n| n + 1) }
            }
        })
        .app_bar(AppBar::new("Button Hot Reload"))
        .into_element()
    }
}

fn main() {
    App::new().title("Button Hot Reload").size(520, 400).launch(Demo);
}
