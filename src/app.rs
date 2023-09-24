use leptos::*;
use wasm_bindgen::prelude::*;

mod markdown_elements;
use markdown_elements::Text;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {cx,
        <p>Hello Glaggle!</p>
        <Text content="glaggle".into()/>
    }
}
