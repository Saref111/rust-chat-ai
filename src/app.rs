
use leptos::*;
use leptos_meta::*;
// use leptos_router::*;
use crate::{
    ui::{chat_board::*, chat_controls::*}, 
    models::conversation::{Conversation, Message}, 
    api::converse
};


#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (
        conversation,
        set_conversation
    ) = create_signal(Conversation::new());

    let send = create_action(move |text: &String| {
        set_conversation.update(|conversation| {
            conversation.messages.push(Message {
                text: text.to_owned(),
                is_user: true,
            });
        });
        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let typing_message = Message {
                text: "typing...".to_owned(),
                is_user: false,
            };
            set_conversation.update(|conversation| {
                conversation.messages.push(typing_message);
            });
        }
    });

    create_effect(move |_| {
        if let Some(response) = send.input().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            })
        }
    });

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="CHAT AI"/>
        <ChatBoard conversation/>
        <ChatControls send/>
        
        // content for this welcome page
        // <Router>
        //     <main>
        //         <Routes>
        //             <Route path="" view=HomePage/>
        //             <Route path="" view=HomePage/>
        //             <Route path="/*any" view=NotFound/>
        //         </Routes>
        //     </main>
        // </Router>
    }
}




// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to CHAT_AI!"</h1>
//         <button on:click=on_click>"Click Me: " {count}</button>
//     }
// }

// 404 - Not Found
// #[component]
// fn NotFound() -> impl IntoView {
//     // set an HTTP status code 404
//     // this is feature gated because it can only be done during
//     // initial server-side rendering
//     // if you navigate to the 404 page subsequently, the status
//     // code will not be set because there is not a new HTTP request
//     // to the server
//     #[cfg(feature = "ssr")]
//     {
//         // this can be done inline because it's synchronous
//         // if it were async, we'd use a server function
//         let resp = expect_context::<leptos_actix::ResponseOptions>();
//         resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
//     }

//     view! {
//         <h1>"Not Found"</h1>
//     }
// }
