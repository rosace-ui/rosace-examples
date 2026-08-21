//! Phase 32 Step 4 exit-bar demo: a real running app rendering a string
//! containing at least one emoji correctly (real color, not a missing-glyph
//! box), inline with regular text.

use rosace::prelude::*;

struct EmojiDemo;

impl Component for EmojiDemo {
    fn build(&self, _ctx: &mut Context) -> BoxedWidget {
        Scaffold::new(
            Column::new()
                .padding(EdgeInsets::all(24.0))
                .spacing(16.0)
                .child(Text::new("Emoji inline with text: 😀🎉🚀 hello").size(24.0))
                .child(Text::new("Smaller size: ☀️ sunny today ⭐").size(16.0))
                .child(Text::new("Plain text only, no emoji here").size(16.0)),
        )
        .app_bar(AppBar::new("emoji_demo"))
        .boxed()
    }
}

fn main() {
    env_logger::init();
    App::new().title("emoji_demo").size(420, 260).launch(EmojiDemo);
}
