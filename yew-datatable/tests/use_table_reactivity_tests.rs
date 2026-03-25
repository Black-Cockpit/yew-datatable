//! Reactivity integration tests for the `use_table` hook.
//!
//! These tests verify that parent-provided data changes are reflected by the
//! table handle without requiring an effect-based `set_data` synchronization.

use wasm_bindgen::JsCast;
use wasm_bindgen_test::wasm_bindgen_test;
use yew::prelude::*;
use yew_datatable::prelude::{ColumnDef, ColumnDefBuilder, use_table};

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone, PartialEq)]
struct TestRow {
    id: usize,
    label: String,
}

/// Builds the test columns used by the reactivity harness.
fn create_columns() -> Vec<ColumnDef<TestRow>> {
    vec![
        ColumnDefBuilder::new("id", "ID")
            .accessor(|row: &TestRow| row.id as i32)
            .build(),
        ColumnDefBuilder::new("label", "Label")
            .accessor(|row: &TestRow| row.label.clone())
            .build(),
    ]
}

#[function_component(ReactivityHarness)]
fn reactivity_harness() -> Html {
    // Initialize the parent-controlled rows with an empty dataset.
    let rows = use_state(Vec::<TestRow>::new);

    // Build a table handle directly from the current parent rows.
    let table = use_table(create_columns(), (*rows).clone(), None);

    // Create a callback that loads two rows into parent state.
    let on_load_two = {
        let rows = rows.clone();
        Callback::from(move |_| {
            rows.set(vec![
                TestRow {
                    id: 1,
                    label: "alpha".to_string(),
                },
                TestRow {
                    id: 2,
                    label: "beta".to_string(),
                },
            ]);
        })
    };

    // Create a callback that shrinks parent state to one row.
    let on_shrink_one = {
        let rows = rows.clone();
        Callback::from(move |_| {
            rows.set(vec![TestRow {
                id: 1,
                label: "alpha".to_string(),
            }]);
        })
    };

    // Create a callback that exercises existing table operations after sync.
    let on_page_size_one = {
        let table = table.clone();
        Callback::from(move |_| {
            table.set_page_size(1);
        })
    };

    // Collect values that expose parent state and table state together.
    let prop_count = rows.len();
    let visible_count = table.visible_rows().len();
    let total_count = table.total_row_count();
    let page_count = table.page_row_count();

    html! {
        <div>
            <button id="load-two" onclick={on_load_two}>{"load-two"}</button>
            <button id="shrink-one" onclick={on_shrink_one}>{"shrink-one"}</button>
            <button id="page-size-one" onclick={on_page_size_one}>{"page-size-one"}</button>

            <span id="prop-count">{prop_count}</span>
            <span id="visible-count">{visible_count}</span>
            <span id="total-count">{total_count}</span>
            <span id="page-count">{page_count}</span>
        </div>
    }
}

/// Waits one render cycle so state updates can be reflected in the DOM.
async fn wait_for_render_cycle() {
    // Yield to the browser event loop so Yew can flush updates.
    gloo::timers::future::TimeoutFuture::new(1).await;
}

/// Reads an element text content by element identifier.
fn text_by_id(document: &web_sys::Document, id: &str) -> String {
    // Locate the element by ID in the mounted test DOM.
    let element = document
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing element: {id}"));

    // Extract and return the text content as a string.
    element
        .text_content()
        .unwrap_or_else(|| panic!("missing text content: {id}"))
}

/// Clicks an element by identifier to trigger its callback.
fn click_by_id(document: &web_sys::Document, id: &str) {
    // Locate the element by ID and cast it to HtmlElement.
    let element = document
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("missing clickable element: {id}"))
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap_or_else(|_| panic!("element is not HtmlElement: {id}"));

    // Trigger a click to execute the callback wired in the component.
    element.click();
}

/// Validates that `use_table` synchronizes parent data changes without effect-based `set_data`.
#[wasm_bindgen_test]
async fn test_use_table_synchronizes_with_parent_data_without_effect_sync() {
    // Access the browser document used by wasm-bindgen-test.
    let document = web_sys::window()
        .expect("window should be available")
        .document()
        .expect("document should be available");

    // Create and mount a root element for rendering the Yew component.
    let root = document.create_element("div").expect("root element should be created");
    document
        .body()
        .expect("document body should exist")
        .append_child(&root)
        .expect("root should be appended");

    // Render the harness component.
    yew::Renderer::<ReactivityHarness>::with_root(root).render();
    wait_for_render_cycle().await;

    // Verify initial parity between parent data and table data.
    assert_eq!(text_by_id(&document, "prop-count"), "0");
    assert_eq!(text_by_id(&document, "visible-count"), "0");
    assert_eq!(text_by_id(&document, "total-count"), "0");

    // Load two rows in parent state and verify table data updates accordingly.
    click_by_id(&document, "load-two");
    wait_for_render_cycle().await;
    assert_eq!(text_by_id(&document, "prop-count"), "2");
    assert_eq!(text_by_id(&document, "visible-count"), "2");
    assert_eq!(text_by_id(&document, "total-count"), "2");
    assert_eq!(text_by_id(&document, "page-count"), "2");

    // Exercise a table operation and verify the table remains functional.
    click_by_id(&document, "page-size-one");
    wait_for_render_cycle().await;
    assert_eq!(text_by_id(&document, "total-count"), "2");
    assert_eq!(text_by_id(&document, "visible-count"), "1");
    assert_eq!(text_by_id(&document, "page-count"), "1");

    // Shrink parent data and verify the synchronized table view tracks it.
    click_by_id(&document, "shrink-one");
    wait_for_render_cycle().await;
    assert_eq!(text_by_id(&document, "prop-count"), "1");
    assert_eq!(text_by_id(&document, "visible-count"), "1");
    assert_eq!(text_by_id(&document, "total-count"), "1");
}
