use leptos::{ev::MouseEvent, prelude::*};
use wasm_bindgen::prelude::*;
stylance::import_style!(style, "app.module.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (pos, set_pos) = signal((0, 0));
    let (start_pos, set_start_pos) = signal((0, 0));
    let (pos_mod, set_pos_mod) = signal((0, 0));
    let (is_pressed, set_is_pressed) = signal(false);

    let on_down = move |e: MouseEvent| {
        set_pos_mod.set((0, 0));
        set_start_pos.set((e.screen_x(), e.screen_y()));
        set_is_pressed.set(true);
    };

    let on_move = move |e: MouseEvent| {
        set_pos_mod.set((e.screen_x(), e.screen_y()));
    };

    let on_up = move |_: MouseEvent| {
        set_is_pressed.set(false);
        set_pos.set((
            pos.get().0 - (start_pos.get().0 - pos_mod.get().0),
            pos.get().1 - (start_pos.get().1 - pos_mod.get().1),
        ));
    };

    view! {
        <div
            class=style::container
            on:mousedown=on_down
            on:mousemove=on_move
            on:mouseup=on_up
        >
            <div
                class=style::board
                style:left=move || format!("{}px", match is_pressed.get() {
                    false => pos.get().0,
                    true => pos.get().0 - (start_pos.get().0 - pos_mod.get().0)
                })
                style:top=move || format!("{}px", match is_pressed.get() {
                    false => pos.get().1,
                    true => pos.get().1 - (start_pos.get().1 - pos_mod.get().1)
                })
                />
        </div>
    }
}
