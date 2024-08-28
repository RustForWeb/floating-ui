use yew::prelude::*;
use yew_router::prelude::*;

use crate::spec::arrow::Arrow;
// use crate::spec::auto_placement::AutoPlacement;
// use crate::spec::auto_update::AutoUpdate;
// use crate::spec::border::Border;
// use crate::spec::containing_block::ContainingBlock;
// use crate::spec::decimal_size::DecimalSize;
// use crate::spec::flip::Flip;
// use crate::spec::hide::Hide;
// use crate::spec::inline::Inline;
// use crate::spec::offset::Offset;
use crate::spec::placement::Placement;
use crate::spec::relative::Relative;
// use crate::spec::scroll::Scroll;
// use crate::spec::scrollbars::Scrollbars;
// use crate::spec::shift::Shift;
// use crate::spec::size::Size;
// use crate::spec::table::Table;
// use crate::spec::transform::Transform;
// use crate::spec::virtual_element::VirtualElement;
use crate::utils::new::New;

#[derive(Clone, Copy, Debug, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Index,
    #[at("/new")]
    New,

    #[at("/placement")]
    Placement,
    #[at("/relative")]
    Relative,
    #[at("/transform")]
    Transform,
    #[at("/border")]
    Border,
    #[at("/scroll")]
    Scroll,
    #[at("/decimal-size")]
    DecimalSize,
    #[at("/table")]
    Table,
    #[at("/scrollbars")]
    Scrollbars,
    #[at("/shift")]
    Shift,
    #[at("/flip")]
    Flip,
    #[at("/size")]
    Size,
    #[at("/arrow")]
    Arrow,
    #[at("/offset")]
    Offset,
    #[at("/hide")]
    Hide,
    #[at("/autoPlacement")]
    AutoPlacement,
    #[at("/inline")]
    Inline,
    #[at("/autoUpdate")]
    AutoUpdate,
    #[at("/shadow-DOM")]
    ShadowDom,
    #[at("/containing-block")]
    ContainingBlock,
    #[at("/virtual-element")]
    VirtualElement,
    #[at("/perf")]
    Perf,
    #[at("/iframe")]
    Iframe,
    #[at("/top-layer")]
    TopLayer,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Index => html! { <Index /> },
        Route::New => html! { <New /> },

        Route::Placement => html! { <Placement /> },
        Route::Relative => html! { <Relative /> },
        Route::Arrow => html! { <Arrow /> },

        _ => html! { <h1>{"Not Found"}</h1> },
    }
}

#[derive(PartialEq, Properties)]
pub struct AppWrapperProps {
    pub children: Html,
}

// TODO: active classes for links

#[function_component]
pub fn AppWrapper(props: &AppWrapperProps) -> Html {
    html! {
        <div>
            <main>
                {props.children.clone()}
            </main>
            <nav>
                <div class="nav-top">
                    <Link<Route> classes={classes!("home-button")} to={Route::Index}>{"Tests"}</Link<Route>>
                    <Link<Route> classes={classes!("new-button")} to={Route::New}>{"New"}</Link<Route>>
                </div>
                <ul>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Placement}>{"placement"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Relative}>{"relative"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Transform}>{"transform"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Border}>{"border"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Scroll}>{"scroll"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::DecimalSize}>{"decimal size"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Table}>{"table"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Scrollbars}>{"scrollbars"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Shift}>{"shift"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Flip}>{"flip"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Size}>{"size"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Arrow}>{"arrow"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Offset}>{"offset"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Hide}>{"hide"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::AutoPlacement}>{"autoPlacement"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Inline}>{"inline"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::AutoUpdate}>{"autoUpdate"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::ShadowDom}>{"shadow DOM"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::ContainingBlock}>{"containing block"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::VirtualElement}>{"virtual element"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Perf}>{"perf"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::Iframe}>{"iframe"}</Link<Route>>
                    <Link<Route> classes={classes!("nav-link")} to={Route::TopLayer}>{"top layer"}</Link<Route>>
                </ul>
            </nav>
        </div>
    }
}

#[function_component]
pub fn Index() -> Html {
    html! {
        <>
            <h1>{"Floating UI Testing Grounds"}</h1>
            <p>
                {"Welcome! On the left is a navigation bar to browse through
                different testing files. These files, and the control buttons, are
                used by Playwright to take screenshots of the page for visual
                snapshot testing."}
            </p>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <AppWrapper>
                <Switch<Route> render={switch} />
            </AppWrapper>
        </BrowserRouter>
    }
}
