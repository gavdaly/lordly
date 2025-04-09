use crate::components::{Avatar, Button, PickList, Search, TagList};
use leptos::*;

#[component]
pub fn KitchenSink() -> impl IntoView {
    let code = r#"#header h1 a {
    display: block;
    width: 300px;
    height: 80px;
}"#;

    let cite = r#"malesuada
fames
ac turpis"#;
    view! {
        <header>
            <a href="./">Home</a>
            <Avatar
                src=r#"https://img.freepik.com/premium-vector/product-manager-woman-icon-flat-illustration-product-manager-woman-vector-icon-isolated-white-background_98396-40241.jpg?w=1380"#
                shape="circular"
            />
        </header>
        <nav>
            <menu>
                <li>
                    <a href="#">"marketing"</a>
                </li>
                <li>
                    <a href="#">"content"</a>
                </li>
                <li>
                    <a href="#">"product"</a>
                </li>
                <li>
                    <a href="#">"test"</a>
                </li>
                <li>
                    <a href="#">"test"</a>
                </li>
                <li>
                    <a href="#">"test"</a>
                </li>
            </menu>
        </nav>
        <Search/>
        <section class="content">
            <img
                class="full"
                src="https://images.unsplash.com/photo-1682686580391-615b1f28e5ee?q=80&w=2670&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDF8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                alt=""
            />
            <main>
                <h1>
                    "This is the primary heading and there should only be one of these per page"
                </h1>
                <p>
                    "A small " <u>"paragraph"</u> " to " <em>"emphasis"</em> " and show "
                    <strong>"important"</strong> " bits. " <kbd>"Ctrl"</kbd> " + "
                    <kbd>"Shift"</kbd> " + " <kbd>"R"</kbd> " " <sub>"should"</sub> " "
                    <sup>"implement"</sup> " " <code>"run()"</code> "This one "
                    <dfn id="deet">"deet"</dfn> "."
                </p>
                <p>
                    "A " <abbr>"sec"</abbr> " (second) " <i>"paragraph"</i> " and "
                    <mark>"marks"</mark>
                </p>
                <ul>
                    <li>"This is a list item"</li>
                    <li>"So is this - there could be more"</li>
                    <li>
                        "Make sure to style list items to:" <ul>
                            <li>"Not forgetting child list items"</li>
                            <li>"Not forgetting child list items"</li>
                            <li>"Not forgetting child list items"</li>
                            <li>"Not forgetting child list items"</li>
                        </ul>
                    </li>
                    <li>"A couple more"</li>
                    <li>"top level list items"</li>
                </ul>
                <p>"Don't forget " <strong>"Ordered lists"</strong> ":"</p>
                <ol>
                    <li>
                        "Lorem " <b>"ipsum dolor sit amet"</b> ", consectetuer adipiscing elit."
                    </li>
                    <li>
                        "Aliquam tincidunt mauris eu risus." <ol>
                            <li>"Lorem ipsum dolor sit amet, consectetuer adipiscing elit."</li>
                            <li>"Aliquam tincidunt mauris eu risus."</li>
                        </ol>
                    </li>
                    <li>"Lorem ipsum dolor sit amet, consectetuer adipiscing elit."</li>
                    <li>"Aliquam tincidunt mauris eu risus."</li>
                </ol>
                <picture>
                    <source
                        srcset="https://images.unsplash.com/photo-1701505708183-59f709e7d4e6?q=80&w=1032&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        media="(orientation: portrait)"
                    />
                    <img
                        src="https://images.unsplash.com/photo-1701505708183-59f709e7d4e6?q=80&w=1032&auto=format&fit=crop&ixlib=rb-4.0.3&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D"
                        alt="Image of folds"
                    />
                </picture>
                <section>
                    <hgroup>
                        <h2>
                            "A sub heading which is not as important as the first, but is quite imporant overall"
                        </h2>
                        <p>
                            <q cite="example.com">
                                "Pellentesque habitant morbi tristique senectus"
                            </q>
                            " et netus et "
                            <s>{cite}</s>
                            " egestas."
                            <span>
                                "Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante."
                            </span>
                            " Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo."
                        </p>
                    </hgroup>
                    <table>
                        <caption>"Example tension."</caption>
                        <colgroup>
                            <col/>
                            <col span="2" class="batman"/>
                            <col span="2" class="flash"/>
                        </colgroup>
                        <thead>
                            <th></th>
                            <th>"Table Heading"</th>
                            <th>"Table Heading"</th>
                            <th>"Table Heading"</th>
                            <th>"Table Heading"</th>
                        </thead>
                        <tbody>
                            <tr>
                                <th>"Summ"</th>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                            </tr>
                            <tr>
                                <th>"Summ"</th>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                            </tr>
                            <tr>
                                <th>"Summ"</th>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                            </tr>
                            <tr>
                                <th>"Summ"</th>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                                <td>"table data"</td>
                            </tr>
                        </tbody>
                        <tfoot>
                            <tr>
                                <th>"Total"</th>
                                <td>"table total"</td>
                                <td>"table total"</td>
                                <td>"table total"</td>
                                <td>"table total"</td>
                            </tr>
                        </tfoot>
                    </table>

                    <h3>
                        "A sub heading which is not as important as the second, but should be used with consideration"
                    </h3>
                    <p>
                        "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo."
                    </p>
                    <blockquote cite="http://example.com">
                        <p>
                            "“Ooh - a blockquote! Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus magna. Cras in mi at felis aliquet congue. Ut a est eget ligula molestie gravida. Curabitur massa. Donec eleifend, libero at sagittis mollis, tellus est malesuada tellus, at luctus turpis elit sit amet quam. Vivamus pretium ornare est.”"
                        </p>
                        <footer>"someone " <cite>"material"</cite></footer>
                    </blockquote>

                    <hr/>

                    <h4>
                        "A sub heading which is not as important as the second, but should be used with consideration"
                    </h4>
                    <p>
                        "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo."
                    </p>
                    <pre>
                        <code>{code}</code>
                    </pre>
                    <h5>
                        "A sub heading which is not as important as the second, but should be used with consideration"
                    </h5>
                    <p>
                        <samp>
                            "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu xslibero sit amet quam egestas semper."
                        </samp>
                        "Aenean ultricies mi vitae est. Mauris placerat eleifend leo."
                    </p>
                    <dl>
                        <dt>"Definition list"</dt>
                        <dd>
                            "Consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                        </dd>
                        <dt>"Lorem ipsum dolor sit amet"</dt>
                        <dd>
                            "Consectetur adipisicing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."
                        </dd>
                    </dl>
                    <figure>
                        <blockquote>
                            <p>
                                "It was a bright cold day in April, and the clocks were striking thirteen."
                            </p>
                        </blockquote>
                        <figcaption>
                            "First sentence in " <cite>
                                <a href="http://www.george-orwell.org/1984/0.html">
                                    "Nineteen Eighty-Four"
                                </a>
                            </cite> " by George Orwell (Part 1, Chapter 1)."
                        </figcaption>
                    </figure>
                    <h6>
                        "This heading plays a relatively small bit part role, if you use it at all"
                    </h6>
                    <p>
                        "Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum tortor quam, feugiat vitae, ultricies eget, tempor sit amet, ante. Donec eu libero sit amet quam egestas semper. Aenean ultricies mi vitae est. Mauris placerat eleifend leo."
                    </p>
                    <form>
                        <div>
                            <label for="text">"text input"</label>
                            <input id="text" type="text"/>
                        </div>
                        <div>
                            <label for="tel">"telephone input"</label>
                            <input id="tel" type="tel" inputmode="tel" autocomplete="tel"/>
                        </div>
                        <div>
                            <label for="email">"email"</label>
                            <input id="email" type="email" inputmode="email" autocomplete="email"/>
                        </div>
                        <div>
                            <label for="date">"date"</label>
                            <input type="date" id="date"/>
                        </div>
                        <div>
                            <label for="datetime">"datetime"</label>
                            <input type="datetime" id="datetime"/>
                        </div>
                        <div>
                            <label for="datetime-local">"datetime-local"</label>
                            <input type="datetime-local" id="datetime-local"/>
                        </div>
                        <div>
                            <label for="file2">"file"</label>
                            <input type="file" id="file2"/>
                        </div>
                        <div>
                            <label for="month">"month"</label>
                            <input type="month" name="month" id="month"/>
                        </div>
                        <div>
                            <label for="number">"number"</label>
                            <input type="number" name="number" id="number"/>
                        </div>
                        <div>
                            <label for="password">"password"</label>
                            <input
                                type="password"
                                name="password"
                                autocomplete="current-password"
                                id="password"
                            />
                        </div>
                        <div>
                            <label for="password_new">"password new"</label>
                            <input
                                type="password"
                                autocomplete="new-password"
                                name="password_new"
                                id="password_new"
                            />
                        </div>
                        <div>
                            <label for="password_conf">"password conf"</label>
                            <input type="password" name="password_conf" id="password_conf"/>
                        </div>
                        <div>
                            <label for="range">"range"</label>
                            <input type="range" name="range" id="range"/>
                        </div>
                        <div>
                            <label for="time">"time"</label>
                            <input type="time" name="time" id="time"/>
                        </div>
                        <div>
                            <label for="url">"url"</label>
                            <input type="url" name="url" inputmode="url" id="url"/>
                        </div>
                        <div>
                            <label for="week">"week"</label>
                            <input type="week" name="week" id="week"/>
                        </div>
                        <div>
                            <label for="color">"color"</label>
                            <input type="color" id="color" name="color"/>
                        </div>
                        <div>
                            <label for="dino-select">"Choose a dinosaur:"</label>
                            <select id="dino-select">
                                <optgroup label="Theropods">
                                    <option>"Tyrannosaurus"</option>
                                    <option>"Velociraptor"</option>
                                    <option>"Deinonychus"</option>
                                </optgroup>
                                <optgroup label="Sauropods">
                                    <option>"Diplodocus"</option>
                                    <option>"Saltasaurus"</option>
                                    <option>"Apatosaurus"</option>
                                </optgroup>
                            </select>
                        </div>
                        <div>
                            <label for="textarea">"textarea"</label>
                            <textarea id="textarea" name="textarea"></textarea>
                        </div>
                        <fieldset>
                            <legend>"data"</legend>
                            <div>
                                <input type="radio" name="radio" id="radio1" vaule="radio1"/>
                                <label for="radio1">"radio1"</label>
                            </div>
                            <div>
                                <input type="radio" name="radio" id="radio2" value="radio2"/>
                                <label for="radio2">"radio2"</label>
                            </div>
                        </fieldset>
                        <fieldset>
                            <legend>checks</legend>
                            <div>
                                <input type="checkbox" name="check" id="check1" value="check1"/>
                                <label for="check1">check1</label>
                            </div>
                            <div>
                                <input type="checkbox" name="check" id="check2" value="check2"/>
                                <label for="check2">check1</label>
                            </div>
                        </fieldset>
                        <PickList
                            list=vec![
                                ("mar".into(), "Marnaria".into()),
                                ("alf".into(), "Alfrado".into()),
                            ]

                            label="Sauce Type"
                        />
                        <TagList
                            list=vec![
                                ("onion".into(), "Onions".into()),
                                ("sun".into(), "Sun Dried Tomatoes".into()),
                                ("pineapple".into(), "Pineapple".into()),
                            ]

                            label="Toppings"
                        />
                        <Button type_="submit">"Submit"</Button>
                        <Button type_="reset">"Reset"</Button>
                    </form>
                    <form disabled>
                        <div>
                            <label for="text2">text input</label>
                            <input id="text2" type="text"/>
                        </div>
                        <Button type_="submit">"Submit"</Button>
                        <Button type_="reset">"Reset"</Button>
                    </form>
                    <form data-state="error">
                        <div>
                            <label for="text3">"text input"</label>
                            <input id="text3" type="text"/>
                        </div>
                        <Button type_="submit">"submit"</Button>
                        <Button type_="reset">"reset"</Button>
                    </form>
                    <aside>
                        <p>"aside information"</p>
                    </aside>
                    <details>
                        <summary>"Details"</summary>
                        "Something small enough to escape casual notice."
                    </details>
                    <meter id="fuel" min="0" max="100" low="33" high="66" optimum="80" value="50">
                        "at 50/100"
                    </meter>
                    <div>
                        <label for="file">"File progress:"</label>
                        <progress id="file" max="100" value="70">
                            "70%"
                        </progress>
                    </div>
                    <p>
                        "The volume of a box is " <var>"l"</var> " × " <var>"w"</var> " × "
                        <var>"h"</var> ", where " <var>"l"</var> " represents the length,"
                        <var>"w"</var> " the width and " <var>"h"</var> " the height of the box."
                    </p>
                </section>
            </main>
            <section class="feature">
                <h2>"Feature"</h2>
            </section>
            <aside class="popout">
                <h2>"Pop out"</h2>
            </aside>
            <footer>
                <time datetime="2024-07-07">"July 7"</time>
                <section>"testing"</section>
                <address>
                    "location" <a href="mailto:test@test.com">"email"</a>
                    <a href="tel:+18008889999">"+1 (800) 888-9999"</a>
                </address>
                <a href="#" rel="terms-of-service">
                    "terms of service"
                </a>
                <a href="#" rel="privacy-policy">
                    "privacy policy"
                </a>
                <a href="#" rel="license">
                    "license"
                </a>
                <a href="#" rel="help">
                    "help"
                </a>

                <small>&copy; now</small>
            </footer>
        </section>
        <dialog id="greeting">
            <p>"Greetings, one and all!"</p>
            <form method="dialog">
                <Button type_="submit">"OK"</Button>
            </form>
        </dialog>
    }
}
