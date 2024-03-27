use crate::{components::login_form::*, contexts::CurrentUserContext, Route};
use yew::prelude::*;
use yew_router::components::Redirect;

#[function_component(Login)]
pub fn login() -> Html {
    let user_currernt_ctx = use_context::<CurrentUserContext>().expect("user context missed");
    match &user_currernt_ctx.user{
        Some(_) => {
            html!{
                <Redirect<Route> to={Route::Home}/>
            }
        },
        None => {
            html! {
                <div class = "container">
                    <div class="row min-vh-100 d-flex justify-content-center align-items-center">
                        <div class = "col-md-4">
                            <LoginForm />
                        </div>
                    </div>
                </div>
            }
        }
    }
   
}
