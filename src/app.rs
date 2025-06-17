use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::kitchen_sink::KitchenSink;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::components::*;
use leptos_router::path;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (dialog_open, set_dialog_open) = signal(false);

    view! {
        <Stylesheet id="leptos" href="/pkg/lordly.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <button on:click=move |_| {set_dialog_open.set(true)}>"+"</button>
            <Dia open=dialog_open>
                <nav>
                    <button on:click=move |_| {set_dialog_open.set(false)}>"X"</button>
                    <menu>
                        <li>
                            <A href="/">"Home"</A>
                        </li>
                        <li>
                            <A href="/alert">"Alert"</A>
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
                            <A href="/drawer">"drawer"</A>
                        </li>
                        <li>
                            <A href="/loading">"loading"</A>
                        </li>
                        <li>
                            <A href="/message">"message"</A>
                        </li>
                        <li>
                            <A href="/pagination">"pagination"</A>
                        </li>
                        <li>
                            <A href="/popover">"popover"</A>
                        </li>
                        <li>
                            <A href="/qrcode">"qrcode"</A>
                        </li>
                        <li>
                            <A href="/rate">"rate"</A>
                        </li>
                        <li>
                            <A href="/search">"search"</A>
                        </li>
                        <li>
                            <A href="/switch">"switch"</A>
                        </li>
                        <li>
                            <A href="/table">"table"</A>
                        </li>
                        <li>
                            <A href="/tag">"tag"</A>
                        </li>
                        <li>
                            <A href="/toast">"toast"</A>
                        </li>
                        <li>
                            <A href="/tooltip">"tooltip"</A>
                        </li>
                        <li>
                            <A href="/upload">"upload"</A>
                        </li>
                        <li>
                            <A href="/kitchen_sink">"kitchen sink"</A>
                        </li>
                    </menu>
                </nav>
            </Dia>
            <main class="content">
            <Routes fallback=move || view!{<div>"fallback"</div>}>
                    <Route path=path!("/") view=move || view! { <h1>"Home"</h1> }/>
                    <Route
                        path=path!("/alert")
                        view=move || {
                            view! {
                                <Alert title="Success" has_closer=true>
                                    "You Seccessifully did something"
                                </Alert>
                            }
                        }
                    />
                    <Route
                        path=path!("/avatar")
                        view=move || {
                            view! {
                                <h1>"Avatar"</h1>
                                <main>
                                    <Avatar name="Gav" src="https://gavdaly.com/profile.jpg"/>
                                    <Avatar
                                        name="Gav"
                                        src="https://gavdaly.com/profile.jpg"
                                        width="200"
                                        shape="rounded"
                                    />
                                    <Avatar
                                        name="Gav"
                                        src="https://gavdaly.com/profile.jpg"
                                        width="300"
                                        shape="circular"
                                    />
                                </main>
                            }
                        }
                    />

                    <Route
                        path=path!("/badge")
                        view=move || {
                            view! { <Badge>"Badge"</Badge> }
                        }
                    />

                    <Route
                        path=path!("/button")
                        view=move || {
                            view! { <Button>"Button"</Button> }
                        }
                    />

                    <Route
                        path=path!("/dialog")
                        view=move || {
                            let (d, set_d) = signal(false);
                            view! { <Dia open=d>"Dialog"</Dia> }
                        }
                    />

                    <Route
                        path=path!("/drawer")
                        view=move || {
                            view! {
                                <Drawer id="drawer1" anchor="left">
                                    "test"
                                </Drawer>
                            }
                        }
                    />

                    <Route
                        path=path!("/loading")
                        view=move || {
                            view! { <Loading>"Loading..."</Loading> }
                        }
                    />

                    <Route
                        path=path!("/message")
                        view=move || {
                            view! { <Message>"Message"</Message> }
                        }
                    />

                    <Route
                        path=path!("/pagination")
                        view=move || {
                            view! {
                                <Pagination current=1 total=10/>
                                <Pagination current=2 total=10/>
                                <Pagination current=5 total=10/>
                            }
                        }
                    />

                    <Route
                        path=path!("/popover")
                        view=move || {
                            view! { <Popover id="pop">"Popover"</Popover> }
                        }
                    />

                    <Route
                        path=path!("/qrcode")
                        view=move || {
                            view! { <Qrcode value="Qrcode".into()/> }
                        }
                    />

                    // <Route
                    //     path=path!("/search")
                    //     view=move || {
                    //         view! { <Search/> }
                    //     }
                    // />

                    <Route
                        path=path!("/switch")
                        view=move || {
                            view! { <Switch name="switch"/> }
                        }
                    />

                    <Route
                        path=path!("/table")
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
                        path=path!("/tag")
                        view=move || {
                            view! { <Tag color="warning">"Tag"</Tag> }
                        }
                    />

                    <Route
                        path=path!("/toast")
                        view=move || {
                            view! { <Toast>"Toast"</Toast> }
                        }
                    />

                    <Route
                        path=path!("/tooltip")
                        view=move || {
                            view! { <Tooltip>"Tooltip"</Tooltip> }
                        }
                    />

                    <Route
                        path=path!("/drop_area")
                        view=move || {
                            view! { <DropArea name="avatar" /> }
                        }
                    />

                    <Route path=path!("/file")
                        view=move || {
                            view! { <File name="file" /> }
                        }
                    />

                    <Route
                        path=path!("/input")
                        view=move || {
                            view! {
                                <h1>"Input"</h1>
                                <Input name="input" label="Input"/>
                            }
                        }
                    />

                    <Route
                        path=path!("/rate")
                        view=move || {
                            view! {
                                <h1>"Rate"</h1>
                                <Rate max=5 name="rating"/>
                            }
                        }
                    />

                    <Route
                        path=path!("/picklist")
                        view=move || {
                            view! {
                                <PickList list=vec![("l".into(), "name".into())] label="label"/>
                            }
                        }
                    />

                    <Route
                        path=path!("/taglist")
                        view=move || {
                            view! { <TagList list=vec![("2".into(), "two".into())] label="label"/> }
                        }
                    />

                    <Route
                        path=path!("/kitchen_sink")
                        view=move || {
                            view! { <KitchenSink/> }
                        }
                    />

                </Routes>
            </main>

        </Router>
    }
}
