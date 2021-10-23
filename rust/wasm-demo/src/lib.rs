#![recursion_limit = "512"]

pub mod effects;

use kreida::*;

use crate::effects::{Effect, Lines, Sinusoid1, Sinusoid2, Spirograph, Wave};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use web_sys::{Document, HtmlCanvasElement};
use yew::prelude::*;
use yew::services::Task;
use yew::services::{ConsoleService, RenderService};

struct Demo {
    this: ComponentLink<Self>,
    document: Document,
    // body: HtmlElement,
    body_node: NodeRef,
    canvas: kreida::Canvas,
    // canvas_element: HtmlCanvasElement,
    canvas_node: NodeRef,
    job: Option<Box<dyn Task>>,
    state: State,
}

struct State {
    dark_side: bool,
    is_fullscreen: bool,
    time: f64,
    #[allow(dead_code)]
    fun: Fun,
    effect: EffectFun,
    frames: Vec<f64>,
}

#[derive(Clone, Copy)]
enum Fun {
    Sinusoid1,
    Sinusoid2,
    Lines,
    Spirograph,
    Wave,
}

enum EffectFun {
    Sinusoid1(Sinusoid1),
    Sinusoid2(Sinusoid2),
    Lines(Lines),
    Spirograph(Spirograph),
    Wave(Wave),
}

enum Msg {
    ToggleDark,
    ToggleFullscreen,
    Resize,
    Select(Fun),
    Start,
    Step(f64),
}

impl Component for Demo {
    type Message = Msg;
    type Properties = ();

