//! The root component: owns navigation, app-wide state, and the theme.
//!
//! Navigation shape (the whole point of this app — see the README): a
//! `Welcome` intro, then a `Home` hub of top-level sections (Widgets,
//! Platform Channel, Persistence, Network — the last two are "Upcoming"
//! placeholders for now), then `Widgets` is itself a LIST that drills into
//! one `WidgetDetail(kind)` page per widget — never one giant kitchen-sink
//! screen. Each detail page is meant to be small and self-contained enough
//! to copy-paste directly, or for an AI assistant to learn a widget's full
//! feature set from in isolation.

use std::time::Duration;

use rosace::prelude::*;
use rosace::theme::{use_theme, set_theme_mode, ThemeMode};
use rosace::animate::use_animation;
use rosace_ffi::ChannelCallState;

use crate::screens::{home_screen, platform_channel_screen, welcome_screen, widget_detail_screen, widget_list_screen};

/// Every widget this app has a dedicated detail page for. Add a variant +
/// a `widget_detail_screen` match arm (in `screens/widgets/mod.rs`) to add
/// one — that's the entire cost of covering a new widget.
#[derive(Clone, Copy, PartialEq, Debug, Hash)]
pub enum WidgetKind {
    Checkbox,
    Radio,
    Switch,
    Button,
    TextInput,
    Slider,
    Progress,
    Card,
    Chip,
    Divider,
    Avatar,
    Badge,
    Dropdown,
    Segmented,
    Container,
    Image,
    AspectRatio,
    Grid,
    Wrap,
    Table,
    DataTable,
    Carousel,
    Tabs,
    Accordion,
    Stepper,
    RatingBar,
    Skeleton,
    TextArea,
    SearchBar,
    DatePicker,
    TimePicker,
    Tooltip,
    Menu,
    Dialog,
    Sheet,
    Drawer,
    Snackbar,
    Toast,
    NavRail,
    BottomNav,
    AppBar,
    Scaffold,
    Hero,
    ShaderPaint,
    CustomPaint,
    WillPopScope,
    ListView,
    Autocomplete,
    Dismissible,
    PullToRefresh,
    InteractiveViewer,
    Stack,
    Semantics,
    Responsive,
}

