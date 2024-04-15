use leptos::*;

use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    }).await.unwrap();
    use llm::KnownModel;
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant";
    let mut history = format!(
        "{character_name}:Hello - How may I help you today\n\
        {user_name}: What is the capital of France?\n\
        {character_name}: Paris is the capital of France\n"
    );

//  Extract all the messages from the current session and arrange them according to the user
//  Push the arranged messages into the history to prepare the chat
    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}:{msg}\n")
        } else {
            format!("{user_name}:{msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    
    let mut session = model.start_session(Default::default());

    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}")
                    .as_str()
                    .into(),
            parameters: Some(&llm::InferenceRequest::default()),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        }
    );

    Ok(String::from(""))
}