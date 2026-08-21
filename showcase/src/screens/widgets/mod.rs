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
            std::sync::Arc::new(
                Row::new()
                    .child(Spacer::new(side))
                    .child(Expanded::new(body))
                    .child(Spacer::new(side)),
            )
        } else {
            body
        }
    });

    std::sync::Arc::new(fb.attach(page))
}

fn widget_detail_body(
    kind: WidgetKind,
    demo: &WidgetDemoState,
    fb: &crate::feedback::Feedback,
    nav: &ScreenNav<Screen>,
) -> BoxedWidget {
    match kind {
        WidgetKind::Checkbox => std::sync::Arc::new(checkbox::checkbox_detail(&demo.checkbox)),
        WidgetKind::Radio => std::sync::Arc::new(radio::radio_detail(&demo.radio, fb)),
        WidgetKind::Switch => std::sync::Arc::new(switch::switch_detail(&demo.switch)),
        WidgetKind::Button => std::sync::Arc::new(button::button_detail(&demo.button_presses, fb)),
        WidgetKind::TextInput => std::sync::Arc::new(text_input::text_input_detail(&demo.text_input)),
        WidgetKind::Slider => std::sync::Arc::new(slider::slider_detail(&demo.slider)),
        WidgetKind::Progress => std::sync::Arc::new(progress::progress_detail(&demo.slider)),
        WidgetKind::Card => std::sync::Arc::new(card::card_detail()),
        WidgetKind::Chip => std::sync::Arc::new(chip::chip_detail(&demo.chip_selected)),
        WidgetKind::Divider => std::sync::Arc::new(divider::divider_detail()),
        WidgetKind::Avatar => std::sync::Arc::new(avatar::avatar_detail()),
        WidgetKind::Badge => std::sync::Arc::new(badge::badge_detail()),
        WidgetKind::Dropdown => std::sync::Arc::new(dropdown::dropdown_detail(
            &demo.dropdown_selected, &demo.dropdown_open, &demo.dropdown2_open, &demo.dropdown3_open,
            &demo.dropdown_styled_open, &demo.dropdown_styled_selected,
        )),
        WidgetKind::Segmented => std::sync::Arc::new(segmented::segmented_detail(&demo.segmented_selected)),
        WidgetKind::Carousel => std::sync::Arc::new(carousel::carousel_detail(&demo.carousel_page)),
        WidgetKind::Image => std::sync::Arc::new(image::image_detail()),
        WidgetKind::Container => std::sync::Arc::new(container::container_detail()),
        WidgetKind::ShaderPaint => std::sync::Arc::new(shader_paint::shader_paint_detail()),
        WidgetKind::CustomPaint => std::sync::Arc::new(custom_paint::custom_paint_detail()),
        WidgetKind::DataTable => std::sync::Arc::new(data_table::data_table_detail(&demo.data_table_sort, &demo.data_table_selected)),
        WidgetKind::DatePicker => std::sync::Arc::new(date_picker::date_picker_detail(
            &demo.date_picker_single, &demo.date_picker_range, &demo.date_picker_bounded,
            &demo.date_picker_accented, &demo.date_picker_vertical, &demo.date_picker_month,
        )),
        WidgetKind::TimePicker => std::sync::Arc::new(time_picker::time_picker_detail(
            &demo.time_picker_default, &demo.time_picker_24h, &demo.time_picker_minute,
            &demo.time_picker_step, &demo.time_picker_styled, &demo.time_picker_unit,
        )),
        WidgetKind::Dialog => std::sync::Arc::new(dialog::dialog_detail(
            &demo.dialog_modal_open, &demo.dialog_non_modal_open, &demo.dialog_full_page_open, &demo.dialog_styled_open,
        )),
        WidgetKind::Drawer => std::sync::Arc::new(drawer::drawer_detail(
            &demo.drawer_side_open, &demo.drawer_full_open, &demo.drawer_styled_open,
        )),
        WidgetKind::Accordion => std::sync::Arc::new(accordion::accordion_detail(&demo.accordion_expanded, &demo.accordion_styled)),
        WidgetKind::Grid => std::sync::Arc::new(grid::grid_detail()),
        WidgetKind::Menu => std::sync::Arc::new(menu::menu_detail(&demo.menu_open, &demo.menu_styled_open, fb)),
        WidgetKind::NavRail => std::sync::Arc::new(nav_rail::nav_rail_detail(&demo.nav_rail_selected)),
        WidgetKind::RatingBar => std::sync::Arc::new(rating_bar::rating_bar_detail(&demo.rating_bar_value)),
        WidgetKind::Scaffold => std::sync::Arc::new(scaffold::scaffold_detail()),
        WidgetKind::SearchBar => std::sync::Arc::new(search_bar::search_bar_detail(&demo.search_bar_value, fb)),
        WidgetKind::Sheet => std::sync::Arc::new(sheet::sheet_detail(
            &demo.sheet_default_open, &demo.sheet_detent_open, &demo.sheet_full_open, &demo.sheet_styled_open,
        )),
        WidgetKind::Skeleton => std::sync::Arc::new(skeleton::skeleton_detail()),
        WidgetKind::Snackbar => std::sync::Arc::new(snackbar::snackbar_detail(&demo.snackbar_open, &demo.snackbar_styled_open)),
        WidgetKind::Stepper => std::sync::Arc::new(stepper::stepper_detail(
            &demo.stepper_value, &demo.stepper_bounded, &demo.stepper_step, &demo.stepper_sized, &demo.stepper_styled,
        )),
        WidgetKind::Table => std::sync::Arc::new(table::table_detail()),
        WidgetKind::Tabs => std::sync::Arc::new(tabs::tabs_detail(&demo.tabs_selected)),
        WidgetKind::TextArea => std::sync::Arc::new(text_area::text_area_detail(&demo.text_area_value)),
        WidgetKind::Toast => std::sync::Arc::new(toast::toast_detail(
            &demo.toast_info_open, &demo.toast_success_open, &demo.toast_error_open, &demo.toast_styled_open,
        )),
        WidgetKind::Tooltip => std::sync::Arc::new(tooltip::tooltip_detail()),
        WidgetKind::Wrap => std::sync::Arc::new(wrap::wrap_detail()),
        WidgetKind::AspectRatio => std::sync::Arc::new(aspect_ratio::aspect_ratio_detail()),
        WidgetKind::BottomNav => std::sync::Arc::new(bottom_nav::bottom_nav_detail(&demo.bottom_nav_selected)),
        WidgetKind::AppBar => std::sync::Arc::new(app_bar::app_bar_detail()),
        WidgetKind::Hero => std::sync::Arc::new(hero::hero_detail()),
        WidgetKind::ListView => std::sync::Arc::new(list_view::list_view_detail(fb)),
        WidgetKind::Autocomplete => std::sync::Arc::new(autocomplete::autocomplete_detail(
            &demo.autocomplete_value, &demo.autocomplete_open,
            &demo.autocomplete_limited_value, &demo.autocomplete_limited_open, fb,
        )),
        WidgetKind::Dismissible => std::sync::Arc::new(dismissible::dismissible_detail(&demo.dismissible_removed, fb)),
        WidgetKind::PullToRefresh => std::sync::Arc::new(pull_to_refresh::pull_to_refresh_detail(
            &demo.pull_refresh_count, &demo.pull_refresh_busy, fb,
        )),
        WidgetKind::InteractiveViewer => std::sync::Arc::new(interactive_viewer::interactive_viewer_detail()),
        WidgetKind::Stack => std::sync::Arc::new(stack::stack_detail(fb)),
        WidgetKind::Semantics => std::sync::Arc::new(semantics::semantics_detail(fb)),
        WidgetKind::Responsive => std::sync::Arc::new(responsive::responsive_detail()),
        WidgetKind::WillPopScope => std::sync::Arc::new(will_pop_scope::will_pop_scope_detail(
            &demo.will_pop_draft, &demo.will_pop_saved, &demo.will_pop_confirm, nav, fb,
        )),
    }
}
