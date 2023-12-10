use leptos::*;

use crate::models::conversation::*;

#[component]
pub fn ChatBoard(conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        <div> This is chat! </div>
    }
}
