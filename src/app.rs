use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::model::conversation::Conversation;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Defining the conversation signal
    let (conversation, set_conversation) = create_signal(Conversation::new());
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/rust-ai.css"/>

        // sets the document title
        <Title text="Rust AI"/>
        
    }
}


