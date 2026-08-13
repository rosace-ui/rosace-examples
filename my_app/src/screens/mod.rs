//! One file per screen. Re-export each screen's builder here.

mod calendar;
mod counter;
mod home;
mod timer;

pub use calendar::calendar_screen;
pub use counter::counter_screen;
pub use home::home_screen;
pub use timer::timer_screen;
