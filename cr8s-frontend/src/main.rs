use yew::prelude::*;
use yew_router::prelude::*;

mod api;
mod components;
mod pages;
pub mod contexts;

#[derive(Routable, PartialEq, Clone)]
enum Route{
  #[at("/")]
  Home,
  #[at("/login")]
  Login,
  #[at("/rustaceans")]
  Rustaceans,
  #[at("/crates")]
  Crates,
  #[not_found]
  #[at("/404")]
  NotFound,
}

fn Pages(route: Route) -> Html{
    match route{
      Route::NotFound => html!{<pages::not_found::NotFound/>},
      Route::Login => html!{<pages::login::Login />},
      _=> html!{<pages::home::Home/>},
    }
  
}

#[function_component(App)]
fn app() -> Html {
    html! {
      
      <BrowserRouter>
        <contexts::CurrentUserProvider>
          <Switch<Route> render={Pages}/>
        </contexts::CurrentUserProvider>
        
      </BrowserRouter>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
