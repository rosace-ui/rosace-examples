//! Widget gallery — a single scrolling stress-test of the whole widget set,
//! used to exercise the GPU-shapes render path on mobile. Everything is one
//! `Component` so all interactive state lives at the top (the `picker_demo`
//! pattern); the pickers, sliders, and month-slide are the heavy animations.

use rosace::prelude::*;
use rosace::theme::set_theme;
use rosace::widgets::{
    Avatar, Badge, Chip, CircularProgress, Dropdown, Accordion, Icon, IconKind,
    ProgressBar, RatingBar, SegmentedControl, Skeleton, Stepper, TextArea, TextInput,
    DatePicker, SimpleDate, TimePicker, SimpleTime, TimeUnit,
};

pub struct AppRoot;

/// A titled section: heading + its demo widgets stacked in a card.
fn section(title: &str, children: Vec<BoxedWidget>) -> BoxedWidget {
    Box::new(
        Column::new()
            .spacing(10.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::title(title))
            .child(
                Card::new(
                    Column::new()
                        .spacing(12.0)
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .children(children),
                )
                .padding(EdgeInsets::all(16.0)),
            ),
    )
}

impl Component for AppRoot {
    fn build(&self, ctx: &mut Context) -> Element {
        // ── State (the app owns everything) ─────────────────────────────────
        let is_dark = ctx.state(false); // match the light launch theme (avoids a two-tap)
        let switch_on = ctx.state(true);
        let check_on = ctx.state(true);
        let radio_sel = ctx.state(0usize);
        let seg_sel = ctx.state(0usize);
        let chip_on = ctx.state(false);
        let slider_val = ctx.state(0.4f32);
        let stepper_val = ctx.state(2i64);
        let rating_val = ctx.state(3.0f32);
        let dd_open = ctx.state(false);
        let dd_sel = ctx.state(0usize);
        let exp_open = ctx.state(false);
        let text_val = ctx.state(String::new());
        let area_val = ctx.state(String::new());
        let cal_month = ctx.state(SimpleDate::new(2026, 7, 1));
        let cal_sel = ctx.state(None::<SimpleDate>);
        let time_val = ctx.state(SimpleTime::new(9, 30));
        let time_unit = ctx.state(TimeUnit::Hour);

        // ── Sections ────────────────────────────────────────────────────────
        let typography = section("Typography", vec![
            Box::new(Text::display("Display")),
            Box::new(Text::heading("Heading")),
            Box::new(Text::title("Title")),
            Box::new(Text::new("Body — the quick brown fox jumps over the lazy dog.")),
            Box::new(Text::caption("Caption / secondary text")),
        ]);

        let buttons = section("Buttons", vec![
            Box::new(Wrap::new().spacing(8.0).run_spacing(8.0).children(vec![
                Box::new(Button::new("Primary")),
                Box::new(Button::new("Secondary").variant(ButtonVariant::Secondary)),
                Box::new(Button::new("Ghost").variant(ButtonVariant::Ghost)),
                Box::new(Button::new("Danger").variant(ButtonVariant::Danger)),
                Box::new(Button::new("Success").variant(ButtonVariant::Success)),
                Box::new(Button::new("Link").variant(ButtonVariant::Link)),
            ])),
        ]);

        let selection = section("Selection controls", vec![
            Box::new({
                let s = switch_on.clone();
                Switch::new(switch_on.get()).label("Switch").on_change(move |v| s.set(v))
            }),
            Box::new({
                let c = check_on.clone();
                Checkbox::new(check_on.get()).label("Checkbox").on_change(move |v| c.set(v))
            }),
            Box::new(Row::new().spacing(16.0).children(
                ["A", "B", "C"].iter().enumerate().map(|(i, lbl)| {
                    let r = radio_sel.clone();
                    Box::new(
                        Radio::new(radio_sel.get() == i)
                            .label(*lbl)
                            .on_select(move || r.set(i)),
                    ) as BoxedWidget
                }).collect(),
            )),
            Box::new({
                let s = seg_sel.clone();
                SegmentedControl::new(vec!["Day", "Week", "Month"], seg_sel.get())
                    .on_change(move |i| s.set(i))
            }),
            Box::new({
                let c = chip_on.clone();
                Chip::new("Toggle chip").selected_if(chip_on.get()).on_toggle(move |v| c.set(v))
            }),
        ]);

        let inputs = section("Value inputs", vec![
            Box::new({
                let s = slider_val.clone();
                Slider::new(slider_val.get()).on_change(move |v| s.set(v))
            }),
            Box::new({
                let s = stepper_val.clone();
                Stepper::new(stepper_val.get()).on_change(move |v| s.set(v))
            }),
            Box::new({
                let r = rating_val.clone();
                RatingBar::new(rating_val.get()).on_change(move |v| r.set(v))
            }),
            Box::new({
                let s = dd_sel.clone();
                Dropdown::new(vec!["One", "Two", "Three"], dd_sel.get(), dd_open.clone())
                    .on_change(move |i| s.set(i))
            }),
            Box::new({
                let t = text_val.clone();
                TextInput::new().placeholder("Text input").value(text_val.get())
                    .on_change(move |v| t.set(v))
            }),
            Box::new({
                let a = area_val.clone();
                TextArea::new().placeholder("Text area — multi-line").value(area_val.get())
                    .height(80.0).on_change(move |v| a.set(v))
            }),
        ]);

        let indicators = section("Indicators", vec![
            Box::new(ProgressBar::new(slider_val.get())),
            Box::new(Row::new().spacing(16.0).cross_axis_alignment(CrossAxisAlignment::Center).children(vec![
                Box::new(CircularProgress::new(slider_val.get())),
                Box::new(Badge::new("9+")),
                Box::new(Avatar::new("RS")),
            ])),
            Box::new(Skeleton::new().width(200.0)),
        ]);

        let icons = section("Icons", vec![
            Box::new(Wrap::new().spacing(14.0).run_spacing(14.0).children(
                [IconKind::Home, IconKind::Search, IconKind::Settings, IconKind::User,
                 IconKind::Star, IconKind::Heart, IconKind::Bell, IconKind::Calendar,
                 IconKind::Edit, IconKind::Trash, IconKind::Download, IconKind::Filter]
                    .into_iter().map(|k| Box::new(Icon::new(k)) as BoxedWidget).collect(),
            )),
        ]);

        let disclosure = section("Disclosure", vec![
            Box::new(Accordion::new(
                "Expand for details",
                exp_open.clone(),
                Text::new("Hidden content revealed with an animated height transition."),
            )),
            Box::new(ListTile::new("List tile").subtitle("With a subtitle line")),
        ]);

        let date = {
            let (mc, sc) = (cal_month.clone(), cal_sel.clone());
            let mut dp = DatePicker::new(cal_month.get()).today(SimpleDate::new(2026, 7, 29));
            if let Some(d) = cal_sel.get() { dp = dp.selected(d); }
            section("DatePicker", vec![Box::new(
                dp.on_select(move |d, _| sc.set(Some(d))).on_month_change(move |m| mc.set(m)),
            )])
        };

        let timer = {
            let (tc, uc) = (time_val.clone(), time_unit.clone());
            section("TimePicker", vec![Box::new(
                TimePicker::new(time_val.get())
                    .editing(time_unit.get())
                    .on_change(move |v| tc.set(v))
                    .on_unit_change(move |u| uc.set(u)),
            )])
        };

        // ── Layout ──────────────────────────────────────────────────────────
        let body = ScrollView::new(
            Column::new()
                .padding(EdgeInsets::all(20.0))
                .spacing(20.0)
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .child(typography)
                .child(buttons)
                .child(selection)
                .child(inputs)
                .child(indicators)
                .child(icons)
                .child(disclosure)
                .child(date)
                .child(timer)
                .child(Spacer::gap(0.0, 32.0)),
        );

        let label = if is_dark.get() { "\u{2600} Light" } else { "\u{263e} Dark" };
        let d = is_dark.clone();
        let bar = AppBar::new("Widget Gallery").action(Button::new(label).on_press(move || {
            let next = !d.get();
            d.set(next);
            set_theme(if next { crate::theme::dark() } else { crate::theme::light() });
        }));

        Scaffold::new(body).app_bar(bar).into_element()
    }
}
