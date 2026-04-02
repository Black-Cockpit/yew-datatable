//! Props `PartialEq` integration tests.
//!
//! These tests verify that components re-render when the `table` handle
//! changes (data mutation, sorting, pagination) and when the `render_cell`
//! callback is present. They confirm the fix for the bug where custom
//! `PartialEq` implementations excluded the `table` field, preventing
//! Yew from detecting prop changes.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::prelude::*;
use yew_datatable::prelude::{CellRenderContext, ColumnDef, ColumnDefBuilder, TableBody, TableHeader, use_table};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

/// Test row model used across all tests.
#[derive(Clone, PartialEq)]
struct TestRow {
    /// Unique identifier for the row.
    id: usize,

    /// Display label for the row.
    label: String,
}

/// Builds the column definitions shared by all test harnesses.
fn create_columns() -> Vec<ColumnDef<TestRow>> {
    vec![
        ColumnDefBuilder::new("id", "ID")
            .accessor(|row: &TestRow| row.id as i32)
            .sortable(true)
            .build(),
        ColumnDefBuilder::new("label", "Label")
            .accessor(|row: &TestRow| row.label.clone())
            .sortable(true)
            .build(),
    ]
}

/// Builds the initial dataset used by the test harness.
fn initial_rows() -> Vec<TestRow> {
    vec![
        TestRow {
            id: 1,
            label: "alpha".to_string(),
        },
        TestRow {
            id: 2,
            label: "beta".to_string(),
        },
        TestRow {
            id: 3,
            label: "gamma".to_string(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Harness: TableBody re-renders on data change
// ---------------------------------------------------------------------------

/// Harness that renders a `TableBody` and exposes buttons to mutate data.
#[function_component(TableBodyDataHarness)]
fn table_body_data_harness() -> Html {
    // Initialize the parent-controlled rows.
    let rows = use_state(initial_rows);

    // Build a table handle from the current parent rows.
    let table = use_table(create_columns(), (*rows).clone(), None);

    // Create a callback that replaces the dataset.
    let on_replace = {
        let rows = rows.clone();
        Callback::from(move |_| {
            rows.set(vec![
                TestRow {
                    id: 10,
                    label: "delta".to_string(),
                },
                TestRow {
                    id: 20,
                    label: "epsilon".to_string(),
                },
            ]);
        })
    };

    // Expose the visible row count for assertion.
    let visible_count = table.visible_rows().len();

    html! {
        <div>
            <button id="replace-data" onclick={on_replace}>{"replace"}</button>
            <span id="body-visible-count">{visible_count}</span>
            <table>
                <TableBody<TestRow> table={table.clone()} />
            </table>
        </div>
    }
}

/// Validates that `TableBody` re-renders when the underlying data changes.
#[wasm_bindgen_test]
async fn test_table_body_rerenders_on_data_change() {
    // Access the browser document.
    let document = document();

    // Mount the harness component.
    let root = mount(&document);
    yew::Renderer::<TableBodyDataHarness>::with_root(root).render();
    wait().await;

    // Verify initial row count.
    assert_eq!(text(&document, "body-visible-count"), "3");

    // Count tbody rows in the DOM.
    let initial_rows = document.query_selector_all("tbody tr").unwrap().length();
    assert_eq!(initial_rows, 3);

    // Replace data and verify the component re-renders.
    click(&document, "replace-data");
    wait().await;

    // Verify updated row count.
    assert_eq!(text(&document, "body-visible-count"), "2");

    // Verify DOM rows updated.
    let updated_rows = document.query_selector_all("tbody tr").unwrap().length();
    assert_eq!(updated_rows, 2);
}

// ---------------------------------------------------------------------------
// Harness: TableBody re-renders on sort change
// ---------------------------------------------------------------------------

/// Harness that renders a `TableBody` and exposes a button to toggle sort.
#[function_component(TableBodySortHarness)]
fn table_body_sort_harness() -> Html {
    // Build a table handle from static rows.
    let table = use_table(create_columns(), initial_rows(), None);

    // Create a callback that toggles sort on the label column.
    let on_sort = {
        let table = table.clone();
        Callback::from(move |_| {
            table.toggle_sort("label", false);
        })
    };

    // Collect the first visible row label for assertion.
    let first_label = table
        .visible_rows()
        .first()
        .map(|r| r.original.label.clone())
        .unwrap_or_default();

    html! {
        <div>
            <button id="toggle-sort" onclick={on_sort}>{"sort"}</button>
            <span id="first-label">{first_label}</span>
            <table>
                <TableBody<TestRow> table={table.clone()} />
            </table>
        </div>
    }
}

/// Validates that `TableBody` re-renders when sorting changes.
#[wasm_bindgen_test]
async fn test_table_body_rerenders_on_sort_change() {
    // Access the browser document.
    let document = document();

    // Mount the harness component.
    let root = mount(&document);
    yew::Renderer::<TableBodySortHarness>::with_root(root).render();
    wait().await;

    // Verify initial order (insertion order).
    assert_eq!(text(&document, "first-label"), "alpha");

    // Toggle sort ascending on label column.
    click(&document, "toggle-sort");
    wait().await;

    // After ascending sort, first label should be "alpha".
    let label_asc = text(&document, "first-label");
    assert_eq!(label_asc, "alpha");

    // Toggle sort again (descending).
    click(&document, "toggle-sort");
    wait().await;

    // After descending sort, first label should be "gamma".
    assert_eq!(text(&document, "first-label"), "gamma");
}

// ---------------------------------------------------------------------------
// Harness: TableBody re-renders on page change
// ---------------------------------------------------------------------------

/// Harness that renders a `TableBody` with page size 2 and exposes pagination.
#[function_component(TableBodyPageHarness)]
fn table_body_page_harness() -> Html {
    // Build a table handle from static rows.
    let table = use_table(create_columns(), initial_rows(), None);

    // Set initial page size to 2.
    let initialized = use_state(|| false);
    if !*initialized {
        table.set_page_size(2);
        initialized.set(true);
    }

    // Create a callback that goes to the next page.
    let on_next = {
        let table = table.clone();
        Callback::from(move |_| {
            table.next_page();
        })
    };

    // Expose the current page and visible row count.
    let current_page = table.current_page();
    let visible_count = table.visible_rows().len();

    html! {
        <div>
            <button id="next-page" onclick={on_next}>{"next"}</button>
            <span id="current-page">{current_page}</span>
            <span id="page-visible-count">{visible_count}</span>
            <table>
                <TableBody<TestRow> table={table.clone()} />
            </table>
        </div>
    }
}

/// Validates that `TableBody` re-renders when the page changes.
#[wasm_bindgen_test]
async fn test_table_body_rerenders_on_page_change() {
    // Access the browser document.
    let document = document();

    // Mount the harness component.
    let root = mount(&document);
    yew::Renderer::<TableBodyPageHarness>::with_root(root).render();
    wait().await;

    // Verify page 0 shows 2 rows.
    assert_eq!(text(&document, "current-page"), "0");
    assert_eq!(text(&document, "page-visible-count"), "2");

    // Go to next page.
    click(&document, "next-page");
    wait().await;

    // Verify page 1 shows 1 row.
    assert_eq!(text(&document, "current-page"), "1");
    assert_eq!(text(&document, "page-visible-count"), "1");
}

// ---------------------------------------------------------------------------
// Harness: TableBody re-renders with render_cell callback
// ---------------------------------------------------------------------------

/// Harness that uses a `render_cell` callback capturing changing state.
#[function_component(RenderCellHarness)]
fn render_cell_harness() -> Html {
    // Track a prefix that changes on button click.
    let prefix = use_state(|| "v1".to_string());

    // Build a table handle from static rows.
    let table = use_table(create_columns(), initial_rows(), None);

    // Create a callback that changes the prefix.
    let on_change_prefix = {
        let prefix = prefix.clone();
        Callback::from(move |_| {
            prefix.set("v2".to_string());
        })
    };

    // Build the render_cell callback that captures the prefix.
    let current_prefix = (*prefix).clone();
    let render_cell = Callback::from(move |ctx: CellRenderContext<TestRow>| -> Html {
        // Prefix the cell value with the current prefix.
        html! { <span class="custom-cell">{format!("{}:{}", current_prefix, ctx.value)}</span> }
    });

    html! {
        <div>
            <button id="change-prefix" onclick={on_change_prefix}>{"change"}</button>
            <span id="prefix-value">{(*prefix).clone()}</span>
            <table>
                <TableBody<TestRow>
                    table={table.clone()}
                    render_cell={Some(render_cell)}
                />
            </table>
        </div>
    }
}

/// Validates that `TableBody` re-renders when `render_cell` captures changed state.
#[wasm_bindgen_test]
async fn test_table_body_rerenders_with_render_cell_callback() {
    // Access the browser document.
    let document = document();

    // Mount the harness component.
    let root = mount(&document);
    yew::Renderer::<RenderCellHarness>::with_root(root).render();
    wait().await;

    // Verify initial prefix is rendered in custom cells.
    let first_cell = document
        .query_selector("span.custom-cell")
        .unwrap()
        .expect("custom cell should exist");
    let initial_text = first_cell.text_content().unwrap_or_default();
    assert!(
        initial_text.starts_with("v1:"),
        "expected cell to start with 'v1:', got '{initial_text}'"
    );

    // Change the prefix.
    click(&document, "change-prefix");
    wait().await;

    // Verify the cell now reflects the new prefix.
    let updated_cell = document
        .query_selector("span.custom-cell")
        .unwrap()
        .expect("custom cell should exist after update");
    let updated_text = updated_cell.text_content().unwrap_or_default();
    assert!(
        updated_text.starts_with("v2:"),
        "expected cell to start with 'v2:', got '{updated_text}'"
    );
}

// ---------------------------------------------------------------------------
// Harness: TableHeader re-renders on sort change
// ---------------------------------------------------------------------------

/// Harness that renders a `TableHeader` and verifies it updates on sort.
#[function_component(TableHeaderSortHarness)]
fn table_header_sort_harness() -> Html {
    // Build a table handle from static rows.
    let table = use_table(create_columns(), initial_rows(), None);

    // Create a callback that toggles sort on the id column.
    let on_sort = {
        let table = table.clone();
        Callback::from(move |_| {
            table.toggle_sort("id", false);
        })
    };

    // Expose the sort direction for the id column.
    let sort_dir = table
        .get_sort_direction(&"id".into())
        .map(|d| format!("{d:?}"))
        .unwrap_or_else(|| "none".to_string());

    html! {
        <div>
            <button id="toggle-header-sort" onclick={on_sort}>{"sort"}</button>
            <span id="sort-direction">{sort_dir}</span>
            <table>
                <TableHeader<TestRow> table={table.clone()} />
            </table>
        </div>
    }
}

/// Validates that `TableHeader` re-renders when sorting changes.
#[wasm_bindgen_test]
async fn test_table_header_rerenders_on_sort_change() {
    // Access the browser document.
    let document = document();

    // Mount the harness component.
    let root = mount(&document);
    yew::Renderer::<TableHeaderSortHarness>::with_root(root).render();
    wait().await;

    // Verify no initial sort direction.
    assert_eq!(text(&document, "sort-direction"), "none");

    // Toggle sort.
    click(&document, "toggle-header-sort");
    wait().await;

    // Verify ascending sort direction.
    let dir = text(&document, "sort-direction");
    assert_ne!(dir, "none", "sort direction should change after toggle");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns the browser document.
fn document() -> web_sys::Document {
    // Access the window and document objects.
    web_sys::window()
        .expect("window should be available")
        .document()
        .expect("document should be available")
}

/// Creates and mounts a root element for rendering.
fn mount(document: &web_sys::Document) -> web_sys::Element {
    // Create a container div element.
    let root = document.create_element("div").expect("root element should be created");

    // Append the container to the document body.
    document
        .body()
        .expect("document body should exist")
        .append_child(&root)
        .expect("root should be appended");

    root
}

/// Waits one render cycle so state updates are reflected in the DOM.
async fn wait() {
    // Yield to the browser event loop so Yew can flush updates.
    gloo::timers::future::TimeoutFuture::new(1).await;
}

/// Reads the text content of an element by its ID.
fn text(document: &web_sys::Document, id: &str) -> String {
    // Locate the element by ID.
    let element = document
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing element: {id}"));

    // Extract and return its text content.
    element
        .text_content()
        .unwrap_or_else(|| panic!("missing text content: {id}"))
}

/// Clicks an element by its ID to trigger its callback.
fn click(document: &web_sys::Document, id: &str) {
    // Locate the element and cast it to HtmlElement.
    let element = document
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing clickable element: {id}"))
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap_or_else(|_| panic!("element is not HtmlElement: {id}"));

    // Trigger a click event.
    element.click();
}
