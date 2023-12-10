use leptos::*;

#[component]
pub fn ChatControls(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let (input, set_input) = create_signal("".to_owned());
    view! {
        <textarea on:input={move |event| {
            set_input.update(|value| *value = event_target_value(&event));
        }}></textarea>
        <button on:click={move |_| {
            send.dispatch(input.get())
        }}>Send</button>
    }
}