impl WidgetKind {
    pub fn name(&self) -> &'static str {
        match self {
            WidgetKind::Checkbox => "Checkbox",
            WidgetKind::Radio => "Radio",
            WidgetKind::Switch => "Switch",
            WidgetKind::Button => "Button",
            WidgetKind::TextInput => "Text Input",
            WidgetKind::Slider => "Slider",
            WidgetKind::Progress => "Progress",
            WidgetKind::Card => "Card",
            WidgetKind::Chip => "Chip",
            WidgetKind::Divider => "Divider",
            WidgetKind::Avatar => "Avatar",
            WidgetKind::Badge => "Badge",
            WidgetKind::Dropdown => "Dropdown",
            WidgetKind::Segmented => "Segmented Control",
            WidgetKind::Container => "Container",
            WidgetKind::Image => "Image",
            WidgetKind::AspectRatio => "Aspect Ratio",
            WidgetKind::Grid => "Grid",
            WidgetKind::Wrap => "Wrap",
            WidgetKind::Table => "Table",
            WidgetKind::DataTable => "Data Table",
            WidgetKind::Carousel => "Carousel",
            WidgetKind::Tabs => "Tabs",
            WidgetKind::Accordion => "Accordion",
            WidgetKind::Stepper => "Stepper",
            WidgetKind::RatingBar => "Rating Bar",
            WidgetKind::Skeleton => "Skeleton",
            WidgetKind::TextArea => "Text Area",
            WidgetKind::SearchBar => "Search Bar",
            WidgetKind::DatePicker => "Date Picker",
            WidgetKind::TimePicker => "Time Picker",
            WidgetKind::Tooltip => "Tooltip",
            WidgetKind::Menu => "Menu",
            WidgetKind::Dialog => "Dialog",
            WidgetKind::Sheet => "Sheet",
            WidgetKind::Drawer => "Drawer",
            WidgetKind::Snackbar => "Snackbar",
            WidgetKind::Toast => "Toast",
            WidgetKind::NavRail => "Nav Rail",
            WidgetKind::BottomNav => "Bottom Navigation Bar",
            WidgetKind::AppBar => "App Bar",
            WidgetKind::Scaffold => "Scaffold",
            WidgetKind::Hero => "Hero",
            WidgetKind::ShaderPaint => "Shader Paint",
            WidgetKind::CustomPaint => "Custom Paint",
            WidgetKind::WillPopScope => "Will Pop Scope",
            WidgetKind::ListView => "List View",
            WidgetKind::Autocomplete => "Autocomplete",
            WidgetKind::Dismissible => "Dismissible",
            WidgetKind::PullToRefresh => "Pull To Refresh",
            WidgetKind::InteractiveViewer => "Interactive Viewer",
            WidgetKind::Stack => "Stack",
            WidgetKind::Semantics => "Semantics",
            WidgetKind::Responsive => "Responsive",
        }
    }

    /// One line describing what the detail page covers — shown in the
    /// widget list so a reader knows what they're navigating into.
    pub fn subtitle(&self) -> &'static str {
        match self {
            WidgetKind::Checkbox => "Checked, unchecked, indeterminate, disabled, custom color",
            WidgetKind::Radio => "A grouped choice — only one option selected at a time",
            WidgetKind::Switch => "On/off with animated thumb travel",
            WidgetKind::Button => "Every variant: primary, secondary, ghost, danger",
            WidgetKind::TextInput => "Bound to app state, placeholder, and obscured (password-style)",
            WidgetKind::Slider => "A draggable value in a range, plus a custom range and disabled state",
            WidgetKind::Progress => "Linear and circular, determinate and an indeterminate spinner",
            WidgetKind::Card => "Elevated content container: border, elevation, background, radius",
            WidgetKind::Chip => "A toggleable pill for filters/tags, selected/unselected/disabled",
            WidgetKind::Divider => "A thin separating line, horizontal, vertical, or indented",
            WidgetKind::Avatar => "A circular initials badge, custom size and colors",
            WidgetKind::Badge => "Count, label, or dot marker for a corner overlay",
            WidgetKind::Dropdown => "Single selection from an opened list of options",
            WidgetKind::Segmented => "Mutually-exclusive choice shown as one connected bar",
            WidgetKind::Container => "The fundamental box: background, gradient, border, shadow, shape, material",
            WidgetKind::Image => "Placeholder fills, fit modes, and alt text",
            WidgetKind::AspectRatio => "Sizes a child to a fixed width:height ratio",
            WidgetKind::Grid => "Fixed-column grid: uniform, staggered (masonry), and bento (spanning lattice)",
            WidgetKind::Wrap => "A flow layout that wraps children onto new runs",
            WidgetKind::Table => "A layout table with auto/fixed/flex columns",
            WidgetKind::DataTable => "A data grid: sortable header, row selection, striping",
            WidgetKind::Carousel => "A swipeable page container with dot indicators",
            WidgetKind::Tabs => "An interactive tab bar over switchable content",
            WidgetKind::Accordion => "A collapsible, animated section with a chevron header",
            WidgetKind::Stepper => "A numeric −/+ control with bounds and a custom step",
            WidgetKind::RatingBar => "A tappable row of stars",
            WidgetKind::Skeleton => "A shimmering, self-animating loading placeholder",
            WidgetKind::TextArea => "A multi-line editable field with scrolling",
            WidgetKind::SearchBar => "A TextInput preset with a leading search icon",
            WidgetKind::DatePicker => "A month-grid calendar: single-date or range selection",
            WidgetKind::TimePicker => "A Material clock-dial time picker, 12h/24h",
            WidgetKind::Tooltip => "A hover label wrapping any widget",
            WidgetKind::Menu => "A vertical list of pressable rows, opened as a dropdown",
            WidgetKind::Dialog => "A title/message/actions surface — modal, non-modal, or full page",
            WidgetKind::Sheet => "A bottom sheet: content height, detents, or full screen",
            WidgetKind::Drawer => "A slide-in side panel, side or full screen",
            WidgetKind::Snackbar => "A bottom-anchored message with an action button",
            WidgetKind::Toast => "A transient auto-dismissing notification pill",
            WidgetKind::NavRail => "A vertical navigation sidebar with sections and badges",
            WidgetKind::BottomNav => "A bottom-pinned bar of 3-5 top-level destinations",
            WidgetKind::AppBar => "A top bar with title, leading, and action slots",
            WidgetKind::Scaffold => "Full-page layout: app bar, nav rail, body, FAB, bottom bar",
            WidgetKind::Hero => "Shared-element tagging for cross-screen transitions",
            WidgetKind::ShaderPaint => "Fills its rect with a registered custom shader material",
            WidgetKind::CustomPaint => "A leaf widget that draws with a closure",
            WidgetKind::WillPopScope => "Confirm before leaving a screen with unsaved work",
            WidgetKind::ListView => "A virtualized list — only the visible rows are ever built",
            WidgetKind::Autocomplete => "A text field that filters a list into an overlay as you type",
            WidgetKind::Dismissible => "Swipe a row away, with a background revealed underneath",
            WidgetKind::PullToRefresh => "Drag down past the trigger distance to refresh",
            WidgetKind::InteractiveViewer => "Pan and zoom any child, with optional zoom controls",
            WidgetKind::Stack => "Overlap children; Positioned anchors them to the edges",
            WidgetKind::Semantics => "The accessibility escape hatch: annotate or hide a subtree",
            WidgetKind::Responsive => "Builds a different tree depending on the space available",
        }
    }

    /// The full catalog, in the order the widget list shows them — the
    /// single place that determines what's covered so far.
    /// The URL slug for this widget, derived from [`WidgetKind::name`] rather
    /// than listed again: "Text Input" -> "text-input". Fifty-odd variants
    /// with a hand-written second table is fifty-odd chances for the two to
    /// disagree, and a route that formats to a path nothing parses.
    pub fn slug(&self) -> String {
        self.name().to_lowercase().replace(' ', "-")
    }

    pub const ALL: &'static [WidgetKind] = &[
        WidgetKind::Checkbox,
        WidgetKind::Radio,
        WidgetKind::Switch,
        WidgetKind::Button,
        WidgetKind::TextInput,
        WidgetKind::Slider,
        WidgetKind::Progress,
        WidgetKind::Card,
        WidgetKind::Chip,
        WidgetKind::Divider,
        WidgetKind::Avatar,
        WidgetKind::Badge,
        WidgetKind::Dropdown,
        WidgetKind::Segmented,
        WidgetKind::Container,
        WidgetKind::Image,
        WidgetKind::AspectRatio,
        WidgetKind::Grid,
        WidgetKind::Wrap,
        WidgetKind::Table,
        WidgetKind::DataTable,
        WidgetKind::Carousel,
        WidgetKind::Tabs,
        WidgetKind::Accordion,
        WidgetKind::Stepper,
        WidgetKind::RatingBar,
        WidgetKind::Skeleton,
        WidgetKind::TextArea,
        WidgetKind::SearchBar,
        WidgetKind::DatePicker,
        WidgetKind::TimePicker,
        WidgetKind::Tooltip,
        WidgetKind::Menu,
        WidgetKind::Dialog,
        WidgetKind::Sheet,
        WidgetKind::Drawer,
        WidgetKind::Snackbar,
        WidgetKind::Toast,
        WidgetKind::NavRail,
        WidgetKind::BottomNav,
        WidgetKind::AppBar,
        WidgetKind::Scaffold,
        WidgetKind::Hero,
        WidgetKind::ShaderPaint,
        WidgetKind::CustomPaint,
        WidgetKind::WillPopScope,
        WidgetKind::ListView,
        WidgetKind::Autocomplete,
        WidgetKind::Dismissible,
        WidgetKind::PullToRefresh,
        WidgetKind::InteractiveViewer,
        WidgetKind::Stack,
        WidgetKind::Semantics,
        WidgetKind::Responsive,
    ];
}

