use crate::api::user::{self, api_login, User};
use crate::components::input::Input;
use crate::components::alert::Alert;
use crate::contexts::{CurrentUserActions, CurrentUserContext, CurrentUserDispatchActions};
use crate::Route;
use gloo_console::log;
use web_sys::*;
use yew::{platform::spawn_local, prelude::*};
use gloo_net::Error;
use yew_router::hooks::*;


async fn login(username: String, password:String) -> Result<User, Error>{
    Ok(api_login(username, password).await?)
}

#[function_component(LoginForm)]
pub fn login_form() -> Html {
    let navigator = use_navigator();
    let name_change_handler = use_state(String::default);
    let username = (*name_change_handler).clone();
    let username_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            name_change_handler.set(input.value());
            
        }

    });

    let password_change_handler = use_state(String::default);
    let password = (*password_change_handler).clone();
    let password_changed = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlInputElement>();
        if let Some(input) = target {
            password_change_handler.set(input.value());
        }
    });
    let error_change_handelr = use_state(String::default);
    let error_message = (*error_change_handelr).clone();

    let user_currernt_ctx = use_context::<CurrentUserContext>().expect("user context missed");

    let user_name = username.clone();
    let user_password = password.clone();
    let error_change_handelr = error_change_handelr.clone();
    
    let onsubmit = Callback::from(move |e: SubmitEvent| {
        e.prevent_default();
        let user_name = user_name.clone();
        let user_password = user_password.clone();
        let error_change_handelr= error_change_handelr.clone();
        let cloned_navigator = navigator.clone();
        let user_currernt_ctx = user_currernt_ctx.clone();
        spawn_local( async move {
            match login(user_name.clone(), user_password.clone()).await{
                Ok(responses) => {
                    let id = responses.id;
                    let username = responses.username;
                    let created_at = responses.created_at;
                    user_currernt_ctx.clone().dispatch(CurrentUserDispatchActions{
                        action_type: CurrentUserActions::LoginSuccess,
                        login_response: Some(user::MainUser{id, username, created_at,}),
                    });
                    if let Some(nav) = cloned_navigator{
                        nav.push(&Route::Home)
                    }
                },
                Err(e) => error_change_handelr.set(e.to_string()),
            }
        });
        
    });

    html! {
        <form onsubmit = {onsubmit}>
            if error_message.len() > 0{
                <Alert alert_type = "danger" message = {error_message} />
            }
            <Input 
                input_type= "text" 
                name = "name" 
                label="UserName" 
                value = {username} 
                on_change = {username_change}
            />
            <br/>
            <Input 
                input_type= "password" 
                name = "password" 
                label="Password" 
                value = {password} 
                on_change = {password_changed}
            />
            <br/>
            <button class = "btn btn-primary m-3" type = "submit"> {"submit"}</button>
        </form>
    }
}