    fn create((): Self::Properties, this: ComponentLink<Self>) -> Self {
        let window = web_sys::window().expect("global window");
        let document = window.document().expect("window document");
        // let body = document.body().expect("document body");

        // let canvas_node: HtmlCanvasElement = document
        //     .create_element("canvas")
        //     .unwrap()
        //     .dyn_into()
        //     .unwrap();
        // body.append_child(&canvas_node);

        let on_resize = Closure::wrap(Box::new({
            let this = this.clone();
            move |_: web_sys::Event| this.send_message(Msg::Resize)
        }) as Box<dyn Fn(_)>);
        window.set_onresize(Some(on_resize.as_ref().unchecked_ref()));
        std::mem::forget(on_resize);

        ConsoleService::log("123");

        let state = State {
            dark_side: false,
            is_fullscreen: false,
            time: 0.0,
            frames: Vec::with_capacity(64),
            fun: Fun::Wave,
            effect: EffectFun::Wave(Wave {}),
        };

        Demo {
            this,
            document,
            // body,
            body_node: NodeRef::default(),
            canvas: Canvas::new(320, 240),
            // canvas_element: canvas_node,
            canvas_node: NodeRef::default(),
            job: None,
            state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDark => {
                self.state.dark_side = !self.state.dark_side;
                true
            }
            Msg::ToggleFullscreen => {
                self.toggle_fullscreen();
                true
            }
            Msg::Resize => {
                self.resize();
                false
            }
            Msg::Select(fun) => {
                self.state.fun = fun;
                self.state.effect = match fun {
                    Fun::Wave => EffectFun::Wave(Wave::default()),
                    Fun::Spirograph => EffectFun::Spirograph(Spirograph::default()),
                    Fun::Lines => EffectFun::Lines(Lines::default()),
                    Fun::Sinusoid1 => EffectFun::Sinusoid1(Sinusoid1::default()),
                    Fun::Sinusoid2 => EffectFun::Sinusoid2(Sinusoid2::default()),
                };

                self.this.send_message(Msg::Start);
                true
            }
            Msg::Start => {
                match &mut self.state.effect {
                    EffectFun::Wave(o) => o.set_time(self.state.time),
                    EffectFun::Spirograph(o) => o.set_time(self.state.time),
                    EffectFun::Lines(o) => o.set_time(self.state.time),
                    EffectFun::Sinusoid1(o) => o.set_time(self.state.time),
                    EffectFun::Sinusoid2(o) => o.set_time(self.state.time),
                };
                if self.job.is_none() {
                    self.request_frame();
                    self.resize();
                }
                false
            }
            Msg::Step(tm) => {
                self.state.time = tm / 1000.0;
                self.render_frame();
                self.update_fps();
                self.request_frame();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let this = &self.this;
        let toggle_dark = this.callback(|_| Msg::ToggleDark);
        let toggle_fullscreen = this.callback(|_| Msg::ToggleFullscreen);
        let set_sinusoid1 = this.callback(|_| Msg::Select(Fun::Sinusoid1));
        let set_sinusoid2 = this.callback(|_| Msg::Select(Fun::Sinusoid2));
        let set_spirograph = this.callback(|_| Msg::Select(Fun::Spirograph));
        let set_lines = this.callback(|_| Msg::Select(Fun::Lines));
        let set_wave = this.callback(|_| Msg::Select(Fun::Wave));

        let side = if self.state.dark_side {
            "Dark"
        } else {
            "Light"
        };
        let side_class = if self.state.dark_side {
            "dark"
        } else {
            "light"
        };

        html! {
            <body
                ref = self.body_node.clone()
                class = (side_class)
            >
                <main>
                    <canvas ref = self.canvas_node.clone()></canvas>
                    <nav class="ui">
                        <h1>
                            { format!("Kreida demo :: Rust + WAsm + Canvas 2D ({} side)", side) }
                        </h1>
                        <menu class="top">
                            <div class="fps-counter">
                                { self.state.frames.len() }
                            </div>
                            <div class="mode">
                                <div class="mode" onclick=toggle_dark>
                                    <i class=("fa",
                                        if self.state.dark_side { "fa-sun" } else { "fa-moon" })></i>
                                </div>
                               <div class="fullscreen" onclick=toggle_fullscreen>
                                    <i class=("fa", "fa-expand")></i>
                                </div>
                            </div>
                        </menu>
                        <menu class="bottom",>
                            <div onclick=set_sinusoid1>
                                { "Sinusoid1" }
                            </div>
                            <div onclick=set_sinusoid2>
                                { "Sinusoid2" }
                            </div>
                            <div onclick=set_lines>
                                { "Lines" }
                            </div>
                            <div onclick=set_spirograph>
                                { "Spirograph" }
                            </div>
                            <div onclick=set_wave>
                                { "Wave" }
                            </div>
                        </menu>
                    </nav>
                </main>
            </body>
        }
    }
}

impl Demo {
    fn request_frame(&mut self) {
        let callback = self.this.callback(|tm| Msg::Step(tm));
        let handle = RenderService::request_animation_frame(callback);
        self.job = Some(Box::new(handle));
    }

    pub fn resize(&mut self) {
        let canvas_element: HtmlCanvasElement = self.canvas_node.cast().unwrap();

        let offset_width = canvas_element.offset_width() as u32;
        let offset_height = canvas_element.offset_height() as u32;

        let width = canvas_element.width();
        let height = canvas_element.height();

        let (diff_width, diff_height) = (width != offset_width, height != offset_height);

        if diff_width {
            canvas_element.set_width(offset_width);
        }

        if diff_height {
            canvas_element.set_height(offset_height);
        }

        if diff_width || diff_height {
            self.canvas.resize(offset_width, offset_height);
        }
    }

    fn toggle_fullscreen(&mut self) {
        self.state.is_fullscreen = !self.state.is_fullscreen;
        if self.state.is_fullscreen {
            let body: HtmlElement = self.body_node.cast().unwrap();
            if let Err(_e) = body.request_fullscreen() {
                self.state.is_fullscreen = false;
            }
        } else {
            let _ = self.document.exit_fullscreen();
        };
    }

    fn render_frame(&mut self) {
        let cnv = self.canvas_node.cast::<HtmlCanvasElement>();
        if let Some(cnv) = cnv {
            let ctx = cnv
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            let (width, height) = (self.canvas.width(), self.canvas.height());
            let mut buf = self.canvas.buffer();
            match &mut self.state.effect {
                EffectFun::Wave(o) => o.update(&mut buf, self.state.time),
                EffectFun::Spirograph(o) => o.update(&mut buf, self.state.time),
                EffectFun::Lines(o) => o.update(&mut buf, self.state.time),
                EffectFun::Sinusoid1(o) => o.update(&mut buf, self.state.time),
                EffectFun::Sinusoid2(o) => o.update(&mut buf, self.state.time),
            };
            let clamped = wasm_bindgen::Clamped(buf.as_bytes());
            let image_data =
                web_sys::ImageData::new_with_u8_clamped_array_and_sh(clamped, width, height)
                    .unwrap();

            ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }

    fn update_fps(&mut self) {
        self.state.frames.push(self.state.time);
        let low_limit = self.state.time - 1.0;
        self.state.frames.retain(|e| *e > low_limit);
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Demo>::new().mount_as_body();
}
