mod app;
mod voca;

use app::*;
use leptos::*;

pub fn wasm_main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
