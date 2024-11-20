use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::kitchen_sink::KitchenSink;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/lordly.css"/>

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
            <main class="content">
                <Routes>
                    <Route path="/" view=move || view! { <h1>"Home"</h1> }/>
                    <Route
                        path="/avatar"
                        view=move || {
                            view! {
                                <h1>"Avatar"</h1>
                                <Avatar image=""/>
                            }
                        }
                    />

                    <Route
                        path="/badge"
                        view=move || {
                            view! { <Badge>"Badge"</Badge> }
                        }
                    />

                    <Route
                        path="/button"
                        view=move || {
                            view! { <Button>"Button"</Button> }
                        }
                    />

                    <Route
                        path="/dialog"
                        view=move || {
                            view! { <Dialog id="dialog1">"Dialog"</Dialog> }
                        }
                    />

                    <Route
                        path="/drawer"
                        view=move || {
                            view! {
                                <Drawer id="drawer1" anchor="left">
                                    "test"
                                </Drawer>
                            }
                        }
                    />

                    <Route
                        path="/loading"
                        view=move || {
                            view! { <Loading>"Loading..."</Loading> }
                        }
                    />

                    <Route
                        path="/message"
                        view=move || {
                            view! { <Message>"Message"</Message> }
                        }
                    />

                    <Route
                        path="/pagination"
                        view=move || {
                            view! {
                                <Pagination current=1 total=10/>
                                <Pagination current=2 total=10/>
                                <Pagination current=5 total=10/>
                            }
                        }
                    />

                    <Route
                        path="/popover"
                        view=move || {
                            view! { <Popover id="pop">"Popover"</Popover> }
                        }
                    />

                    <Route
                        path="/qrcode"
                        view=move || {
                            view! { <Qrcode value="Qrcode".into()/> }
                        }
                    />

                    <Route
                        path="/search"
                        view=move || {
                            view! { <Search/> }
                        }
                    />

                    <Route
                        path="/switch"
                        view=move || {
                            view! { <Switch name="switch"/> }
                        }
                    />

                    <Route
                        path="/table"
                        view=move || {
                            view! {
                                <Table body=vec![
                                    vec!["one".into(), "two".into()],
                                    vec!["three".into(), "four".into()],
                                ]/>
                            }
                        }
                    />

                    <Route
                        path="/tag"
                        view=move || {
                            view! { <Tag color="warning">"Tag"</Tag> }
                        }
                    />

                    <Route
                        path="/toast"
                        view=move || {
                            view! { <Toast>"Toast"</Toast> }
                        }
                    />

                    <Route
                        path="/tooltip"
                        view=move || {
                            view! { <Tooltip>"Tooltip"</Tooltip> }
                        }
                    />

                    <Route
                        path="/upload"
                        view=move || {
                            view! { <Upload name="avatar".into() drop_area=true/> }
                        }
                    />

                    <Route
                        path="/input"
                        view=move || {
                            view! {
                                <h1>"Input"</h1>
                                <Input name="input" label="Input"/>
                            }
                        }
                    />

                    <Route
                        path="/rate"
                        view=move || {
                            view! {
                                <h1>"Rate"</h1>
                                <Rate max=5 name="rating"/>
                            }
                        }
                    />

                    <Route
                        path="/picklist"
                        view=move || {
                            view! {
                                <PickList list=vec![("l".into(), "name".into())] label="label"/>
                            }
                        }
                    />

                    <Route
                        path="/taglist"
                        view=move || {
                            view! { <TagList list=vec![("2".into(), "two".into())] label="label"/> }
                        }
                    />

                    <Route
                        path="/kitchen_sink"
                        view=move || {
                            view! { <KitchenSink/> }
                        }
                    />

                </Routes>
            </main>

        </Router>
    }
}
