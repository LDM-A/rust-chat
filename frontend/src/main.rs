use yew::prelude::*;
use yew_hooks::use_websocket;

#[function_component]
fn App() -> Html {
    let messages_handler = use_state(Vec::default);
    let messages = (*messages_handler).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();

    // If ws.message changes -> call function 
    use_effect_with(ws.message,  move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            cloned_messages.push(ws_msg.clone());
            messages_handler.set(cloned_messages);
        }
    });
    html!{
        <ul id="chat">
        {
            messages.iter().map(|m| html!{<li>{m}</li>}).collect::<Html>()
        }
        </ul>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