/// The tiny bit of "try it yourself" interactive state each detail page
/// needs — created once in `AppRoot::build` and threaded down as plain
/// `Atom`s, matching this codebase's established convention (see
/// `gallery_all`'s doc: "everything is one `Component` so all interactive
/// state lives at the top"). A dedicated struct rather than five separate
/// parameters so adding a widget's demo state later is a one-line change
/// here instead of a signature change everywhere `WidgetDemoState` is
/// threaded through.
#[derive(Clone)]
pub struct WidgetDemoState {
    /// Shared tap-feedback toast — see `crate::feedback`. One channel for
    /// the whole catalog rather than one atom per demo: a screen with eight
    /// controls would otherwise carry eight `Atom<bool>`s that can never be
    /// visible at once.
    pub feedback_open: Atom<bool>,
    pub feedback_message: Atom<String>,
    pub will_pop_draft: Atom<String>,
    pub will_pop_saved: Atom<String>,
    pub will_pop_confirm: Atom<bool>,
    pub autocomplete_value: Atom<String>,
    pub autocomplete_open: Atom<bool>,
    pub autocomplete_limited_value: Atom<String>,
    pub autocomplete_limited_open: Atom<bool>,
    pub dismissible_removed: Atom<i32>,
    pub pull_refresh_count: Atom<i32>,
    pub pull_refresh_busy: Atom<bool>,
    pub checkbox: Atom<bool>,
    pub radio: Atom<u8>,
    pub switch: Atom<bool>,
    pub text_input: Atom<String>,
    pub button_presses: Atom<i32>,
    /// Shared by the Slider and Progress detail pages — dragging one
    /// visibly drives the other.
    pub slider: Atom<f32>,
    /// The scrubbed-list demo: a controller the app drives, and the bar
    /// position the list reports back into.
    pub list_scrub: Atom<f32>,
    pub list_ctrl: rosace::scroll::ScrollController,
    pub chip_selected: Atom<bool>,
    pub dropdown_selected: Atom<usize>,
    /// Each Dropdown instance on the detail screen needs its own open/closed
    /// state — sharing one atom made every dropdown on the page open/close
    /// together, since `Dropdown` just reflects whatever `Atom<bool>` it's
    /// handed.
    pub dropdown_open: Atom<bool>,
    pub dropdown2_open: Atom<bool>,
    pub dropdown3_open: Atom<bool>,
    pub dropdown_styled_open: Atom<bool>,
    pub dropdown_styled_selected: Atom<usize>,
    pub segmented_selected: Atom<usize>,
    pub carousel_page: Atom<usize>,
    pub tabs_selected: Atom<usize>,
    pub accordion_expanded: Atom<bool>,
    pub accordion_styled: Atom<bool>,
    pub stepper_value: Atom<i64>,
    pub stepper_bounded: Atom<i64>,
    pub stepper_step: Atom<i64>,
    pub stepper_sized: Atom<i64>,
    pub stepper_styled: Atom<i64>,
    pub time_picker_default: Atom<SimpleTime>,
    pub time_picker_24h: Atom<SimpleTime>,
    pub time_picker_minute: Atom<SimpleTime>,
    pub time_picker_step: Atom<SimpleTime>,
    pub time_picker_styled: Atom<SimpleTime>,
    pub time_picker_unit: Atom<TimeUnit>,
    pub date_picker_single: Atom<SimpleDate>,
    pub date_picker_range: Atom<(SimpleDate, Option<SimpleDate>)>,
    pub date_picker_bounded: Atom<SimpleDate>,
    pub date_picker_accented: Atom<SimpleDate>,
    pub date_picker_vertical: Atom<SimpleDate>,
    pub date_picker_month: Atom<SimpleDate>,
    pub data_table_sort: Atom<(usize, SortDirection)>,
    pub data_table_selected: Atom<Vec<bool>>,
    pub rating_bar_value: Atom<f32>,
    pub text_area_value: Atom<String>,
    pub search_bar_value: Atom<String>,
    pub menu_open: Atom<bool>,
    pub menu_styled_open: Atom<bool>,
    pub dialog_modal_open: Atom<bool>,
    pub dialog_non_modal_open: Atom<bool>,
    pub dialog_full_page_open: Atom<bool>,
    pub dialog_styled_open: Atom<bool>,
    pub sheet_default_open: Atom<bool>,
    pub sheet_detent_open: Atom<bool>,
    pub sheet_full_open: Atom<bool>,
    pub sheet_styled_open: Atom<bool>,
    pub drawer_side_open: Atom<bool>,
    pub drawer_full_open: Atom<bool>,
    pub drawer_styled_open: Atom<bool>,
    pub snackbar_open: Atom<bool>,
    pub snackbar_styled_open: Atom<bool>,
    pub toast_info_open: Atom<bool>,
    pub toast_success_open: Atom<bool>,
    pub toast_error_open: Atom<bool>,
    pub toast_styled_open: Atom<bool>,
    pub nav_rail_selected: Atom<usize>,
    pub bottom_nav_selected: Atom<usize>,
}

