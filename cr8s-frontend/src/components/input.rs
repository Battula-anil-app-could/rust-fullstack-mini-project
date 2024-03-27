use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct Props {
    pub input_type: AttrValue,
    pub name: AttrValue,
    pub label: AttrValue,
    pub value: AttrValue,
    pub on_change: Callback<Event>,
}
#[function_component(Input)]
pub fn input(props: &Props) -> Html {
    let input_id = format!("edit-{}", props.name);
    html! {
        <>
            <label for = {input_id.clone()} >{props.label.clone()} </label>
            <input
                id={input_id}
                type = {props.input_type.clone()}
                name = {props.name.clone()}
                value = {props.value.clone()}
                onchange = {props.on_change.clone()}
                class = "form-control"
            />
        </>
    }
}
