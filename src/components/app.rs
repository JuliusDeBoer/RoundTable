use leptos::prelude::*;
use wasm_bindgen::prelude::*;
stylance::import_style!(style, "app.module.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class=style::container>
            <h1>"Hello, World!"</h1>
        </main>
    }
}
