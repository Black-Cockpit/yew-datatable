//! Pagination feature demonstration.

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::use_table;

#[function_component(PaginationDemo)]
pub fn pagination_demo() -> Html {
    let columns = create_columns();
    let data = generate_sample_data();
    let table = use_table(columns, data, None);

    let on_first = {
        let table = table.clone();
        Callback::from(move |_| table.go_to_page(0))
    };
    let on_prev = {
        let table = table.clone();
        Callback::from(move |_| table.previous_page())
    };
    let on_next = {
        let table = table.clone();
        Callback::from(move |_| table.next_page())
    };
    let on_last = {
        let table = table.clone();
        let last = table.page_count().saturating_sub(1);
        Callback::from(move |_| table.go_to_page(last))
    };

    let on_page_size_5 = {
        let table = table.clone();
        Callback::from(move |_| table.set_page_size(5))
    };
    let on_page_size_10 = {
        let table = table.clone();
        Callback::from(move |_| table.set_page_size(10))
    };
    let on_page_size_25 = {
        let table = table.clone();
        Callback::from(move |_| table.set_page_size(25))
    };
    let on_page_size_50 = {
        let table = table.clone();
        Callback::from(move |_| table.set_page_size(50))
    };

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();
    let page_size = table.page_size();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Pagination Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Demonstrates pagination with configurable page sizes."}
                </p>
            </div>

            <div class="flex flex-wrap items-center gap-4 mb-6">
                <span class="text-sm font-medium text-gray-700">{"Page Size:"}</span>
                <div class="flex gap-2">
                    {[5, 10, 25, 50].iter().map(|&size| {
                        let onclick = match size {
                            5 => on_page_size_5.clone(),
                            10 => on_page_size_10.clone(),
                            25 => on_page_size_25.clone(),
                            _ => on_page_size_50.clone(),
                        };
                        let is_active = page_size == size;
                        html! {
                            <button
                                onclick={onclick}
                                class={classes!(
                                    "px-3", "py-1.5", "text-sm", "rounded-md", "transition",
                                    if is_active {
                                        "bg-indigo-600 text-white"
                                    } else {
                                        "bg-gray-100 text-gray-700 hover:bg-gray-200"
                                    }
                                )}
                            >
                                {size}
                            </button>
                        }
                    }).collect::<Html>()}
                </div>
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

            <div class="flex flex-wrap items-center justify-between gap-4 mt-6 pt-4 border-t border-gray-200">
                <span class="text-sm text-gray-500">
                    {format!("Showing {} to {} of {} entries",
                        table.current_page() * table.page_size() + 1,
                        ((table.current_page() + 1) * table.page_size()).min(table.filtered_row_count()),
                        table.filtered_row_count()
                    )}
                </span>

                <div class="flex items-center gap-2">
                    <button
                        onclick={on_first}
                        disabled={!table.can_previous_page()}
                        class="px-3 py-1.5 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition"
                    >
                        {"« First"}
                    </button>
                    <button
                        onclick={on_prev}
                        disabled={!table.can_previous_page()}
                        class="px-3 py-1.5 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition"
                    >
                        {"‹ Prev"}
                    </button>
                    <span class="px-4 py-1.5 text-sm bg-indigo-50 text-indigo-700 rounded-md font-medium">
                        {format!("Page {} of {}", table.current_page() + 1, table.page_count().max(1))}
                    </span>
                    <button
                        onclick={on_next}
                        disabled={!table.can_next_page()}
                        class="px-3 py-1.5 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition"
                    >
                        {"Next ›"}
                    </button>
                    <button
                        onclick={on_last}
                        disabled={!table.can_next_page()}
                        class="px-3 py-1.5 text-sm border border-gray-300 rounded-md hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed transition"
                    >
                        {"Last »"}
                    </button>
                </div>
            </div>
        </div>
    }
}
