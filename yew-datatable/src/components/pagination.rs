//! Pagination component.

use crate::hooks::use_table::UseTableHandle;
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Props for the Pagination component.
#[derive(Properties, Clone)]
pub struct PaginationProps<T: Clone + PartialEq + 'static> {
    /// The table handle from use_table hook.
    pub table: UseTableHandle<T>,

    /// Custom class for the pagination container.
    #[prop_or_default]
    pub class: Classes,

    /// Custom class for buttons.
    #[prop_or_default]
    pub button_class: Classes,

    /// Custom class for disabled buttons.
    #[prop_or_default]
    pub disabled_class: Classes,

    /// Whether to show page size selector.
    #[prop_or(true)]
    pub show_page_size_selector: bool,

    /// Whether to show page info.
    #[prop_or(true)]
    pub show_page_info: bool,

    /// Page size options.
    #[prop_or_else(|| vec![10, 20, 30, 50, 100])]
    pub page_size_options: Vec<usize>,
}

/// Compares `PaginationProps` by all fields, including the table handle.
impl<T: Clone + PartialEq + 'static> PartialEq for PaginationProps<T> {
    fn eq(&self, other: &Self) -> bool {
        // Compare the table handle for reactivity-aware equality.
        self.table == other.table
            // Compare all configuration fields.
            && self.class == other.class
            && self.button_class == other.button_class
            && self.disabled_class == other.disabled_class
            && self.show_page_size_selector == other.show_page_size_selector
            && self.show_page_info == other.show_page_info
            && self.page_size_options == other.page_size_options
    }
}

/// Pagination component for navigating table pages.
#[function_component(Pagination)]
pub fn pagination<T: Clone + PartialEq + 'static>(props: &PaginationProps<T>) -> Html {
    // Retrieve pagination state from the table handle.
    let current_page = props.table.current_page();
    let page_count = props.table.page_count();
    let page_size = props.table.page_size();
    let total_rows = props.table.filtered_row_count();
    let can_previous = props.table.can_previous_page();
    let can_next = props.table.can_next_page();

    // Create the callback for navigating to the first page.
    let on_first = {
        let table = props.table.clone();
        Callback::from(move |_: MouseEvent| {
            table.go_to_page(0);
        })
    };

    // Create the callback for navigating to the previous page.
    let on_previous = {
        let table = props.table.clone();
        Callback::from(move |_: MouseEvent| {
            table.previous_page();
        })
    };

    // Create the callback for navigating to the next page.
    let on_next = {
        let table = props.table.clone();
        Callback::from(move |_: MouseEvent| {
            table.next_page();
        })
    };

    // Create the callback for navigating to the last page.
    let on_last = {
        let table = props.table.clone();
        let last_page = page_count.saturating_sub(1);
        Callback::from(move |_: MouseEvent| {
            table.go_to_page(last_page);
        })
    };

    // Create the callback for changing the page size.
    let on_page_size_change = {
        let table = props.table.clone();
        Callback::from(move |e: Event| {
            // Extract the select element from the event target.
            if let Some(target) = e.target() {
                if let Ok(select) = target.dyn_into::<web_sys::HtmlSelectElement>() {
                    // Parse the selected value and update the page size.
                    let value = select.value();
                    if let Ok(size) = value.parse::<usize>() {
                        table.set_page_size(size);
                    }
                }
            }
        })
    };

    // Create a helper closure for button class resolution.
    let button_class = |disabled: bool| {
        if disabled {
            classes!(props.button_class.clone(), props.disabled_class.clone())
        } else {
            props.button_class.clone()
        }
    };

    // Calculate the visible row range for the info display.
    let start_row = if total_rows == 0 {
        0
    } else {
        current_page * page_size + 1
    };
    let end_row = ((current_page + 1) * page_size).min(total_rows);

    html! {
        <div class={classes!("pagination", props.class.clone())}>
            {if props.show_page_size_selector {
                html! {
                    <div class="page-size-selector">
                        <label>{"Show "}</label>
                        <select onchange={on_page_size_change} value={page_size.to_string()}>
                            {props.page_size_options.iter().map(|&size| {
                                html! {
                                    <option
                                        value={size.to_string()}
                                        selected={size == page_size}
                                    >
                                        {size}
                                    </option>
                                }
                            }).collect::<Html>()}
                        </select>
                        <label>{" entries"}</label>
                    </div>
                }
            } else {
                html! {}
            }}

            <div class="page-navigation">
                <button
                    class={button_class(!can_previous)}
                    onclick={on_first}
                    disabled={!can_previous}
                >
                    {"⟪"}
                </button>
                <button
                    class={button_class(!can_previous)}
                    onclick={on_previous}
                    disabled={!can_previous}
                >
                    {"◀"}
                </button>
                <span class="page-info-inline">
                    {format!("Page {} of {}", current_page + 1, page_count.max(1))}
                </span>
                <button
                    class={button_class(!can_next)}
                    onclick={on_next}
                    disabled={!can_next}
                >
                    {"▶"}
                </button>
                <button
                    class={button_class(!can_next)}
                    onclick={on_last}
                    disabled={!can_next}
                >
                    {"⟫"}
                </button>
            </div>

            {if props.show_page_info {
                html! {
                    <div class="page-info">
                        {format!("Showing {} to {} of {} entries", start_row, end_row, total_rows)}
                    </div>
                }
            } else {
                html! {}
            }}
        </div>
    }
}
