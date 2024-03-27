use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct Props {
    pub alert_type: AttrValue,
    pub message: AttrValue,

}
#[function_component(Alert)]
pub fn alert(props: &Props) -> Html {
    let alert_type = format!("alert alert-{}", props.alert_type);
    html!{
        <div class = {alert_type}> {&props.message}</div>
    }
}