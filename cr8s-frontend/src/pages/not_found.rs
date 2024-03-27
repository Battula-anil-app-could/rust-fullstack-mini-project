
use yew::prelude::*;

use crate::components::alert::Alert;

#[function_component(NotFound)]
pub fn notfound() -> Html {
    html! {
        <div class = "container">
            <div class="row min-vh-100 d-flex justify-content-center align-items-center">
                <div class = "col-md-4">
                   <h1> <Alert alert_type = "danger" message="404 Not Found"/> </h1>
                </div>
            </div>
        </div>
    }
}
