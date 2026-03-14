//! Filtering feature demonstration.

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::use_table;

#[function_component(FilteringDemo)]
pub fn filtering_demo() -> Html {
    let columns = create_columns();
    let data = generate_sample_data();
    let table = use_table(columns, data, None);

    let on_global_filter = {
        let table = table.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                table.set_global_filter(input.value());
            }
        })
    };

    let on_name_filter = {
        let table = table.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                table.set_column_filter("first_name", input.value());
            }
        })
    };

    let on_dept_filter = {
        let table = table.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                table.set_column_filter("department", input.value());
            }
        })
    };

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Filtering Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Demonstrates global and column-specific filtering."}
                </p>
            </div>

            <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">{"Global Search"}</label>
                    <input
                        type="text"
                        placeholder="Search all columns..."
                        oninput={on_global_filter}
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition"
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">{"Filter by First Name"}</label>
                    <input
                        type="text"
                        placeholder="Filter first name..."
                        oninput={on_name_filter}
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none transition"
                    />
                </div>
                <div>
                    <label class="block text-sm font-medium text-gray-700 mb-1">{"Filter by Department"}</label>
                    <input
                        type="text"
                        placeholder="Filter department..."
                        oninput={on_dept_filter}
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-purple-500 focus:border-purple-500 outline-none transition"
                    />
                </div>
            </div>

            <div class="bg-gray-50 rounded-lg p-3 mb-4 text-sm">
                <span class="font-medium text-gray-700">{"Results: "}</span>
                <span class="text-gray-600">{format!("{} of {} records", table.filtered_row_count(), table.total_row_count())}</span>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-gray-50 border-b-2 border-gray-200">
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
        </div>
    }
}
