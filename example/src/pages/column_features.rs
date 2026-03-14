//! Column features demonstration (visibility, ordering).

use crate::data::{create_columns, generate_sample_data};
use yew::prelude::*;
use yew_datatable::prelude::{ColumnId, use_table};

#[function_component(ColumnFeaturesDemo)]
pub fn column_features_demo() -> Html {
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

    let column_ids = table.visible_column_ids();
    let rows = table.visible_rows();

    html! {
        <div class="bg-white rounded-xl shadow-md p-6">
            <div class="mb-6">
                <h2 class="text-2xl font-bold text-gray-800 mb-2">{"Column Features Demo"}</h2>
                <p class="text-gray-600 mb-4">
                    {"Demonstrates column visibility toggling."}
                </p>
            </div>

            <div class="mb-6">
                <h3 class="text-lg font-semibold text-gray-700 mb-3">{"Toggle Columns"}</h3>
                <div class="flex flex-wrap gap-2">
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
                                    "px-3", "py-1.5", "text-sm", "rounded-md", "border", "transition",
                                    if is_visible {
                                        "bg-green-100 border-green-300 text-green-700"
                                    } else {
                                        "bg-gray-100 border-gray-300 text-gray-500 line-through"
                                    }
                                )}
                            >
                                {header}
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

            <div class="mt-4 text-sm text-gray-500">
                {format!("Showing {} of {} columns", column_ids.len(), all_column_ids.len())}
            </div>
        </div>
    }
}