impl WidgetDemoState {
    /// All the demo atoms in one place. Also used by the catalog render
    /// test, which needs a real `Context` to allocate them.
    pub fn new(ctx: &mut Context) -> Self {
        Self {
            feedback_open: ctx.state(false),
            feedback_message: ctx.state(String::new()),
            will_pop_draft: ctx.state(String::new()),
            will_pop_saved: ctx.state(String::new()),
            will_pop_confirm: ctx.state(false),
            autocomplete_value: ctx.state(String::new()),
            autocomplete_open: ctx.state(false),
            autocomplete_limited_value: ctx.state(String::new()),
            autocomplete_limited_open: ctx.state(false),
            dismissible_removed: ctx.state(0i32),
            pull_refresh_count: ctx.state(0i32),
            pull_refresh_busy: ctx.state(false),
            checkbox: ctx.state(true),
            radio: ctx.state(0u8),
            switch: ctx.state(true),
            text_input: ctx.state(String::new()),
            button_presses: ctx.state(0i32),
            slider: ctx.state(0.4f32),
            list_scrub: ctx.state(0.0f32),
            list_ctrl: ctx.state(rosace::scroll::ScrollController::new()).get(),
            chip_selected: ctx.state(true),
            dropdown_selected: ctx.state(0usize),
            dropdown_open: ctx.state(false),
            dropdown2_open: ctx.state(false),
            dropdown3_open: ctx.state(false),
            dropdown_styled_open: ctx.state(false),
            dropdown_styled_selected: ctx.state(0usize),
            segmented_selected: ctx.state(0usize),
            carousel_page: ctx.state(0usize),
            tabs_selected: ctx.state(0usize),
            accordion_expanded: ctx.state(false),
            accordion_styled: ctx.state(false),
            stepper_value: ctx.state(3i64),
            stepper_bounded: ctx.state(4i64),
            stepper_step: ctx.state(10i64),
            stepper_sized: ctx.state(3i64),
            stepper_styled: ctx.state(3i64),
            time_picker_default: ctx.state(SimpleTime::new(9, 30)),
            time_picker_24h: ctx.state(SimpleTime::new(14, 15)),
            time_picker_minute: ctx.state(SimpleTime::new(9, 30)),
            time_picker_step: ctx.state(SimpleTime::new(9, 30)),
            time_picker_styled: ctx.state(SimpleTime::new(9, 30)),
            time_picker_unit: ctx.state(TimeUnit::Hour),
            date_picker_single: ctx.state(SimpleDate::new(2026, 8, 1)),
            date_picker_range: ctx.state((SimpleDate::new(2026, 8, 5), Some(SimpleDate::new(2026, 8, 12)))),
            date_picker_bounded: ctx.state(SimpleDate::new(2026, 8, 1)),
            date_picker_accented: ctx.state(SimpleDate::new(2026, 8, 1)),
            date_picker_vertical: ctx.state(SimpleDate::new(2026, 8, 1)),
            date_picker_month: ctx.state(SimpleDate::new(2026, 8, 1)),
            data_table_sort: ctx.state((0usize, SortDirection::Ascending)),
            data_table_selected: ctx.state(vec![true, false]),
            rating_bar_value: ctx.state(3.0f32),
            text_area_value: ctx.state(String::new()),
            search_bar_value: ctx.state(String::new()),
            menu_open: ctx.state(false),
            menu_styled_open: ctx.state(false),
            dialog_modal_open: ctx.state(false),
            dialog_non_modal_open: ctx.state(false),
            dialog_full_page_open: ctx.state(false),
            dialog_styled_open: ctx.state(false),
            sheet_default_open: ctx.state(false),
            sheet_detent_open: ctx.state(false),
            sheet_full_open: ctx.state(false),
            sheet_styled_open: ctx.state(false),
            drawer_side_open: ctx.state(false),
            drawer_full_open: ctx.state(false),
            drawer_styled_open: ctx.state(false),
            snackbar_open: ctx.state(false),
            snackbar_styled_open: ctx.state(false),
            toast_info_open: ctx.state(false),
            toast_success_open: ctx.state(false),
            toast_error_open: ctx.state(false),
            toast_styled_open: ctx.state(false),
            nav_rail_selected: ctx.state(0usize),
            bottom_nav_selected: ctx.state(0usize),
        }
    }
}

