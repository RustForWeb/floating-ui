use leptos::prelude::*;
use leptos_router::components::{Outlet, ParentRoute, Route, Router, Routes, A};
use leptos_router::path;

use crate::spec::arrow::Arrow;
use crate::spec::auto_placement::AutoPlacement;
use crate::spec::auto_update::AutoUpdate;
use crate::spec::border::Border;
use crate::spec::containing_block::ContainingBlock;
use crate::spec::decimal_size::DecimalSize;
use crate::spec::flip::Flip;
use crate::spec::hide::Hide;
use crate::spec::inline::Inline;
use crate::spec::offset::Offset;
use crate::spec::placement::Placement;
use crate::spec::relative::Relative;
use crate::spec::scroll::Scroll;
use crate::spec::scrollbars::Scrollbars;
use crate::spec::shift::Shift;
use crate::spec::size::Size;
use crate::spec::table::Table;
use crate::spec::transform::Transform;
use crate::spec::virtual_element::VirtualElement;
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
    "shadow-DOM",
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
                    <A href="/" attr:class="home-button">
                        Tests
                    </A>
                    <A href="/new" attr:class="new-button">
                        New
                    </A>
                </div>
                <ul>
                    <For
                        each=|| ROUTES
                        key=|path| path.to_string()
                        children=move |path| {
                            view! {
                                <A href=format!("/{path}") attr:class="nav-link">
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
            <Routes fallback=|| view! { <h1>"Not Found"</h1> }>
                <ParentRoute path=path!("/") view=AppWrapper>
                    <Route path=path!("") view=Index />

                    <Route path=path!("new") view=New />
                    <Route path=path!("placement") view=Placement />
                    <Route path=path!("relative") view=Relative />
                    <Route path=path!("transform") view=Transform />
                    <Route path=path!("border") view=Border />
                    <Route path=path!("scroll") view=Scroll />
                    <Route path=path!("decimal-size") view=DecimalSize />
                    <Route path=path!("table") view=Table />
                    <Route path=path!("scrollbars") view=Scrollbars />
                    <Route path=path!("shift") view=Shift />
                    <Route path=path!("flip") view=Flip />
                    <Route path=path!("size") view=Size />
                    <Route path=path!("arrow") view=Arrow />
                    <Route path=path!("offset") view=Offset />
                    <Route path=path!("hide") view=Hide />
                    <Route path=path!("autoPlacement") view=AutoPlacement />
                    <Route path=path!("inline") view=Inline />
                    <Route path=path!("autoUpdate") view=AutoUpdate />
                    // <Route path=path!("shadow-DOM") view=ShadowDom />
                    <Route path=path!("containing-block") view=ContainingBlock />
                    <Route path=path!("virtual-element") view=VirtualElement />
                    // <Route path=path!("perf") view=Perf />
                    // <Route path=path!("iframe") view=Iframe />
                    // <Route path=path!("top-layer") view=TopLayer />
                </ParentRoute>
            </Routes>
        </Router>
    }
}
