//! One file per screen (or, for `widgets`, one file per widget). Re-export
//! each screen's builder here.

mod home;
mod platform_channel;
mod welcome;
pub(crate) mod widgets;

pub use home::home_screen;
pub use platform_channel::platform_channel_screen;
pub use welcome::welcome_screen;
pub use widgets::{widget_detail_screen, widget_list_screen};