impl std::fmt::Display for WidgetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.slug())
    }
}

impl std::str::FromStr for WidgetKind {
    type Err = ();
    /// Looked up against `ALL` rather than a second match arm, for the same
    /// reason `slug` is derived: one list, one source of truth.
    fn from_str(s: &str) -> Result<Self, ()> {
        WidgetKind::ALL.iter().copied().find(|k| k.slug() == s).ok_or(())
    }
}

/// Every screen in the app. Add a variant + a match arm to add a route.
#[rosace::routes]
#[derive(Clone, Copy, PartialEq, Hash, Debug)]
pub enum Screen {
    #[route("/welcome")]
    Welcome,
    #[route("/")]
    Home,
    #[route("/widgets")]
    Widgets,
    #[route("/widget/:kind")]
    WidgetDetail(WidgetKind),
    /// The Hero demo's destination. A hero morph only exists DURING a
    /// navigation, so demonstrating one needs a real second screen — the
    /// widget page itself cannot show it.
    #[route("/hero/:i")]
    HeroDetail(usize),
    /// The travel demo's destination — a different POSITION as well as a
    /// different size, so the flight is a journey rather than a growth.
    #[route("/hero-far/:i")]
    HeroFar(usize),
    #[route("/platform-channel")]
    PlatformChannel,
}

