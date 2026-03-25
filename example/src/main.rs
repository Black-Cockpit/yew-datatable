//! Example application demonstrating yew-datatable features.

mod data;
mod pages;

use crate::pages::async_data::AsyncDataDemo;
use crate::pages::column_features::ColumnFeaturesDemo;
use crate::pages::filtering::FilteringDemo;
use crate::pages::full_demo::FullDemo;
use crate::pages::pagination::PaginationDemo;
use crate::pages::selection::SelectionDemo;
use crate::pages::sorting::SortingDemo;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Page {
    FullDemo,
    Sorting,
    Filtering,
    Pagination,
    Selection,
    ColumnFeatures,
    AsyncData,
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component(App)]
fn app() -> Html {
    let current_page = use_state(|| Page::FullDemo);

    let nav_items = [
        (Page::FullDemo, "Full Demo"),
        (Page::Sorting, "Sorting"),
        (Page::Filtering, "Filtering"),
        (Page::Pagination, "Pagination"),
        (Page::Selection, "Selection"),
        (Page::ColumnFeatures, "Column Features"),
        (Page::AsyncData, "Async Data"),
    ];

    html! {
        <div class="min-h-screen bg-gray-100">
            // Header
            <header class="bg-gradient-to-r from-indigo-600 to-purple-600 text-white shadow-lg">
                <div class="max-w-7xl mx-auto px-4 py-6">
                    <h1 class="text-3xl font-bold">{"Yew DataTable"}</h1>
                    <p class="text-indigo-200 mt-1">{"A TanStack Table-inspired data table for Yew"}</p>
                </div>
            </header>

            // Navigation
            <nav class="bg-white shadow-sm border-b">
                <div class="max-w-7xl mx-auto px-4">
                    <div class="flex gap-1 overflow-x-auto py-2">
                        {nav_items.iter().map(|(page, label)| {
                            let is_active = *current_page == *page;
                            let onclick = {
                                let current_page = current_page.clone();
                                let page = page.clone();
                                Callback::from(move |_| current_page.set(page.clone()))
                            };

                            html! {
                                <button
                                    onclick={onclick}
                                    class={classes!(
                                        "px-4", "py-2", "text-sm", "font-medium", "rounded-md", "transition", "whitespace-nowrap",
                                        if is_active {
                                            "bg-indigo-600 text-white"
                                        } else {
                                            "text-gray-600 hover:bg-gray-100"
                                        }
                                    )}
                                >
                                    {*label}
                                </button>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>
            </nav>

            // Main content
            <main class="max-w-7xl mx-auto px-4 py-8">
                {match *current_page {
                    Page::FullDemo => html! { <FullDemo /> },
                    Page::Sorting => html! { <SortingDemo /> },
                    Page::Filtering => html! { <FilteringDemo /> },
                    Page::Pagination => html! { <PaginationDemo /> },
                    Page::Selection => html! { <SelectionDemo /> },
                    Page::ColumnFeatures => html! { <ColumnFeaturesDemo /> },
                    Page::AsyncData => html! { <AsyncDataDemo /> },
                }}
            </main>

            // Footer
            <footer class="text-center py-8 text-gray-400 text-sm">
                {"Built with Yew and yew-datatable • "}
                <a href="https://github.com/Black-Cockpit/yew-datatable" class="text-indigo-500 hover:underline">
                    {"GitHub"}
                </a>
            </footer>
        </div>
    }
}
