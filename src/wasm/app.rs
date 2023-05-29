use leptos::ev::Event;
use leptos::html::Input;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::{File, FileReader, ProgressEvent};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Erase log when it is compiled into release mode
#[cfg(not(debug_assertions))]
fn log(_s: &str) {}

macro_rules! bind {
    (Option | $to_bind: ident := $expr: expr, $msg: expr) => {
        let $to_bind = match $expr {
            Some(val) => val,
            None => {
                error($msg);
                return;
            }
        };
    };
    (Result | $to_bind: ident := $expr: expr) => {
        let $to_bind = match $expr {
            Ok(val) => val,
            Err(err) => {
                error(&format!("{err:?}"));
                return;
            }
        };
    };
    (Result | $to_bind: ident := $expr: expr, $msg: expr) => {
        let $to_bind = match $expr {
            Ok(val) => val,
            Err(_) => {
                error($msg);
                return;
            }
        };
    };
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let (toml_content, set_toml_content) = create_signal::<String>(cx, String::with_capacity(450));
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_change = move |ev: Event| {
        log(&format!("{ev:?}"));
        ev.prevent_default();
        let files_from_web = {
            bind!(Option | input := input_element(), "This is not intended <input>");
            bind!(Option | files := input.files(), "This <input> type is not \"file\".");
            files
        };
        log(&format!("{files_from_web:?}"));

        let mut files: Vec<File> = Vec::with_capacity(10);
        let mut idx = 0;
        while let Some(file) = files_from_web.item(idx) {
            files.push(file);
            idx += 1;
        }
        log(&format!("{files:?}"));

        for file in files {
            bind!(Result | file_reader := FileReader::new());
            let fr_c = file_reader.clone();
            let onloadend_cb = Closure::wrap(Box::new(move |_e: ProgressEvent| {
                bind!(Result | raw_content := fr_c.result(), "Cannot get the file content");
                bind!(
                    Option | content := raw_content.as_string(),
                    &format!("Expected string, got {:?}", raw_content.js_typeof())
                );
                set_toml_content.update(|s| s.push_str(content.as_str()));
                set_toml_content.update(|s| s.push_str("\n\n\n\n\n"));
            }) as Box<dyn Fn(ProgressEvent)>);
            file_reader.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
            file_reader.read_as_text(&file).expect("blob not readable");
            onloadend_cb.forget();
        }
    };

    view! { cx,
        <h1 class="my-6 lg:text-4xl text-2xl font-bold text-slate-100/90">
            "Vrot Vocabulary Memorizing Helper"
        </h1>
        <div>
            <p class="mb-4 text-lg font-semibold text-slate-100/90">"Input .toml files"</p>
            <input type="file" id="file-reader" accept=".toml" multiple
            class="block w-full text-sm text-slate-500
            file:mr-4 file:py-2 file:px-4 file:rounded-full file:border-cyan-200
            file:border-solid file:text-sm file:font-semibold file:bg-cyan-100
            file:text-cyan-900 hover:file:bg-violet-100 hover:file:border-violet-200"
            node_ref=input_element
            on:change=on_change
            />
        </div>
        <p class="text-slate-100">{ toml_content }</p>
    }
}
