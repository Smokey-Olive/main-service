use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Hello New Smokey Olive Ci Automation V3" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav>
              <div>
                <div>
                  <a href=r#"https://smokeyolive.xyz"#>{"SmokeyOlive"}</a>
                </div>
                <ul>
                  <li><a href=r#"https://smokeyolive.xyz"#>{"Home"}</a></li>
                  <li><a href=r#"https://wine.smokeyolive.xyz"#>{"Wine"}</a></li>
                  <li><a href=r#"https://beer.smokeyolive.xyz"#>{"Beer"}</a></li>
                </ul>
              </div>
            </nav>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
