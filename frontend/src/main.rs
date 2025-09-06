use common::{WebSocketMessage, WebSocketMessageType};
use yew::prelude::*;
use yew_hooks::use_websocket;
use crate::message_list::MessageList;
use crate::chat_box::Chatbox;
use crate::users_list::UsersList;

mod message_list;
mod chat_box;
mod users_list;

#[function_component]
fn App() -> Html {
    let messages_handler = use_state(Vec::default);
    let messages = (*messages_handler).clone();

    let users_handler = use_state(Vec::default);
    let users = (*users_handler).clone();

    let ws = use_websocket("ws://127.0.0.1:8000".to_string());

    let mut cloned_messages = messages.clone();

    // If ws.message changes -> call function 
    use_effect_with(ws.message.clone(),  move |ws_message| {
        if let Some(ws_msg) = &**ws_message {
            let websocket_message: WebSocketMessage = serde_json::from_str(&ws_msg).unwrap();
            match websocket_message.message_type {
                WebSocketMessageType::NewMessage => {
                    let msg = websocket_message.message.expect("missing message payload");
                    cloned_messages.push(msg);
                    messages_handler.set(cloned_messages);
                },
                WebSocketMessageType::UsersList => {
                    let users = websocket_message.users.expect("missing users payload");
                    users_handler.set(users);
                }
            }
            
        }
    });
    let cloned_ws = ws.clone();
    let send_message_callback = Callback::from(move | msg: String | {
        cloned_ws.send(msg.clone());
    });
    html!{
        <div class="container-fluid">
            <div class="row">

                <div class="col-sm-3">
                    <UsersList users={users}/>
                </div>

                <div class="col-sm-9">
                    <MessageList messages={messages} />
                </div>

            </div>
            <div class="row">
                <Chatbox send_messages_callback={send_message_callback} />
            </div>
        </div>
    }
        

}

fn main() {
    yew::Renderer::<App>::new().render();
}
