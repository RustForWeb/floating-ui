use leptos::*;
use leptos_router::{Outlet, Route, Router, Routes, A};

use crate::spec::placement::Placement;
use crate::spec::relative::Relative;
use crate::utils::new::New;

const ROUTES: [&str; 2] = ["placement", "relative"];

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
                                    {path.replace('-', "")}
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
                </Route>
            </Routes>
        </Router>
    }
}
