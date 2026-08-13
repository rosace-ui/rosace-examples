//! The widget catalog: a LIST of widgets (not one giant kitchen-sink
//! screen), each drilling into its own dedicated detail page covering
//! every variant/state/feature in isolation — small enough to copy-paste
//! directly, or for an AI assistant to learn one widget's full surface
//! from without wading through everything else.
//!
//! Adding a widget costs exactly three things: a `WidgetKind` variant
//! (`app.rs`), a case in `subtitle`/`ALL` there, and a new file here with
//! its own `pub fn ..._detail() -> impl Widget` wired into the `match`
//! below.

mod app_bar;
mod aspect_ratio;
mod avatar;
mod badge;
mod bottom_nav;
mod button;
mod card;
mod carousel;
mod checkbox;
mod chip;
mod container;
mod custom_paint;
mod data_table;
mod date_picker;
mod dialog;
mod divider;
mod drawer;
mod dropdown;
mod accordion;
mod grid;
mod hero;
mod image;
mod menu;
mod nav_rail;
mod progress;
mod radio;
mod rating_bar;
mod scaffold;
mod search_bar;
mod segmented;
mod shader_paint;
mod sheet;
mod skeleton;
mod slider;
mod snackbar;
mod stepper;
mod switch;
mod table;
mod tabs;
mod text_area;
mod text_input;
mod time_picker;
mod toast;
mod tooltip;
mod wrap;
mod autocomplete;
mod dismissible;
mod interactive_viewer;
mod list_view;
mod pull_to_refresh;
mod responsive;
mod semantics;
mod stack;
mod will_pop_scope;

use rosace::prelude::*;

use crate::app::{Screen, WidgetDemoState, WidgetKind};

pub fn widget_list_screen(nav: &ScreenNav<Screen>) -> impl Widget {
    let mut column = Column::new().padding(EdgeInsets::all(16.0));
    for kind in WidgetKind::ALL {
        let kind = *kind;
        let nav = nav.clone();
        column = column.child(
            ListTile::new(kind.name())
                .subtitle(kind.subtitle())
                .on_press(move || nav.push(Screen::WidgetDetail(kind))),
        );
    }
    ScrollView::new(column)
}

/// Beyond this a single column of examples stops being readable.
const MAX_CONTENT_WIDTH: f32 = 720.0;

pub fn widget_detail_screen(
    kind: WidgetKind,
    demo: &WidgetDemoState,
    nav: &ScreenNav<Screen>,
) -> BoxedWidget {
    let fb = crate::feedback::Feedback::new(
        demo.feedback_open.clone(),
        demo.feedback_message.clone(),
    );

    // Cap the content column on a wide window and centre it. A catalog page
    // is a single column of examples; stretched edge-to-edge on a desktop
    // window it becomes a strip of controls with a metre of dead space
    // beside it, and long labels turn into unreadable full-width lines.
    // Phones take the `else` branch and are untouched.
    //
    // The body is REBUILT inside the closure rather than built once and
    // moved in: `Responsive` runs its builder during both layout and paint,
    // so a build-once-and-hand-over version would yield the real page on the
    // layout pass and an empty one on the paint pass. `WidgetDemoState` is
    // all `Atom`s, so cloning it is cheap and shares the same state.
    let (demo, fb2, nav2) = (demo.clone(), fb.clone(), nav.clone());
    let page = Responsive::new(move |space| {
        let body = widget_detail_body(kind, &demo, &fb2, &nav2);
        if space.width > MAX_CONTENT_WIDTH {
            let side = (space.width - MAX_CONTENT_WIDTH) / 2.0;
            Box::new(
                Row::new()
                    .child(Spacer::new(side))
                    .child(Expanded::new(body))
                    .child(Spacer::new(side)),
            )
        } else {
            body
        }
    });

    Box::new(fb.attach(page))
}