impl Screen {
    fn title(&self) -> &'static str {
        match self {
            Screen::Welcome => "",
            Screen::Home => "showcase",
            Screen::Widgets => "Widgets",
            Screen::WidgetDetail(kind) => kind.name(),
            Screen::HeroDetail(_) => "Hero",
            Screen::HeroFar(_) => "Hero",
            Screen::PlatformChannel => "Platform Channel",
        }
    }
}

pub struct AppRoot;

impl Component for AppRoot {
    fn build(&self, ctx: &mut Context) -> BoxedWidget {
        // Hooks — declared unconditionally, in a stable order.
        let nav = ScreenNav::new(ctx, Screen::Welcome);
        // D031 — keep the address bar in step on web. A no-op on desktop and
        // mobile, so it is called unconditionally.
        nav.sync_url();
        // The welcome screen's reveal animation — owned here (not inside
        // welcome_screen itself) so it survives exactly like any other
        // app-level state, matching the project's established pattern of
        // hooks living in AppRoot::build and plain values threaded down.
        let (welcome_progress, welcome_ctrl) = use_animation(ctx, Duration::from_millis(900));
        let widget_demo = WidgetDemoState::new(ctx);
        // Platform Channel demo state — see screens/platform_channel.rs's
        // module doc. Camera permission is a GlobalAtom (not a ctx.state
        // atom), so it needs the explicit subscribing accessor
        // (rosace_ffi::use_camera_permission) rather than a bare .get(), or
        // this component would never re-render when the permission resolves.
        let device_info_call: Atom<Option<Atom<ChannelCallState>>> = ctx.state(None);
        let camera_permission = rosace_ffi::use_camera_permission(ctx);

        // Same match arms build both the current and (if mid-transition)
        // previous screen, so ScreenTransitionView can animate between
        // them — see nav.push/pop's docs (default-on, theme-governed).
        let build_screen = {
            let nav = nav.clone();
            let welcome_progress = welcome_progress;
            let welcome_ctrl = welcome_ctrl.clone();
            let widget_demo = widget_demo.clone();
            let device_info_call = device_info_call.clone();
            move |s: Screen| -> BoxedWidget {
                match s {
                    Screen::Welcome => std::sync::Arc::new(welcome_screen(&welcome_progress, &welcome_ctrl, &nav)),
                    Screen::Home => std::sync::Arc::new(home_screen(&nav)),
                    Screen::Widgets => std::sync::Arc::new(widget_list_screen(&nav)),
                    Screen::WidgetDetail(kind) => widget_detail_screen(kind, &widget_demo, &nav),
                    Screen::HeroDetail(i) => std::sync::Arc::new(
                        crate::screens::widgets::hero::hero_destination(i, &nav),
                    ),
                    Screen::HeroFar(i) => std::sync::Arc::new(
                        crate::screens::widgets::hero::hero_far_destination(i, &nav),
                    ),
                    Screen::PlatformChannel => {
                        std::sync::Arc::new(platform_channel_screen(&device_info_call, camera_permission))
                    }
                }
            }
        };
        let screen = nav.current().unwrap_or(Screen::Welcome);
        let body = build_screen(screen);
        let outgoing = nav.previous().map(build_screen);
        let view = ScreenTransitionView::new(
            body, nav.current_key(), outgoing, nav.previous_key(), nav.transition_handle(), nav.stack_keys(),
        );

        // Welcome is a full-bleed intro — no app bar, no back button (it's
        // the very first thing anyone sees). Every other screen gets the
        // normal bar with a theme toggle and (off Home) a back button.
        if matches!(screen, Screen::Welcome) {
            return Scaffold::new(view).boxed();
        }

        let mut bar = AppBar::new(screen.title()).back_button(&nav);
        // A real Icon (bundled Material Symbols font, baked into the
        // binary) instead of a raw ☀/☾ Unicode character in the label
        // string — those aren't in the body-text font (Inter) and
        // rendered as a garbled/tofu glyph on Android (no OS-level
        // font-fallback there, unlike desktop).
        // Reads the LIVE theme (not a locally-tracked bool) so this button's
        // label stays correct even when the OS flips light/dark underneath
        // it (`rosace_theme::sync_system_theme`, driven by the native env
        // push) — tapping it then pins `ThemeMode::{Light,Dark}`, overriding
        // system-follow until the app is relaunched.
        let currently_dark = use_theme().is_dark;
        let (label, icon_name) = if currently_dark { ("Light", "light_mode") } else { ("Dark", "dark_mode") };
        bar = bar.action(
            Button::new(label)
                .icon(Icon::named(icon_name).size(14.0))
                .on_press(move || {
                    set_theme_mode(if currently_dark { ThemeMode::Light } else { ThemeMode::Dark });
                }),
        );

        Scaffold::new(view).app_bar(bar).boxed()
    }
}
