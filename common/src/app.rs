use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/keeper.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ExpenseView {
    id: u64,
    description: String,
    supplier_name: String,
    supplier_id: u64,
    amount: f64,
    date: chrono::NaiveDate,
    submitter_id: u64,
    submitter_name: String,
    status: ExpenseStatus,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
enum ExpenseStatus {
    Pending,
    Approved,
    Paid,
}

async fn load_data() -> Result<Vec<ExpenseView>, String> {
    // Simulate a network request
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Return some dummy data
    Ok(vec![
        ExpenseView {
            id: 1,
            description: "Lunch with client".to_string(),
            supplier_name: "Alice".to_string(),
            supplier_id: 1,
            amount: 30.0,
            date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
            submitter_id: 1,
            submitter_name: "Bob".to_string(),
            status: ExpenseStatus::Pending,
        },
        // Add more dummy data as needed
    ])
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    let data = OnceResource::new(load_data());

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <div class="grid-table">
            <div class="grid-header">Date</div>
            <div class="grid-header">Supplier</div>
            <div class="grid-header">Submitted By</div>
            <div class="grid-header">Amount</div>

            <Suspense fallback=move || {
                view! { cx, <p>"Loading..."</p> }
            }>
                {move || {
                    data.get()
                        .map(|data| {
                            view! {
                                <For
                                    // todo: unwrap
                                    each=move || data.clone().unwrap()
                                    key=|exp| exp.id
                                    children=|exp| {
                                        view! {
                                            <div class="grid-cell">
                                                {exp.date.format("%Y-%m-%d").to_string()}
                                            </div>
                                            <div class="grid-cell">{exp.supplier_name}</div>
                                            <div class="grid-cell">{exp.submitter_name}</div>
                                            <div class="grid-cell">{exp.amount}</div>
                                        }
                                    }
                                />
                            }
                        })
                }}

            </Suspense>
        </div>
    }
}