fn widget_detail_body(
    kind: WidgetKind,
    demo: &WidgetDemoState,
    fb: &crate::feedback::Feedback,
    nav: &ScreenNav<Screen>,
) -> BoxedWidget {
    match kind {
        WidgetKind::Checkbox => Box::new(checkbox::checkbox_detail(&demo.checkbox)),
        WidgetKind::Radio => Box::new(radio::radio_detail(&demo.radio, fb)),
        WidgetKind::Switch => Box::new(switch::switch_detail(&demo.switch)),
        WidgetKind::Button => Box::new(button::button_detail(&demo.button_presses, fb)),
        WidgetKind::TextInput => Box::new(text_input::text_input_detail(&demo.text_input)),
        WidgetKind::Slider => Box::new(slider::slider_detail(&demo.slider)),
        WidgetKind::Progress => Box::new(progress::progress_detail(&demo.slider)),
        WidgetKind::Card => Box::new(card::card_detail()),
        WidgetKind::Chip => Box::new(chip::chip_detail(&demo.chip_selected)),
        WidgetKind::Divider => Box::new(divider::divider_detail()),
        WidgetKind::Avatar => Box::new(avatar::avatar_detail()),
        WidgetKind::Badge => Box::new(badge::badge_detail()),
        WidgetKind::Dropdown => Box::new(dropdown::dropdown_detail(
            &demo.dropdown_selected, &demo.dropdown_open, &demo.dropdown2_open, &demo.dropdown3_open,
            &demo.dropdown_styled_open, &demo.dropdown_styled_selected,
        )),
        WidgetKind::Segmented => Box::new(segmented::segmented_detail(&demo.segmented_selected)),
        WidgetKind::Carousel => Box::new(carousel::carousel_detail(&demo.carousel_page)),
        WidgetKind::Image => Box::new(image::image_detail()),
        WidgetKind::Container => Box::new(container::container_detail()),
        WidgetKind::ShaderPaint => Box::new(shader_paint::shader_paint_detail()),
        WidgetKind::CustomPaint => Box::new(custom_paint::custom_paint_detail()),
        WidgetKind::DataTable => Box::new(data_table::data_table_detail(&demo.data_table_sort, &demo.data_table_selected)),
        WidgetKind::DatePicker => Box::new(date_picker::date_picker_detail(
            &demo.date_picker_single, &demo.date_picker_range, &demo.date_picker_bounded,
            &demo.date_picker_accented, &demo.date_picker_vertical, &demo.date_picker_month,
        )),
        WidgetKind::TimePicker => Box::new(time_picker::time_picker_detail(
            &demo.time_picker_default, &demo.time_picker_24h, &demo.time_picker_minute,
            &demo.time_picker_step, &demo.time_picker_styled, &demo.time_picker_unit,
        )),
        WidgetKind::Dialog => Box::new(dialog::dialog_detail(
            &demo.dialog_modal_open, &demo.dialog_non_modal_open, &demo.dialog_full_page_open, &demo.dialog_styled_open,
        )),
        WidgetKind::Drawer => Box::new(drawer::drawer_detail(
            &demo.drawer_side_open, &demo.drawer_full_open, &demo.drawer_styled_open,
        )),
        WidgetKind::Accordion => Box::new(accordion::accordion_detail(&demo.accordion_expanded, &demo.accordion_styled)),
        WidgetKind::Grid => Box::new(grid::grid_detail()),
        WidgetKind::Menu => Box::new(menu::menu_detail(&demo.menu_open, &demo.menu_styled_open, fb)),
        WidgetKind::NavRail => Box::new(nav_rail::nav_rail_detail(&demo.nav_rail_selected)),
        WidgetKind::RatingBar => Box::new(rating_bar::rating_bar_detail(&demo.rating_bar_value)),
        WidgetKind::Scaffold => Box::new(scaffold::scaffold_detail()),
        WidgetKind::SearchBar => Box::new(search_bar::search_bar_detail(&demo.search_bar_value, fb)),
        WidgetKind::Sheet => Box::new(sheet::sheet_detail(
            &demo.sheet_default_open, &demo.sheet_detent_open, &demo.sheet_full_open, &demo.sheet_styled_open,
        )),
        WidgetKind::Skeleton => Box::new(skeleton::skeleton_detail()),
        WidgetKind::Snackbar => Box::new(snackbar::snackbar_detail(&demo.snackbar_open, &demo.snackbar_styled_open)),
        WidgetKind::Stepper => Box::new(stepper::stepper_detail(
            &demo.stepper_value, &demo.stepper_bounded, &demo.stepper_step, &demo.stepper_sized, &demo.stepper_styled,
        )),
        WidgetKind::Table => Box::new(table::table_detail()),
        WidgetKind::Tabs => Box::new(tabs::tabs_detail(&demo.tabs_selected)),
        WidgetKind::TextArea => Box::new(text_area::text_area_detail(&demo.text_area_value)),
        WidgetKind::Toast => Box::new(toast::toast_detail(
            &demo.toast_info_open, &demo.toast_success_open, &demo.toast_error_open, &demo.toast_styled_open,
        )),
        WidgetKind::Tooltip => Box::new(tooltip::tooltip_detail()),
        WidgetKind::Wrap => Box::new(wrap::wrap_detail()),
        WidgetKind::AspectRatio => Box::new(aspect_ratio::aspect_ratio_detail()),
        WidgetKind::BottomNav => Box::new(bottom_nav::bottom_nav_detail(&demo.bottom_nav_selected)),
        WidgetKind::AppBar => Box::new(app_bar::app_bar_detail()),
        WidgetKind::Hero => Box::new(hero::hero_detail()),
        WidgetKind::ListView => Box::new(list_view::list_view_detail(fb)),
        WidgetKind::Autocomplete => Box::new(autocomplete::autocomplete_detail(
            &demo.autocomplete_value, &demo.autocomplete_open,
            &demo.autocomplete_limited_value, &demo.autocomplete_limited_open, fb,
        )),
        WidgetKind::Dismissible => Box::new(dismissible::dismissible_detail(&demo.dismissible_removed, fb)),
        WidgetKind::PullToRefresh => Box::new(pull_to_refresh::pull_to_refresh_detail(
            &demo.pull_refresh_count, &demo.pull_refresh_busy, fb,
        )),
        WidgetKind::InteractiveViewer => Box::new(interactive_viewer::interactive_viewer_detail()),
        WidgetKind::Stack => Box::new(stack::stack_detail(fb)),
        WidgetKind::Semantics => Box::new(semantics::semantics_detail(fb)),
        WidgetKind::Responsive => Box::new(responsive::responsive_detail()),
        WidgetKind::WillPopScope => Box::new(will_pop_scope::will_pop_scope_detail(
            &demo.will_pop_draft, &demo.will_pop_saved, &demo.will_pop_confirm, nav, fb,
        )),
    }
}
