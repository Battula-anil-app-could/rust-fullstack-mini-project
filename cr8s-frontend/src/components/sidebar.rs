use yew::prelude::*;
use yew_router::prelude::*;
use crate::Route;


#[function_component(SideBar)]
pub fn sidebar()->Html{
    let current_page = use_route::<Route>().expect("pages not found");
    
    let home_class = if current_page == Route::Home{
        classes!("nav-link", "active")
    }else{
        classes!("nav-link")
    };
    let raustacen_class = if current_page == Route::Rustaceans{
        classes!("nav-link", "active")
    }else{
        classes!("nav-link")
    };
    let crates_class = if current_page == Route::Crates{
        classes!("nav-link", "active")
    }else{
        classes!("nav-link")
    };
    
    html!{
        <nav class="navbar navbar-light">
            <ol class="nav navbar-nav">
                <li class="nav-item">
                    <Link<Route> to={Route::Home} classes={home_class}>
                        {"Home"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Rustaceans} classes={raustacen_class}>
                        {"Raustacean"}
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={Route::Crates} classes={crates_class}>
                        {"Crates"}
                    </Link<Route>>
                </li>
            </ol>
        </nav>
    }
}