//! Main DataTable component.

use crate::components::pagination::Pagination;
use crate::components::table_body::TableBody;
use crate::components::table_header::TableHeader;
use crate::hooks::use_table::UseTableHandle;
use yew::prelude::*;

/// Props for the DataTable component.
///
/// This component accepts a pre-created table handle from the `use_table` hook.
/// For usage, create the table handle in your component and pass it here.
#[derive(Properties, Clone)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    /// The table handle from use_table hook.
    pub table: UseTableHandle<T>,

    /// Custom class for the table container.
    #[prop_or_default]
    pub class: Classes,

    /// Custom class for the table element.
    #[prop_or_default]
    pub table_class: Classes,

    /// Whether to show pagination.
    #[prop_or(true)]
    pub show_pagination: bool,

    /// Whether to show global filter.
    #[prop_or(true)]
    pub show_global_filter: bool,

    /// Placeholder for global filter input.
    #[prop_or_else(|| "Search...".to_string())]
    pub filter_placeholder: String,

    /// Whether rows are selectable.
    #[prop_or(true)]
    pub selectable: bool,
}

/// Compares `DataTableProps` by their configuration fields, excluding the table handle.
impl<T: Clone + PartialEq + 'static> PartialEq for DataTableProps<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare all configuration fields except the table handle.
        self.class == other.class
            && self.table_class == other.table_class
            && self.show_pagination == other.show_pagination
            && self.show_global_filter == other.show_global_filter
            && self.filter_placeholder == other.filter_placeholder
            && self.selectable == other.selectable
    }
}

/// A complete data table component with all features.
///
/// Use this component with a table handle created by the `use_table` hook:
///
/// ```ignore
/// let table = use_table(columns, data, None);
/// html! { <DataTable<MyRow> table={table} /> }
/// ```
#[function_component(DataTable)]
pub fn data_table<T: Clone + PartialEq + 'static>(props: &DataTableProps<T>) -> Html {
    // Clone the table handle for use in callbacks.
    let table = props.table.clone();

    // Create the global filter input callback.
    let on_global_filter = {
        let table = table.clone();
        Callback::from(move |e: InputEvent| {
            // Extract the input element from the event target.
            let target = e.target_dyn_into::<web_sys::HtmlInputElement>();
            if let Some(input) = target {
                // Apply the filter value to the table.
                table.set_global_filter(input.value());
            }
        })
    };

    html! {
        <div class={classes!("datatable-container", props.class.clone())}>
            {if props.show_global_filter {
                html! {
                    <div class="datatable-toolbar">
                        <input
                            type="text"
                            class="global-filter"
                            placeholder={props.filter_placeholder.clone()}
                            oninput={on_global_filter}
                        />
                    </div>
                }
            } else {
                html! {}
            }}

            <table class={classes!("datatable", props.table_class.clone())}>
                <TableHeader<T> table={table.clone()} />
                <TableBody<T> table={table.clone()} selectable={props.selectable} />
            </table>

            {if props.show_pagination {
                html! {
                    <Pagination<T> table={table.clone()} />
                }
            } else {
                html! {}
            }}
        </div>
    }
}
