mod kitchen_sink;

use crate::error_template::{AppError, ErrorTemplate};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use kitchen_sink::KitchenSink;

use crate::components::*;

pub mod error_template;
pub mod components;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/start-axum-workspace.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            
                <nav>
                <menu>
                    <li>
                        <A href="/">"Home"</A>
                    </li>
                    <li>
                        <A href="/avatar">"avatar"</A>
                    </li>
                    <li>
                        <A href="/badge">"badge"</A>
                    </li>
                    <li>
                        <A href="/button">"button"</A>
                    </li>
                    <li>
                        <A href="/dialog">"dialog"</A>
                    </li>
                    <li>
                        <A href="drawer">"drawer"</A>
                    </li>
                    <li>
                        <A href="loading">"loading"</A>
                    </li>
                    <li>
                        <A href="message">"message"</A>
                    </li>
                    <li>
                        <A href="pagination">"pagination"</A>
                    </li>
                    <li>
                        <A href="popover">"popover"</A>
                    </li>
                    <li>
                        <A href="qrcode">"qrcode"</A>
                    </li>
                    <li>
                        <A href="rate">"rate " <Badge>comming soon</Badge></A>
                    </li>
                    <li>
                        <A href="search">"search"</A>
                    </li>
                    <li>
                        <A href="switch">"switch"</A>
                    </li>
                    <li>
                        <A href="table">"table"</A>
                    </li>
                    <li>
                        <A href="tag">"tag"</A>
                    </li>
                    <li>
                        <A href="toast">"toast"</A>
                    </li>
                    <li>
                        <A href="tooltip">"tooltip"</A>
                    </li>
                    <li>
                        <A href="upload">"upload"</A>
                    </li>
                </menu>
            </nav>
            <main>
            <Routes>
                <Route path="/" view=move || view! { <h1>"Home"</h1> }/>
                <Route
                    path="/avatar"
                    view=move || {
                        view! {
                            <h1>"Avatar"</h1>
                            <Avatar image="".into()/>
                        }
                    }
                />

                <Route
                    path="/badge"
                    view=move || {
                        view! {
                            <h1>"Badge"</h1>
                            <Badge>"Badge"</Badge>
                        }
                    }
                />

                <Route
                    path="/button"
                    view=move || {
                        view! {
                            <h1>"Button"</h1>
                            <Button b_type="button".into()>"Button"</Button>
                        }
                    }
                />

                <Route
                    path="/dialog"
                    view=move || {
                        view! {
                            <h1>"Dialog"</h1>
                            <Dialog id="dialog1".into()>"Dialog"</Dialog>
                        }
                    }
                />

                <Route
                    path="/drawer"
                    view=move || {
                        view! {
                            <h1>"Drawer"</h1>
                            <Drawer id="drawer1".into()>"test"</Drawer>
                        }
                    }
                />

                <Route
                    path="/loading"
                    view=move || {
                        view! {
                            <h1>"Loading"</h1>
                            <Loading>"Loading..."</Loading>
                        }
                    }
                />

                <Route
                    path="/message"
                    view=move || {
                        view! {
                            <h1>"Message"</h1>
                            <Message>"Message"</Message>
                        }
                    }
                />

                <Route
                    path="/pagination"
                    view=move || {
                        view! {
                            <h1>"Pagination"</h1>
                            <Pagination current=1 total=10/>
                        }
                    }
                />

                <Route
                    path="/popover"
                    view=move || {
                        view! {
                            <h1>"Popover"</h1>
                            <Popover>"Popover"</Popover>
                        }
                    }
                />

                <Route
                    path="/qrcode"
                    view=move || {
                        view! {
                            <h1>"Qrcode"</h1>
                            <Qrcode value="Qrcode".into()/>
                        }
                    }
                />

                <Route
                    path="/search"
                    view=move || {
                        view! {
                            <h1>"Search"</h1>
                            <Search/>
                        }
                    }
                />

                <Route
                    path="/switch"
                    view=move || {
                        view! {
                            <h1>"Switch"</h1>
                            <Switch name="switch".into()/>
                        }
                    }
                />

                <Route
                    path="/table"
                    view=move || {
                        view! {
                            <h1>"Table"</h1>
                            <Table body=vec![] _headers=None _footer=None/>
                        }
                    }
                />

                <Route
                    path="/tag"
                    view=move || {
                        view! {
                            <h1>"Tag"</h1>
                            <Tag style=None state=None>
                                "Tag"
                            </Tag>
                        }
                    }
                />

                <Route
                    path="/toast"
                    view=move || {
                        view! {
                            <h1>"Toast"</h1>
                            <Toast>"Toast"</Toast>
                        }
                    }
                />

                <Route
                    path="/tooltip"
                    view=move || {
                        view! {
                            <h1>"Tooltip"</h1>
                            <Tooltip>"Tooltip"</Tooltip>
                        }
                    }
                />

                <Route
                    path="/upload"
                    view=move || {
                        view! {
                            <h1>"Upload"</h1>
                            <Upload/>
                        }
                    }
                />

                <Route
                    path="/input"
                    view=move || {
                        view! {
                            <h1>"Input"</h1>
                            <Input name="input".into() label="Input".into() _summary=None/>
                        }
                    }
                />

                <Route
                    path="/rate"
                    view=move || {
                        view! {
                            <h1>"Rate"</h1>
                            <Rate rating=move || 0 max=5 set_rating=move |_| {}/>
                        }
                    }
                />

                <Route
                    path="/picklist"
                    view=move || {
                        view! {
                            <h1>"PickList"</h1>
                            <PickList list=vec![("l".into(), "name".into())] label="label".into()/>
                        }
                    }
                />

                <Route
                    path="/taglist"
                    view=move || {
                        view! {
                            <h1>"TagList"</h1>
                            <PickList list=vec![("2".into(), "two".into())] label="label".into()/>
                        }
                    }
                />
                <Route
                    path="/kitchen_sink"
                    view=move || {
                        view! {
                            <KitchenSink/>
                        }
                    }
                />
                </Routes>
                </main>
            
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
