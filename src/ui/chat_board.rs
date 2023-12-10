use leptos::*;

use crate::models::conversation::*;

#[component]
pub fn ChatBoard(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let list_ref = create_node_ref::<html::Ul>();

    create_effect(move |_| {
        conversation.get();
        if let Some(ul) = list_ref.get() {
            ul.set_scroll_top(ul.scroll_height())
        }
    });
    view! {
        <ul node_ref=list_ref>
            {move || conversation.get().messages.iter().map(move |m| {
                let class_name = "user-message";
                view! {
                    <li class={class_name}> {m.text.clone()} </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
