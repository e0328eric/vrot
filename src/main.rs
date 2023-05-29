#[cfg(not(target_family = "wasm"))]
use vrot::cli::cli_main;
#[cfg(target_family = "wasm")]
use vrot::wasm::wasm_main;

fn main() {
    #[cfg(not(target_family = "wasm"))]
    match cli_main() {
        Ok(()) => {}
        Err(err) => {
            eprintln!("{err}");
        }
    }
    #[cfg(target_family = "wasm")]
    wasm_main()
}
