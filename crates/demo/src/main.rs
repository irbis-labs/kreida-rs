use gloo::console;
use kreida_demo::DemoApp;
use yew::Renderer;

fn main() {
    console_error_panic_hook::set_once();
    console::log!("Kreida demo started");
    Renderer::<DemoApp>::new().render();
}
