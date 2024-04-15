use leptos::*;

use crate::model::conversation::Conversation;

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        <div>
        // The first move || conversation.get() tracks every time conversation is updated
        {move || conversation.get().messages.iter().map(move |message| {
            view! {
                <div >
                {message.text.clone()}
                </div>
            }
        }).collect::<Vec<_>>()
        }
        </div>
    }
}
