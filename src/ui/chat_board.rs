use leptos::*;

use crate::models::conversation::*;

#[component]
pub fn ChatBoard(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let textarea_ref = create_node_ref::<html::Textarea>();
    view! {
        <ul>
            {move || conversation.get().messages.iter().map(move |m: Message| {
                let class_name = "user-message";
                view! {
                    <li class={class_name}> {m.text.clone()} </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
