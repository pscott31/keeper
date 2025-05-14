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

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ExpenseView {
    id: i64,
    description: String,
    supplier_name: String,
    supplier_id: i64,
    amount: f64,
    date: chrono::NaiveDate,
    submitter_id: i64,
    submitter_name: String,
    status: ExpenseStatus,
}

#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "TEXT"))]
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub enum ExpenseStatus {
    Pending,
    Approved,
    Paid,
}

#[server]
pub async fn load_data() -> Result<Vec<ExpenseView>, ServerFnError> {
    let db = crate::db_pool::use_db();

    match sqlx::query_as!(
        ExpenseView,
        "Select expenses.id as id,
                description,
                suppliers.name as supplier_name,
                supplier_id,
                amount,
                date as 'date: chrono::NaiveDate',
                submitter_id,
                users.name as submitter_name,
                status as 'status: ExpenseStatus'
        from expenses
        join suppliers on suppliers.id = expenses.supplier_id
        join users on users.id = expenses.submitter_id
        
        "
    )
    .fetch_all(&db)
    .await
    {
        Ok(expenses) => Ok(expenses),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
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
