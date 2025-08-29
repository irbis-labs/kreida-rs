use gloo::console;
use kreida_wasm_demo::Demo;
use yew::Renderer;

fn main() {
    console_error_panic_hook::set_once();
    console::log!("Kreida demo started");
    Renderer::<Demo>::new().render();
}
