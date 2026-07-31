//! One file per screen. Re-export each screen's builder here.

mod counter;
mod home;
mod platform_channel;

pub use counter::counter_screen;
pub use home::home_screen;
pub use platform_channel::platform_channel_screen;
