use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub send_messages_callback: Callback<String>
}

#[function_component(Chatbox)]
pub fn chat_box(props: &Props) -> Html {

    let new_message_handle = use_state(String::default);
    let new_message = (*new_message_handle).clone();

    let cloned_new_message_handle = new_message_handle.clone();
    let on_message_change = Callback::from(move |e: Event| {
        let target = e.target_dyn_into::<HtmlTextAreaElement>();
        if let Some(textarea) = target {
            cloned_new_message_handle.set(textarea.value());
        }
    });

    let cloned_new_message = new_message.clone();
    let callback = props.send_messages_callback.clone();
    let on_submit = Callback::from(move |_: MouseEvent| {
        callback.emit(cloned_new_message.clone());
        new_message_handle.set("".to_string());
    });

    html! {
        <div class="input-group">
                    <textarea class="form-control" onchange={on_message_change} value={new_message} ></textarea>
                    <button type="submit" class="btn-primary" onclick={on_submit} >{"Send"}</button>
                </div>
    }
    
}
