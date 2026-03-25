//! Async/server-like data update demonstration.

use crate::data::{Person, create_columns, generate_sample_data};
use gloo::timers::callback::Timeout;
use yew::prelude::*;
use yew_datatable::prelude::use_table;

fn sample_slice(start: usize, end: usize) -> Vec<Person> {
    // Generate the deterministic sample dataset used by the example.
    let data = generate_sample_data();

    // Return only the requested slice range.
    data[start..end].to_vec()
}

#[function_component(AsyncDataDemo)]
pub fn async_data_demo() -> Html {
    // Track whether a simulated server request is in-flight.
    let is_loading = use_state(|| false);

    // Store the current parent-owned rows.
    let rows = use_state(Vec::<Person>::new);

    // Build the table from parent rows so hook sync behavior is visible.
    let table = use_table(create_columns(), (*rows).clone(), None);

    // Simulate an async request for the first page.
    let on_load_first = {
        let is_loading = is_loading.clone();
        let rows = rows.clone();
        Callback::from(move |_| {
            // Mark the request as in-flight.
            is_loading.set(true);

            // Schedule a delayed response that updates parent state.
            let is_loading = is_loading.clone();
            let rows = rows.clone();
            Timeout::new(700, move || {
                // Apply the returned rows and end the loading state.
                rows.set(sample_slice(0, 5));
                is_loading.set(false);
            })
            .forget();
        })
    };

    // Simulate an async request for a different page/filter result.
    let on_load_next = {
        let is_loading = is_loading.clone();
        let rows = rows.clone();
        Callback::from(move |_| {
            // Mark the request as in-flight.
            is_loading.set(true);

            // Schedule a delayed response with a different subset.
            let is_loading = is_loading.clone();
            let rows = rows.clone();
            Timeout::new(700, move || {
                // Apply the returned rows and end the loading state.
                rows.set(sample_slice(5, 10));
                is_loading.set(false);
            })
            .forget();
        })
    };

    // Simulate an async request that returns an empty dataset.
    let on_load_empty = {
        let is_loading = is_loading.clone();
        let rows = rows.clone();
        Callback::from(move |_| {
            // Mark the request as in-flight.
            is_loading.set(true);

            // Schedule a delayed response with no rows.
            let is_loading = is_loading.clone();
            let rows = rows.clone();
            Timeout::new(700, move || {
                // Apply the returned rows and end the loading state.
                rows.set(Vec::new());
                is_loading.set(false);
            })
            .forget();
        })
    };

    // Collect table view values for quick visual verification.
    let column_ids = table.visible_column_ids();
    let visible_rows = table.visible_rows();
    let prop_count = rows.len();
    let total_count = table.total_row_count();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Async Data Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Simulates server responses and shows that hook data stays in sync with parent rows."}
                </p>
                <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 text-sm text-blue-800">
                    <strong>{"How to verify:"}</strong>
                    <ul class="list-disc list-inside mt-2 space-y-1">
                        <li>{"Click a load button and wait for the simulated response."}</li>
                        <li>{"After each response, Parent Rows and Table Total should match."}</li>
                        <li>{"Switch between results to confirm no stale/empty mismatch frame."}</li>
                    </ul>
                </div>
            </div>

            <div class="flex flex-wrap items-center gap-3 mb-4">
                <button
                    onclick={on_load_first}
                    disabled={*is_loading}
                    class="px-3 py-2 bg-indigo-600 text-white text-sm rounded-md hover:bg-indigo-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"Load Server Page 1"}
                </button>
                <button
                    onclick={on_load_next}
                    disabled={*is_loading}
                    class="px-3 py-2 bg-purple-600 text-white text-sm rounded-md hover:bg-purple-700 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"Load Server Page 2"}
                </button>
                <button
                    onclick={on_load_empty}
                    disabled={*is_loading}
                    class="px-3 py-2 bg-gray-700 text-white text-sm rounded-md hover:bg-gray-800 transition disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {"Load Empty Result"}
                </button>
                <span class="text-sm text-gray-600">
                    {if *is_loading { "Loading..." } else { "Idle" }}
                </span>
            </div>

            <div class="flex gap-6 text-sm mb-4">
                <span class="font-medium text-gray-700">{format!("Parent Rows: {}", prop_count)}</span>
                <span class="font-medium text-gray-700">{format!("Table Total: {}", total_count)}</span>
                <span class="font-medium text-gray-700">{format!("Visible Rows: {}", visible_rows.len())}</span>
            </div>

            <div class="overflow-x-auto border rounded-lg">
                <table class="w-full text-sm">
                    <thead>
                        <tr class="bg-gray-50 border-b border-gray-200">
                            {column_ids.iter().map(|column_id| {
                                let header = table.get_column_header(column_id).unwrap_or_default();
                                html! {
                                    <th key={column_id.as_str().to_string()} class="px-4 py-3 text-left font-semibold text-gray-700">
                                        {header}
                                    </th>
                                }
                            }).collect::<Html>()}
                        </tr>
                    </thead>
                    <tbody>
                        {visible_rows.iter().enumerate().map(|(index, row)| {
                            html! {
                                <tr
                                    key={row.id.as_str().to_string()}
                                    class={classes!(
                                        "border-b", "border-gray-100",
                                        (index % 2 == 1).then_some("bg-gray-50/50")
                                    )}
                                >
                                    {column_ids.iter().map(|column_id| {
                                        let value = table.get_cell_value(&row.original, column_id).unwrap_or_default();
                                        html! {
                                            <td key={column_id.as_str().to_string()} class="px-4 py-3 text-gray-600">
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
