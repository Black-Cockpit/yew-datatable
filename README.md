# yew-datatable

[![CI](https://github.com/Black-Cockpit/yew-datatable/workflows/CI/badge.svg)](https://github.com/Black-Cockpit/yew-datatable/actions)
[![Crates.io](https://img.shields.io/crates/v/yew-datatable.svg)](https://crates.io/crates/yew-datatable)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A fully-featured, headless data table library for [Yew](https://yew.rs/) applications, inspired by [TanStack Table v8](https://tanstack.com/table/latest). Provides compile-time type safety, a modular feature set, and a familiar hook-based API for building performant data-driven interfaces in Rust and WebAssembly.

<div align="center">
  <img src="https://raw.githubusercontent.com/Black-Cockpit/yew-datatable/master/assets/yew_data_table.png" alt="yew-datatable" />
</div>

## Overview

yew-datatable separates the headless table engine (`yew-datatable-core`) from the Yew UI layer (`yew-datatable`), letting you use the engine standalone or with the provided Yew components and hooks.

- **Headless Core** — Pure logic, no UI assumptions; bring your own markup and styling
- **Type-Safe Columns** — Compile-time accessor guarantees with generic `ColumnDef<T>`
- **Full Pipeline** — Rows flow through filter → sort → group → expand → paginate automatically
- **Hook-Based API** — `use_table` hook mirrors the TanStack `useReactTable` pattern
- **100% Safe Rust** — No `unsafe` blocks, memory-safe by construction

## Features

| Category | Capabilities |
|----------|-------------|
| **Sorting** | Single and multi-column, stable sort, custom comparators, natural ordering |
| **Filtering** | Column filters, global search, 16 built-in filter functions, custom functions |
| **Pagination** | Client-side and server-side modes, configurable page sizes, full navigation |
| **Selection** | Single-row, multi-row, select-all, toggle, row-click selection |
| **Visibility** | Show/hide columns, default visibility, bulk operations |
| **Ordering** | Reorder columns programmatically with move, swap, before/after |
| **Pinning** | Pin columns to left or right with automatic reordering |
| **Sizing** | Column widths with min/max constraints, resize lifecycle |
| **Expansion** | Expandable rows, tree data, auto-expand depth, expand-all |
| **Grouping** | Group by column values, multi-level grouping, aggregation display |
| **Aggregation** | Sum, mean, min, max, count, unique count, median, first, last |

## Installation

```toml
[dependencies]
yew-datatable = "0.1"
```

For the headless engine only:

```toml
[dependencies]
yew-datatable-core = "0.1"
```

## Quick Start

```rust
use yew::prelude::*;
use yew_datatable::prelude::*;

#[derive(Clone, PartialEq)]
struct Person { name: String, age: u32 }

#[function_component(MyTable)]
fn my_table() -> Html {
    let columns = vec![
        ColumnDefBuilder::new("name", "Name")
            .accessor(|p: &Person| p.name.clone())
            .build(),
        ColumnDefBuilder::new("age", "Age")
            .accessor(|p: &Person| p.age as i32)
            .build(),
    ];

    let data = vec![
        Person { name: "Alice".into(), age: 30 },
        Person { name: "Bob".into(), age: 25 },
    ];

    let table = use_table(columns, data, None);

    html! {
        <div>
            // Sortable headers — click to sort, shift-click for multi-sort
            <table>
                <thead><tr>
                    {table.visible_column_ids().iter().map(|id| {
                        let t = table.clone(); let id = id.clone();
                        html! { <th onclick={Callback::from(move |e: MouseEvent| {
                            t.toggle_sort(id.clone(), e.shift_key());
                        })}>{table.get_column_header(&id).unwrap_or_default()}</th> }
                    }).collect::<Html>()}
                </tr></thead>
                <tbody>
                    {table.visible_rows().iter().map(|row| html! {
                        <tr key={row.id.as_str().to_string()}>
                            {table.visible_column_ids().iter().map(|id| html! {
                                <td>{table.get_cell_value(&row.original, id).unwrap_or_default()}</td>
                            }).collect::<Html>()}
                        </tr>
                    }).collect::<Html>()}
                </tbody>
            </table>
            // Pagination
            <button disabled={!table.can_previous_page()}
                onclick={let t=table.clone(); Callback::from(move |_| t.previous_page())}>
                {"← Prev"}</button>
            <span>{format!("Page {} / {}", table.current_page()+1, table.page_count())}</span>
            <button disabled={!table.can_next_page()}
                onclick={let t=table.clone(); Callback::from(move |_| t.next_page())}>
                {"Next →"}</button>
        </div>
    }
}
```

## Architecture

The library follows a two-crate design:

**`yew-datatable-core`** — The headless engine with zero UI dependencies. It provides the `DataTable<T>` coordinator, `DataTableRowModel<T>` pipeline, feature state types (`SortingState`, `FilterState`, `PaginationState`, etc.), and column definitions with type-safe accessors. Each feature lives in its own module under `features/` with dedicated state, functions, and prelude re-exports.

**`yew-datatable`** — The Yew integration layer. It provides the `use_table` hook for reactive state management and pre-built components (`DataTable`, `TableHeader`, `TableBody`, `Pagination`) that wire into the core engine. All public types from both crates are available through `yew_datatable::prelude::*`.

### TanStack Table Mapping

| TanStack Concept | yew-datatable Equivalent |
|---|---|
| `useReactTable` | `use_table` hook |
| `ColumnDef` | `ColumnDef<T>` / `ColumnDefBuilder` |
| `accessorFn` | `.accessor()` builder method |
| `getCoreRowModel` | `DataTableRowModel::process()` |
| `getSortedRowModel` | Built into the pipeline |
| `getFilteredRowModel` | Built into the pipeline |
| `getPaginationRowModel` | Built into the pipeline |
| `flexRender` | Custom cell rendering via `Callback<CellRenderContext<T>, Html>` |

## Examples

See the [`example/`](example/) directory for a full working demonstration with sorting, filtering, pagination, selection, column features, and a combined full-demo page.

```bash
cd example
trunk serve
```

## Testing

```bash
cargo test --package yew-datatable-core --target x86_64-unknown-linux-gnu
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before submitting a pull request.

## License

Distributed under the [MIT](LICENSE) license.
