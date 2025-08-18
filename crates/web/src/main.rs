use components::Root;
use leptos::prelude::*;
use leptos_meta::*;

fn main() {
    // set up logging
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    provide_meta_context();

    mount_to_body(|| {
        view! {
            <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

            // sets the document title
            <Title text="Welcome to Leptos CSR" />

            // injects metadata in the <head> of the page
            <Meta charset="UTF-8" />
            <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

            <Root />
        }
    })
}
