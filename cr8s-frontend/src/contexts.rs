use std::fmt::format;
use std::rc::Rc;


use gloo_console::log;
use gloo_net::http::Response;
use serde_json::json;
use yew::prelude::*;
use crate::api::user::{MainUser, User};
use gloo_storage::LocalStorage;
use gloo_storage::Storage;

pub type CurrentUserContext = UseReducerHandle<UserContext>;

#[derive(Default, PartialEq)]
pub struct UserContext{
    pub user: Option<MainUser>,
}


impl Reducible for UserContext{

    type Action = CurrentUserDispatchActions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        
        match action.action_type{
           
            CurrentUserActions::LoginSuccess => {
                let login_response = action.login_response.expect("MIssing user");
                let _= LocalStorage::set("user", login_response.clone());
                Self{
                    user: Some(MainUser{
                        id: login_response.id,
                        username: login_response.username,
                        created_at: login_response.created_at,
                    })
                }.into()
               
            },
            CurrentUserActions::LoginFaild => {
                Self{
                    user:None,
                }.into()
            }
        }
        
    }

}

pub struct CurrentUserDispatchActions{
    pub action_type: CurrentUserActions,
    pub login_response: Option<MainUser>,
}
pub enum CurrentUserActions {
    LoginSuccess,
    LoginFaild,
}




#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
#[function_component(CurrentUserProvider)]
pub fn current_user_provider(props: &Props) -> Html {
    let user = use_reducer(UserContext::default);
    if user.user.is_none(){
        match LocalStorage::get::<MainUser>("user") {
            Ok(user_logined) => {
                log!(user_logined.id);
                let id = user_logined.id.clone();
                let username = user_logined.username.clone();
                let created_at = user_logined.created_at.clone();
                user.dispatch(CurrentUserDispatchActions {
                    action_type: CurrentUserActions::LoginSuccess,
                    login_response: Some(MainUser { id, username, created_at }),
                });
            }
            Err(_storage_error) => {  
                let _= "no user loged at";             
            }
        }
        
 
    }
    html!{
        <ContextProvider<CurrentUserContext> context = {user}>
            {props.children.clone()}
        </ContextProvider<CurrentUserContext>>

    }
}