//! Table header component.

use crate::hooks::use_table::UseTableHandle;
use yew::prelude::*;
use yew_datatable_core::prelude::SortDirection;

/// Props for the TableHeader component.
#[derive(Properties, Clone)]
pub struct TableHeaderProps<T: Clone + PartialEq + 'static> {
    /// The table handle from use_table hook.
    pub table: UseTableHandle<T>,

    /// Custom class for the thead element.
    #[prop_or_default]
    pub class: Classes,

    /// Custom class for th elements.
    #[prop_or_default]
    pub th_class: Classes,

    /// Whether to show sort indicators.
    #[prop_or(true)]
    pub show_sort_indicator: bool,
}

/// Compares `TableHeaderProps` by all fields, including the table handle.
impl<T: Clone + PartialEq + 'static> PartialEq for TableHeaderProps<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare the table handle for reactivity-aware equality.
        self.table == other.table
            // Compare all configuration fields.
            && self.class == other.class
            && self.th_class == other.th_class
            && self.show_sort_indicator == other.show_sort_indicator
    }
}

/// Table header component that renders column headers with sorting support.
#[function_component(TableHeader)]
pub fn table_header<T: Clone + PartialEq + 'static>(props: &TableHeaderProps<T>) -> Html {
    // Retrieve the ordered list of visible column identifiers.
    let column_ids = props.table.visible_column_ids();

    html! {
        <thead class={props.class.clone()}>
            <tr>
                {column_ids.iter().map(|column_id| {
                    // Resolve column metadata for the current header cell.
                    let header = props.table.get_column_header(column_id).unwrap_or_default();
                    let is_sortable = props.table.is_column_sortable(column_id);
                    let sort_direction = props.table.get_sort_direction(column_id);
                    let sort_index = props.table.get_sort_index(column_id);

                    // Create the click handler for toggling column sort.
                    let onclick = {
                        let table = props.table.clone();
                        let column_id = column_id.clone();
                        Callback::from(move |e: MouseEvent| {
                            if is_sortable {
                                // Detect shift-click for multi-column sorting.
                                let multi = e.shift_key();
                                table.toggle_sort(column_id.clone(), multi);
                            }
                        })
                    };

                    // Render the sort direction indicator if enabled.
                    let sort_indicator = if props.show_sort_indicator {
                        render_sort_indicator(sort_direction, sort_index)
                    } else {
                        html! {}
                    };

                    // Apply a pointer cursor class when the column is sortable.
                    let cursor_class = if is_sortable { "cursor-pointer" } else { "" };

                    html! {
                        <th
                            key={column_id.as_str().to_string()}
                            class={classes!(props.th_class.clone(), cursor_class)}
                            onclick={onclick}
                        >
                            <div class="flex items-center gap-1">
                                <span>{header}</span>
                                {sort_indicator}
                            </div>
                        </th>
                    }
                }).collect::<Html>()}
            </tr>
        </thead>
    }
}

/// Renders the visual sort direction indicator for a column header.
///
/// # Parameters
///
/// - `direction`: The current sort direction, if any.
/// - `index`: The sort priority index for multi-column sorting.
///
/// # Returns
///
/// - `Html`: The rendered sort indicator markup.
fn render_sort_indicator(direction: Option<SortDirection>, index: Option<usize>) -> Html {
    // Render the appropriate indicator based on sort direction.
    match direction {
        Some(SortDirection::Asc) => html! {
            <span class="sort-indicator">
                {"▲"}
                {index.map(|i| html! { <sub>{i + 1}</sub> }).unwrap_or_default()}
            </span>
        },
        Some(SortDirection::Desc) => html! {
            <span class="sort-indicator">
                {"▼"}
                {index.map(|i| html! { <sub>{i + 1}</sub> }).unwrap_or_default()}
            </span>
        },
        None => html! {},
    }
}
