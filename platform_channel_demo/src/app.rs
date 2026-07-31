//! The root component: owns navigation, app-wide state, and the theme.

use rosace::prelude::*;
use rosace::theme::set_theme;
use rosace_ffi::ChannelCallState;

use crate::screens::{counter_screen, home_screen, platform_channel_screen};

/// Every screen in the app. Add a variant + a match arm to add a route.
#[derive(Clone, Copy, PartialEq)]
pub enum Screen {
    Home,
    Counter,
    PlatformChannel,
}

impl Screen {
    fn title(&self) -> &'static str {
        match self {
            Screen::Home => "platform_channel_demo",
            Screen::Counter => "Counter",
            Screen::PlatformChannel => "Platform Channel",
        }
    }
}

pub struct AppRoot;

impl Component for AppRoot {
    fn build(&self, ctx: &mut Context) -> Element {
        // Hooks — declared unconditionally, in a stable order.
        let nav = ScreenNav::new(ctx, Screen::Home);
        let count = ctx.state(0i32);
        // Starts `false` to match the launch theme (light — see `theme.rs`/
        // `ffi.rs`). If this disagreed with the actual initial theme, the first
        // toggle tap would set the theme it's already showing (a no-op), so it
        // would take two taps to flip the first time.
        let is_dark = ctx.state(false);

        // Platform Channel demo state (see screens/platform_channel.rs). The
        // "Device Info" call's result-atom is created lazily (None until the
        // button is first pressed), so it's stored in ctx.state like any
        // other app state. Camera permission is a GlobalAtom, which is NOT
        // auto-subscribed by ctx.state's hook machinery — use_camera_permission
        // does the explicit subscribe(ctx.component_id()) that makes reading
        // it here actually reactive (see that fn's doc for why).
        let device_info_call: Atom<Option<Atom<ChannelCallState>>> = ctx.state(None);
        let camera_permission = rosace_ffi::use_camera_permission(ctx);

        // Same match arms build both the current and (if mid-transition)
        // previous screen, so ScreenTransitionView can animate between
        // them — see nav.push/pop's docs (default-on, theme-governed).
        let build_screen = {
            let nav = nav.clone();
            let count = count.clone();
            let device_info_call = device_info_call.clone();
            move |s: Screen| -> BoxedWidget {
                match s {
                    Screen::Home => Box::new(home_screen(&nav)),
                    Screen::Counter => Box::new(counter_screen(&count)),
                    Screen::PlatformChannel => {
                        Box::new(platform_channel_screen(&device_info_call, camera_permission))
                    }
                }
            }
        };
        let screen = nav.current().unwrap_or(Screen::Home);
        let body = build_screen(screen);
        let outgoing = nav.previous().map(build_screen);
        let view = ScreenTransitionView::new(body, outgoing, nav.transition_handle());

        // App bar: a back button appears off Home; a theme toggle on the right.
        let mut bar = AppBar::new(screen.title()).back_button(&nav);
        let label = if is_dark.get() { "\u{2600} Light" } else { "\u{263e} Dark" };
        let d = is_dark.clone();
        bar = bar.action(Button::new(label).on_press(move || {
            let next = !d.get();
            d.set(next);
            set_theme(if next { crate::theme::dark() } else { crate::theme::light() });
        }));

        Scaffold::new(view).app_bar(bar).into_element()
    }
}
