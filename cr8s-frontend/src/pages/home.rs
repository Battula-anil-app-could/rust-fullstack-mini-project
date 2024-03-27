use yew::prelude::*;
use yew_router::components::Redirect;
use Route::*;
use crate::{components::sidebar::SideBar, contexts::CurrentUserContext, Route};


#[function_component(Home)]
pub fn home() -> Html{
    let user_currernt_ctx = use_context::<CurrentUserContext>().expect("user context missed");
        match &user_currernt_ctx.user{
            Some(user) => {
                html!{
                    <div class = "container">
                        <div class="row min-vh-100 d-flex justify-content-center mt-3">
                            <div class = "col-4">
                                <SideBar />
                            </div>
                            <div class = "col">
                                <h1> {"Welcome "} {user.username.clone()} </h1>
                                <h2> {"Your Id Is "} {user.id.clone()} </h2>
                                <h3> {"Your loged in at "} {user.created_at.clone()} </h3>
                            </div>
                        </div>
                    </div>
                }
            },
            None => {
                html!{
                    <Redirect<Route> to={Route::Login}/>
                }
            }
        }
    
}