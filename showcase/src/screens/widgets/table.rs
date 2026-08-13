//! `Table` — a LAYOUT table: per-column sizing (auto/fixed/flex) with
//! plain widget cells. Distinct from `DataTable`, the data-grid rendered
//! on top of it.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    Box::new(
        Column::new()
            .spacing(6.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(Text::new(title).color(Color::rgb(120, 120, 120)))
            .child(child),
    )
}

pub fn table_detail() -> impl Widget {
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Auto columns",
                Table::new()
                    .columns(vec![TableColumn::auto(), TableColumn::auto()])
                    .row(vec![Box::new(Text::new("Name")), Box::new(Text::new("Ada"))])
                    .row(vec![Box::new(Text::new("Role")), Box::new(Text::new("Engineer"))]),
            ))
            .child(labeled(
                "Fixed + flex columns",
                Table::new()
                    .columns(vec![TableColumn::fixed(60.0), TableColumn::flex(1.0)])
                    .row(vec![Box::new(Text::new("Id")), Box::new(Text::new("1"))])
                    .row(vec![Box::new(Text::new("Note")), Box::new(Text::new("A longer flexible cell"))]),
            ))
            .child(labeled(
                "Custom spacing + cell padding + row striping + divider",
                Table::new()
                    .columns(vec![TableColumn::auto(), TableColumn::auto()])
                    .spacing(16.0, 10.0)
                    .cell_padding(6.0)
                    .row_background(Color::rgb(245, 245, 248))
                    .divider(1.0)
                    .divider_color(Color::rgb(220, 220, 225))
                    .row(vec![Box::new(Text::new("A")), Box::new(Text::new("1"))])
                    .row(vec![Box::new(Text::new("B")), Box::new(Text::new("2"))])
                    .row(vec![Box::new(Text::new("C")), Box::new(Text::new("3"))]),
            )),
    )
}
