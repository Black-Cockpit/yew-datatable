//! Row selection feature demonstration.

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::use_table;

#[function_component(SelectionDemo)]
pub fn selection_demo() -> Html {
    let columns = create_columns();
    let data = generate_sample_data();
    let table = use_table(columns, data, None);

    let on_select_all = {
        let table = table.clone();
        Callback::from(move |_| table.select_all_rows())
    };

    let on_clear_selection = {
        let table = table.clone();
        Callback::from(move |_| table.clear_selection())
    };

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();

    let selected_count = rows.iter().filter(|r| table.is_row_selected(&r.id)).count();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Row Selection Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Demonstrates single and multi-row selection capabilities."}
                </p>
            </div>

            <div class="flex flex-wrap items-center gap-4 mb-6">
                <button
                    onclick={on_select_all}
                    class="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 transition"
                >
                    {"Select All"}
                </button>
                <button
                    onclick={on_clear_selection}
                    class="px-4 py-2 bg-gray-200 text-gray-700 rounded-md hover:bg-gray-300 transition"
                >
                    {"Clear Selection"}
                </button>
                <span class="text-sm text-gray-600">
                    {format!("{} row(s) selected", selected_count)}
                </span>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-gray-50 border-b-2 border-gray-200">
                            <th class="px-4 py-3 text-left font-semibold text-gray-700 w-12">
                                {"✓"}
                            </th>
                            {column_ids.iter().map(|col_id| {
                                let header = table.get_column_header(col_id).unwrap_or_default();
                                html! {
                                    <th key={col_id.as_str().to_string()} class="px-4 py-3 text-left font-semibold text-gray-700">
                                        {header}
                                    </th>
                                }
                            }).collect::<Html>()}
                        </tr>
                    </thead>
                    <tbody>
                        {rows.iter().enumerate().map(|(idx, row)| {
                            let row_id = row.id.clone();
                            let is_selected = table.is_row_selected(&row_id);

                            let onclick = {
                                let table = table.clone();
                                let row_id = row_id.clone();
                                Callback::from(move |_| table.toggle_row_selection(row_id.clone()))
                            };

                            html! {
                                <tr
                                    key={row_id.as_str().to_string()}
                                    onclick={onclick}
                                    class={classes!(
                                        "border-b", "border-gray-100", "cursor-pointer", "transition-colors",
                                        if is_selected { "bg-indigo-50 hover:bg-indigo-100" } else { "hover:bg-gray-50" },
                                        (idx % 2 == 1 && !is_selected).then_some("bg-gray-50/50")
                                    )}
                                >
                                    <td class="px-4 py-3">
                                        <div class={classes!(
                                            "w-5", "h-5", "rounded", "border-2", "flex", "items-center", "justify-center",
                                            if is_selected { "bg-indigo-600 border-indigo-600 text-white" } else { "border-gray-300" }
                                        )}>
                                            {is_selected.then(|| html!{ <span class="text-xs">{"✓"}</span> })}
                                        </div>
                                    </td>
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
        </div>
    }
}
