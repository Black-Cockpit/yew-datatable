//! Sorting feature demonstration.

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::{SortDirection, use_table};

#[function_component(SortingDemo)]
pub fn sorting_demo() -> Html {
    let columns = create_columns();
    let data = generate_sample_data();
    let table = use_table(columns, data, None);

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Sorting Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Demonstrates single and multi-column sorting capabilities."}
                </p>
                <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 text-sm text-blue-800">
                    <strong>{"Instructions:"}</strong>
                    <ul class="list-disc list-inside mt-2 space-y-1">
                        <li>{"Click any column header to sort ascending"}</li>
                        <li>{"Click again to sort descending"}</li>
                        <li>{"Click a third time to clear sort"}</li>
                        <li>{"Hold Shift and click to add multi-column sort"}</li>
                    </ul>
                </div>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-gray-50 border-b-2 border-gray-200">
                            {column_ids.iter().map(|col_id| {
                                let header = table.get_column_header(col_id).unwrap_or_default();
                                let sort_dir = table.get_sort_direction(col_id);
                                let sort_idx = table.get_sort_index(col_id);
                                let is_sortable = table.is_column_sortable(col_id);

                                let onclick = {
                                    let table = table.clone();
                                    let col_id = col_id.clone();
                                    Callback::from(move |e: MouseEvent| {
                                        if is_sortable {
                                            table.toggle_sort(col_id.clone(), e.shift_key());
                                        }
                                    })
                                };

                                let sort_icon = match sort_dir {
                                    Some(SortDirection::Asc) => "↑",
                                    Some(SortDirection::Desc) => "↓",
                                    None => "",
                                };

                                html! {
                                    <th
                                        key={col_id.as_str().to_string()}
                                        onclick={onclick}
                                        class={classes!(
                                            "px-4", "py-3", "text-left", "font-semibold", "text-gray-700",
                                            is_sortable.then_some("cursor-pointer hover:bg-gray-100")
                                        )}
                                    >
                                        <span class="flex items-center gap-1">
                                            {header}
                                            <span class="text-indigo-500">{sort_icon}</span>
                                            {sort_idx.map(|idx| html! {
                                                <span class="text-xs bg-indigo-100 text-indigo-700 px-1.5 py-0.5 rounded-full ml-1">
                                                    {idx + 1}
                                                </span>
                                            })}
                                        </span>
                                    </th>
                                }
                            }).collect::<Html>()}
                        </tr>
                    </thead>
                    <tbody>
                        {rows.iter().enumerate().map(|(idx, row)| {
                            html! {
                                <tr
                                    key={row.id.as_str().to_string()}
                                    class={classes!(
                                        "border-b", "border-gray-100",
                                        (idx % 2 == 1).then_some("bg-gray-50/50")
                                    )}
                                >
                                    {column_ids.iter().map(|col_id| {
                                        let value = table.get_cell_value(&row.original, col_id).unwrap_or_default();
                                        html! {
                                            <td key={col_id.as_str().to_string()} class="px-4 py-3 text-gray-600">
                                                {value}
                                            </td>
                                        }
                                    }).collect::<Html>()}
                                </tr>
                            }
                        }).collect::<Html>()}
                    </tbody>
                </table>
            </div>

            <div class="mt-4 text-sm text-gray-500">
                {format!("Showing {} rows", rows.len())}
            </div>
        </div>
    }
}
