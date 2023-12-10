use leptos::{ServerFnError, server};
use cfg_if::cfg_if;
use crate::models::conversation::Conversation;

#[server(Converse, "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::{dev::ConnectionInfo, web::Data};
    use rand;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    }).await.unwrap();

    use llm::KnownModel;
    let character_name = "### Assisstant";
    let persona = "I am a very helpful assistant.";
    let user_name = "### Human";
    let mut history = format!("
        {character_name}: Hello, how can I help you today?\n
        {user_name}: What is the capital of France?\n
        {character_name}: Paris.\n
    ");

    for message in &prompt.messages {
        history.push_str(&format!(
            "{}: {}\n", 
            if message.is_user {user_name} else {character_name}, 
            message.text));
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    let mut session = model.start_session(Default::default());
    
    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{persona}\n{history}\n{character_name}").as_str().into(),
            parameters: &llm::InferenceParameters::default(),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut res, &mut buf),
    ).unwrap_or_else(|err| {
        panic!("issue with inference: {:?}", err);
    });

    Ok(res)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::convert::Infallible;


        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            output_string: &'a mut String,
            // tx: tokio::sync::mpsc::Sender<String>,
            // runtime: &'a mut tokio::runtime::Runtime,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {
            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;
        
            move |resp| -> Result<llm::InferenceFeedback, Infallible> {match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        output_string.push_str(t.as_str());
                    } else {
                        output_string.push_str(&reverse_buf);
                    };
        
                    // let tx_cloned = tx.clone();
                    // runtime.block_on(async move {
                    //     tx_cloned.send(text_to_send).await.expect("issue sending on channel");
                    // });
                    // tx_cloned.send(text_to_send).await.expect("issue sending on channel");
        
                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }}
        }
        
    }
}
