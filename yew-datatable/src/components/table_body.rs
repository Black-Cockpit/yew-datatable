//! Table body component.

use crate::components::cell_render_context::CellRenderContext;
use crate::hooks::use_table::UseTableHandle;
use yew::prelude::*;

/// Props for the TableBody component.
#[derive(Properties, Clone)]
pub struct TableBodyProps<T: Clone + PartialEq + 'static> {
    /// The table handle from use_table hook.
    pub table: UseTableHandle<T>,

    /// Custom class for the tbody element.
    #[prop_or_default]
    pub class: Classes,

    /// Custom class for tr elements.
    #[prop_or_default]
    pub tr_class: Classes,

    /// Custom class for td elements.
    #[prop_or_default]
    pub td_class: Classes,

    /// Custom class for selected rows.
    #[prop_or_default]
    pub selected_class: Classes,

    /// Whether rows are selectable by clicking.
    #[prop_or(true)]
    pub selectable: bool,

    /// Custom cell renderer.
    #[prop_or_default]
    pub render_cell: Option<Callback<CellRenderContext<T>, Html>>,
}

/// Compares `TableBodyProps` by all fields, including the table handle.
///
/// The `render_cell` callback is compared through `Callback`'s `PartialEq`,
/// which is pointer equality on the inner `Rc`, so the body memoizes
/// correctly and re-renders only when the callback identity changes.
impl<T: Clone + PartialEq + 'static> PartialEq for TableBodyProps<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare the table handle for reactivity-aware equality.
        self.table == other.table
            // Compare all configuration fields.
            && self.class == other.class
            && self.tr_class == other.tr_class
            && self.td_class == other.td_class
            && self.selected_class == other.selected_class
            && self.selectable == other.selectable
            // Compare the cell renderer by callback pointer equality.
            && self.render_cell == other.render_cell
    }
}

/// Table body component that renders rows and cells.
#[function_component(TableBody)]
pub fn table_body<T: Clone + PartialEq + 'static>(props: &TableBodyProps<T>) -> Html {
    // Retrieve the ordered list of visible column identifiers.
    let column_ids = props.table.visible_column_ids();

    // Collect the visible rows after processing.
    let rows = props.table.visible_rows();

    html! {
        <tbody class={props.class.clone()}>
            {rows.into_iter().enumerate().map(|(row_idx, row)| {
                // Extract the row identifier and selection state.
                let row_id = row.id.clone();
                let is_selected = props.table.is_row_selected(&row_id);

                // Apply the selected class if the row is selected.
                let row_class = if is_selected {
                    classes!(props.tr_class.clone(), props.selected_class.clone())
                } else {
                    props.tr_class.clone()
                };

                // Create the click handler for row selection toggling.
                let onclick = if props.selectable {
                    let table = props.table.clone();
                    let row_id = row_id.clone();
                    Some(Callback::from(move |_: MouseEvent| {
                        table.toggle_row_selection(row_id.clone());
                    }))
                } else {
                    None
                };

                html! {
                    <tr
                        key={row_id.as_str().to_string()}
                        class={row_class}
                        onclick={onclick}
                    >
                        {column_ids.iter().enumerate().map(|(col_idx, column_id)| {
                            // Retrieve the cell value for the current column.
                            let value = props.table.get_cell_value(&row.original, column_id)
                                .unwrap_or_default();

                            // Use the custom renderer if provided, otherwise render plain text.
                            let cell_html = if let Some(render_cell) = &props.render_cell {
                                let ctx = CellRenderContext {
                                    row: row.original.clone(),
                                    row_id: row_id.clone(),
                                    row_index: row_idx,
                                    column_id: column_id.as_str().to_string(),
                                    column_index: col_idx,
                                    value: value.clone(),
                                };
                                render_cell.emit(ctx)
                            } else {
                                html! { {value} }
                            };

                            html! {
                                <td
                                    key={column_id.as_str().to_string()}
                                    class={props.td_class.clone()}
                                >
                                    {cell_html}
                                </td>
                            }
                        }).collect::<Html>()}
                    </tr>
                }
            }).collect::<Html>()}
        </tbody>
    }
}
