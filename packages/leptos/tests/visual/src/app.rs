use leptos::*;
use leptos_router::{Outlet, Route, Router, Routes, A};

use crate::spec::arrow::Arrow;
use crate::spec::auto_placement::AutoPlacement;
use crate::spec::auto_update::AutoUpdate;
use crate::spec::border::Border;
use crate::spec::containing_block::ContainingBlock;
use crate::spec::placement::Placement;
use crate::spec::relative::Relative;
use crate::spec::scroll::Scroll;
use crate::spec::table::Table;
use crate::utils::new::New;

const ROUTES: [&str; 23] = [
    "placement",
    "relative",
    "transform",
    "border",
    "scroll",
    "decimal-size",
    "table",
    "scrollbars",
    "shift",
    "flip",
    "size",
    "arrow",
    "offset",
    "hide",
    "autoPlacement",
    "inline",
    "autoUpdate",
    "shadom-DOM",
    "containing-block",
    "virtual-element",
    "perf",
    "iframe",
    "top-layer",
];

#[component]
pub fn AppWrapper() -> impl IntoView {
    view! {
        <div>
            <main>
                <Outlet />
            </main>
            <nav>
                <div class="nav-top">
                    <A href="/" class="home-button">
                        Tests
                    </A>
                    <A href="/new" class="new-button">
                        New
                    </A>
                </div>
                <ul>
                    <For
                        each=|| ROUTES
                        key=|path| path.to_string()
                        children=move |path| {
                            view! {
                                <A href=format!("/{path}") class="nav-link">
                                    {path.replace('-', " ")}
                                </A>
                            }
                        }
                    />
                </ul>
            </nav>
        </div>
    }
}

#[component]
pub fn Index() -> impl IntoView {
    view! {
        <h1>Floating UI Testing Grounds</h1>
        <p>
            Welcome! On the left is a navigation bar to browse through
            different testing files. These files, and the control buttons, are
            used by Playwright to take screenshots of the page for visual
            snapshot testing.
        </p>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=AppWrapper>
                    <Route path="" view=Index />

                    <Route path="new" view=New />
                    <Route path="placement" view=Placement />
                    <Route path="relative" view=Relative />
                    // <Route path="transform" view=Transform />
                    <Route path="border" view=Border />
                    <Route path="scroll" view=Scroll />

                    <Route path="table" view=Table />

                    <Route path="arrow" view=Arrow />
                    <Route path="autoPlacement" view=AutoPlacement />

                    <Route path="autoUpdate" view=AutoUpdate />

                    <Route path="containing-block" view=ContainingBlock />

                    <Route path="/*any" view=|| view! { <h1>"Not Found"</h1> }/>
                </Route>
            </Routes>
        </Router>
    }
}
