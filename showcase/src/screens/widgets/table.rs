//! `Table` — a LAYOUT table: per-column sizing (auto/fixed/flex) with
//! plain widget cells. Distinct from `DataTable`, the data-grid rendered
//! on top of it.

use rosace::prelude::*;

fn labeled(title: &str, child: impl Widget + 'static) -> BoxedWidget {
    std::sync::Arc::new(
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
                    .row(vec![std::sync::Arc::new(Text::new("Name")), std::sync::Arc::new(Text::new("Ada"))])
                    .row(vec![std::sync::Arc::new(Text::new("Role")), std::sync::Arc::new(Text::new("Engineer"))]),
            ))
            .child(labeled(
                "Fixed + flex columns",
                Table::new()
                    .columns(vec![TableColumn::fixed(60.0), TableColumn::flex(1.0)])
                    .row(vec![std::sync::Arc::new(Text::new("Id")), std::sync::Arc::new(Text::new("1"))])
                    .row(vec![std::sync::Arc::new(Text::new("Note")), std::sync::Arc::new(Text::new("A longer flexible cell"))]),
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
                    .row(vec![std::sync::Arc::new(Text::new("A")), std::sync::Arc::new(Text::new("1"))])
                    .row(vec![std::sync::Arc::new(Text::new("B")), std::sync::Arc::new(Text::new("2"))])
                    .row(vec![std::sync::Arc::new(Text::new("C")), std::sync::Arc::new(Text::new("3"))]),
            )),
    )
}
