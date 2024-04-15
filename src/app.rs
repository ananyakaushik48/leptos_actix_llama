use leptos::*;
use leptos_meta::*;

use crate::{api::converse, components::chat_area::ChatArea, model::conversation::{Conversation, Message}};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Defining the conversation signal
    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send = create_action(move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message)
        });
        converse(conversation.get())
    }); 
    // create_effect essentially works like useEffect from react
//  This is the ... message that will show up on the screen when the model is responding
    create_effect(move |_| {
        // triggers as soon as the user hits send and collects the message in the input  
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            })
        }
    });

    create_effect(move |_| {
        //  triggers whenever a response comes back from the server
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            })
        }
    });
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-ai.css"/>

        // sets the document title
        <Title text="Rust AI"/>
        <ChatArea  conversation />
        // <TypeArea />
    }
}


