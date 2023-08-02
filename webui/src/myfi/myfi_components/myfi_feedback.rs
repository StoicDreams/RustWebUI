use crate::prelude::*;
use std::borrow::BorrowMut;
use std::collections::HashMap;

pub fn myfi_feedback_button_info() -> DrawerToggleInfo {
    DrawerToggleInfo::builder(
        |_| String::from("Give us your Feedback!"),
        |_| html! {<i class="fa-solid fa-comment" />},
        DynContextsHtml::new(get_render_wrapper),
    )
    .set_drawer(Direction::Top)
    .set_on_confirm("Send Feedback", handle_confirm)
    .build()
}

const FEEDBACK_KEY: &str = "feedback";
const DEFAULT_THANK_YOU: &str = "Thank you for your feedback!";

#[derive(Debug, serde::Deserialize)]
struct StandardResponse {
    message: String,
}

fn get_response_message(response: &str, backup: &str) -> String {
    if response.is_empty() {
        return String::from(backup);
    }
    if let Ok(result) = serde_json::from_str::<StandardResponse>(response) {
        return result.message;
    }
    if let Ok(result) = serde_json::from_str::<String>(response) {
        return result;
    }
    String::from(response)
}

fn handle_confirm(contexts: Contexts) -> bool {
    let input_state = use_input_state(FEEDBACK_KEY, String::default, None);
    let value = input_state.get();
    if value.is_empty() {
        return true;
    }
    let post_data = HashMap::from([("Message", value)]);
    match serde_json::to_string(&post_data) {
        Ok(post_body) => {
            spawn_async!({
                // wasm_bindgen_futures::spawn_local(async move {
                let response = fetch(FetchRequest::new(
                    "https://feedback.myfi.ws/api/new".to_string(),
                    FetchMethod::Post(post_body.to_string()),
                ))
                .await;
                if response.is_ok() {
                    _ = GlobalData::set_data(FEEDBACK_KEY, "");
                    match response.get_result() {
                        Some(result) => {
                            let message = get_response_message(&result, DEFAULT_THANK_YOU);
                            contexts.drawer.set(
                                Dialog::alert(
                                    |_| String::from("Thank you"),
                                    DynContextsHtml::new(move |_| html!({ message.clone() })),
                                )
                                .borrow_mut()
                                .message(),
                            );
                        }
                        None => {
                            contexts.drawer.set(
                                Dialog::alert(
                                    |_| String::from("Thank you"),
                                    DynContextsHtml::new(|_| html!(DEFAULT_THANK_YOU)),
                                )
                                .message(),
                            );
                        }
                    }
                } else {
                    contexts.drawer.set(
                        Dialog::alert(
                            |_| String::from("Feedback Failure"),
                            DynContextsHtml::new(|_| html!("We're sorry, it looks like our server failed to save your feedback. Please wait a moment and try again.")),
                        )
                        .message(),
                    );
                }
            });
            true
        }
        Err(error) => {
            jslog!("Failed to create feedback body: {:?}", error);
            false
        }
    }
}

pub(crate) fn get_render_wrapper(_contexts: Contexts) -> Html {
    html! {
        <GetRender />
    }
}

#[function_component(GetRender)]
pub(crate) fn get_render() -> Html {
    let input_state = use_state(|| GlobalData::get_data_or(FEEDBACK_KEY, String::default));
    let onchange = {
        Callback::from(move |value: String| {
            _ = GlobalData::set_data(FEEDBACK_KEY, value);
        })
    };

    fn show_discord() -> Html {
        let show_discord = get_company_singular() == "Stoic Dreams";
        if !show_discord {
            return html!();
        }
        html! {
            <p>
                {"You can also come "}
                <Link title="Web UI at Stoic Dreams Discord server" href="https://discord.com/channels/972856291909332993/1025781071608037466">{"chat with us on the Stoic Dreams discord server."}</Link>
            </p>
        }
    }

    html! {
        <>
            <Paper class="flex-grow d-flex flex-column gap-1">
                {show_discord()}
                <InputMessage class="flex-grow d-flex flex-column" name="Feedback" value={input_state} onchange={onchange} />
            </Paper>
        </>
    }
}
