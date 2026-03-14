//! Full-featured demonstration combining all features.

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::{ColumnId, SortDirection, use_table};

#[function_component(FullDemo)]
pub fn full_demo() -> Html {
    let columns = create_columns();
    let data = generate_sample_data();
    let table = use_table(columns, data, None);

    let all_column_ids: Vec<ColumnId> = vec![
        "id".into(),
        "first_name".into(),
        "last_name".into(),
        "age".into(),
        "email".into(),
        "department".into(),
        "salary".into(),
        "active".into(),
    ];

    let on_global_filter = {
        let table = table.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                table.set_global_filter(input.value());
            }
        })
    };

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

    let on_select_all = {
        let table = table.clone();
        Callback::from(move |_| table.select_all_rows())
    };

    let on_clear_selection = {
        let table = table.clone();
        Callback::from(move |_| table.clear_selection())
    };

    let on_reset = {
        let table = table.clone();
        Callback::from(move |_| table.reset())
    };

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();
    let selected_count = rows.iter().filter(|r| table.is_row_selected(&r.id)).count();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Full Feature Demo"}</h2>
                <p class="text-gray-600">
                    {"All features combined: sorting, filtering, pagination, selection, and column visibility."}
                </p>
            </div>

            // Toolbar
            <div class="flex flex-wrap items-center gap-4 mb-4 p-4 bg-gray-50 rounded-lg">
                <input
                    type="text"
                    placeholder="Search..."
                    oninput={on_global_filter}
                    class="px-3 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none"
                />
                <button
                    onclick={on_select_all}
                    class="px-3 py-2 bg-indigo-600 text-white text-sm rounded-md hover:bg-indigo-700 transition"
                >
                    {"Select All"}
                </button>
                <button
                    onclick={on_clear_selection}
                    class="px-3 py-2 bg-gray-200 text-gray-700 text-sm rounded-md hover:bg-gray-300 transition"
                >
                    {"Clear"}
                </button>
                <button
                    onclick={on_reset}
                    class="px-3 py-2 bg-red-100 text-red-700 text-sm rounded-md hover:bg-red-200 transition"
                >
                    {"Reset All"}
                </button>
                <span class="text-sm text-gray-600 ml-auto">
                    {format!("{} selected", selected_count)}
                </span>
            </div>

            // Column visibility
            <div class="flex flex-wrap gap-2 mb-4">
                <span class="text-sm font-medium text-gray-700 mr-2">{"Columns:"}</span>
                {all_column_ids.iter().map(|col_id| {
                    let is_visible = table.is_column_visible(col_id);
                    let header = table.get_column_header(col_id).unwrap_or_else(|| col_id.as_str().to_string());

                    let onclick = {
                        let table = table.clone();
                        let col_id = col_id.clone();
                        Callback::from(move |_| table.toggle_column_visibility(col_id.clone()))
                    };

                    html! {
                        <button
                            onclick={onclick}
                            class={classes!(
                                "px-2", "py-1", "text-xs", "rounded", "border", "transition",
                                if is_visible {
                                    "bg-green-50 border-green-200 text-green-700"
                                } else {
                                    "bg-gray-50 border-gray-200 text-gray-400 line-through"
                                }
                            )}
                        >
                            {header}
                        </button>
                    }
                }).collect::<Html>()}
            </div>

            // Table
            <div class="overflow-x-auto border rounded-lg">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-gray-50 border-b border-gray-200">
                            <th class="px-3 py-2 text-left w-10">{"✓"}</th>
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
                                            "px-3", "py-2", "text-left", "font-medium", "text-gray-700",
                                            is_sortable.then_some("cursor-pointer hover:bg-gray-100")
                                        )}
                                    >
                                        <span class="flex items-center gap-1">
                                            {header}
                                            <span class="text-indigo-500 text-xs">{sort_icon}</span>
                                            {sort_idx.map(|idx| html! {
                                                <span class="text-xs bg-indigo-100 text-indigo-600 px-1 rounded">
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
                                        if is_selected { "bg-indigo-50" } else if idx % 2 == 1 { "bg-gray-50/50" } else { "" },
                                        "hover:bg-gray-100"
                                    )}
                                >
                                    <td class="px-3 py-2">
                                        <input
                                            type="checkbox"
                                            checked={is_selected}
                                            class="w-4 h-4 text-indigo-600 rounded"
                                            onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                                        />
                                    </td>
                                    {column_ids.iter().map(|col_id| {
                                        let value = table.get_cell_value(&row.original, col_id).unwrap_or_default();
                                        html! {
                                            <td key={col_id.as_str().to_string()} class="px-3 py-2 text-gray-600">
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

            // Pagination
            <div class="flex flex-wrap items-center justify-between gap-4 mt-4">
                <span class="text-sm text-gray-500">
                    {format!("{} - {} of {}",
                        table.current_page() * table.page_size() + 1,
                        ((table.current_page() + 1) * table.page_size()).min(table.filtered_row_count()),
                        table.filtered_row_count()
                    )}
                </span>

                <div class="flex items-center gap-1">
                    <button
                        onclick={on_first}
                        disabled={!table.can_previous_page()}
                        class="px-2 py-1 text-sm border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {"««"}
                    </button>
                    <button
                        onclick={on_prev}
                        disabled={!table.can_previous_page()}
                        class="px-2 py-1 text-sm border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {"‹"}
                    </button>
                    <span class="px-3 py-1 text-sm">
                        {format!("{} / {}", table.current_page() + 1, table.page_count().max(1))}
                    </span>
                    <button
                        onclick={on_next}
                        disabled={!table.can_next_page()}
                        class="px-2 py-1 text-sm border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {"›"}
                    </button>
                    <button
                        onclick={on_last}
                        disabled={!table.can_next_page()}
                        class="px-2 py-1 text-sm border rounded hover:bg-gray-50 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        {"»»"}
                    </button>
                </div>
            </div>
        </div>
    }
}
