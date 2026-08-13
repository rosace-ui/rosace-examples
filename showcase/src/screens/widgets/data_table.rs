//! `DataTable` — the data-grid rendering layer on top of `Table`: sortable
//! header, optional row-selection checkboxes, row striping. Sorting is
//! rendering-only — the app owns the actual comparator.

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

pub fn data_table_detail(
    sort: &Atom<(usize, SortDirection)>,
    selected: &Atom<Vec<bool>>,
) -> impl Widget {
    let cols = || vec![DataTableColumn::new("Name"), DataTableColumn::new("Role"), DataTableColumn::new("Status")];
    let sort_cb = sort.clone();
    let (sort_col, sort_dir) = sort.get();
    let sel_cb = selected.clone();
    ScrollView::new(
        Column::new()
            .padding(EdgeInsets::all(16.0))
            .spacing(20.0)
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .child(labeled(
                "Basic table",
                DataTable::new(cols())
                    .row(vec!["Ada", "Engineer", "Active"])
                    .row(vec!["Grace", "Manager", "Active"])
                    .row(vec!["Alan", "Researcher", "Away"]),
            ))
            .child(labeled(
                &format!("Sorted indicator + row striping — tap a header (col {sort_col}, {sort_dir:?})"),
                DataTable::new(cols())
                    .row(vec!["Ada", "Engineer", "Active"])
                    .row(vec!["Grace", "Manager", "Active"])
                    .row(vec!["Alan", "Researcher", "Away"])
                    .sorted_by(sort_col, sort_dir)
                    .row_striping(Color::rgb(245, 245, 248))
                    .on_sort(move |col, dir| sort_cb.set((col, dir))),
            ))
            .child(labeled(
                &format!("Selectable rows — {:?}", selected.get()),
                DataTable::new(cols())
                    .row(vec!["Ada", "Engineer", "Active"])
                    .row(vec!["Grace", "Manager", "Active"])
                    .selectable(selected.get())
                    .on_select(move |row, checked| {
                        let mut v = sel_cb.get();
                        if row < v.len() { v[row] = checked; }
                        sel_cb.set(v);
                    }),
            ))
            .child(labeled(
                "Fixed + flex column widths",
                DataTable::new(vec![
                    DataTableColumn::new("Id").fixed_width(50.0),
                    DataTableColumn::new("Name").flex(1.0),
                    DataTableColumn::new("Notes").flex(2.0),
                ])
                .row(vec!["1", "Ada", "Founding engineer"])
                .row(vec!["2", "Grace", "Compiler pioneer"]),
            )),
    )
}
