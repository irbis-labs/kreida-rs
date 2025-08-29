use gloo::console;
use gloo::events::EventListener;
use gloo::render::AnimationFrame;
use gloo::render::request_animation_frame;
use web_sys::CanvasRenderingContext2d;
use web_sys::Document;
use web_sys::HtmlCanvasElement;
use web_sys::HtmlElement;
use web_sys::ImageData;
use web_sys::wasm_bindgen::Clamped;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::effects::Effect;
use crate::effects::Lines;
use crate::effects::Sinusoid1;
use crate::effects::Sinusoid2;
use crate::effects::Spirograph;
use crate::effects::Wave;

pub mod effects;

pub struct Demo {
    document: Document,
    #[allow(dead_code)]
    on_resize: EventListener,
    #[allow(dead_code)]
    on_fullscreen_change: EventListener,
    // We need to keep a reference to the body element to request fullscreen
    body_node: NodeRef,
    canvas: kreida::Canvas,
    // canvas_element: HtmlCanvasElement,
    canvas_node: NodeRef,
    raf: Option<AnimationFrame>,
    dark_side: bool,
    is_fullscreen: bool,
    is_running: bool,
    time: f64,
    #[allow(dead_code)]
    fun: Fun,
    effect: EffectFun,
    frames: Vec<f64>,
}

#[derive(Clone, Copy)]
pub enum Fun {
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

pub enum Msg {
    ToggleDark,
    ToggleFullscreen,
    Fullscreen(bool),
    Resize,
    Select(Fun),
    Start,
    Step(f64),
}

impl Component for Demo {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().expect("global window");
        let document = window.document().expect("window document");

        let on_resize = EventListener::new(&window, "resize", {
            let link = ctx.link().clone();
            move |_event| link.send_message(Msg::Resize)
        });

        let on_fullscreen_change = EventListener::new(&window, "fullscreenchange", {
            let link = ctx.link().clone();
            let document = document.clone();
            move |_event| {
                let fullscreen_element = document.fullscreen_element();
                let is_fullscreen = fullscreen_element.is_some();
                console::log!("Fullscreen element:", fullscreen_element);
                link.send_message(Msg::Fullscreen(is_fullscreen))
            }
        });

        Demo {
            document,
            on_resize,
            on_fullscreen_change,
            body_node: NodeRef::default(),
            canvas: kreida::Canvas::new(320, 240),
            canvas_node: NodeRef::default(),
            raf: None,
            dark_side: false,
            is_fullscreen: false,
            is_running: false,
            time: 0.0,
            frames: Vec::with_capacity(64),
            fun: Fun::Wave,
            effect: EffectFun::Wave(Wave {}),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleDark => {
                self.dark_side = !self.dark_side;
                true
            }
            Msg::ToggleFullscreen => {
                self.toggle_fullscreen();
                true
            }
            Msg::Fullscreen(is_fullscreen) => {
                self.is_fullscreen = is_fullscreen;
                true
            }
            Msg::Resize => {
                self.resize();
                false
            }
            Msg::Select(fun) => {
                self.fun = fun;
                self.effect = match fun {
                    Fun::Wave => EffectFun::Wave(Wave::default()),
                    Fun::Spirograph => EffectFun::Spirograph(Spirograph::default()),
                    Fun::Lines => EffectFun::Lines(Lines::default()),
                    Fun::Sinusoid1 => EffectFun::Sinusoid1(Sinusoid1::default()),
                    Fun::Sinusoid2 => EffectFun::Sinusoid2(Sinusoid2::default()),
                };

                ctx.link().send_message(Msg::Start);
                true
            }
            Msg::Start => {
                match &mut self.effect {
                    EffectFun::Wave(o) => o.set_time(self.time),
                    EffectFun::Spirograph(o) => o.set_time(self.time),
                    EffectFun::Lines(o) => o.set_time(self.time),
                    EffectFun::Sinusoid1(o) => o.set_time(self.time),
                    EffectFun::Sinusoid2(o) => o.set_time(self.time),
                };
                if !self.is_running {
                    self.is_running = true;
                    self.request_frame(ctx);
                    self.resize();
                }
                false
            }
            Msg::Step(tm) => {
                self.time = tm / 1000.0;
                self.render_frame();
                self.update_fps();
                self.request_frame(ctx);
                true
            }
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let toggle_dark = ctx.link().callback(|_| Msg::ToggleDark);
        let toggle_fullscreen = ctx.link().callback(|_| Msg::ToggleFullscreen);
        let set_sinusoid1 = ctx.link().callback(|_| Msg::Select(Fun::Sinusoid1));
        let set_sinusoid2 = ctx.link().callback(|_| Msg::Select(Fun::Sinusoid2));
        let set_spirograph = ctx.link().callback(|_| Msg::Select(Fun::Spirograph));
        let set_lines = ctx.link().callback(|_| Msg::Select(Fun::Lines));
        let set_wave = ctx.link().callback(|_| Msg::Select(Fun::Wave));

        let side = if self.dark_side { "Dark" } else { "Light" };
        let side_class = if self.dark_side { "dark" } else { "light" };
        let side_icon = if self.dark_side { "fa-sun" } else { "fa-moon" };
        let fullscreen_icon = if self.is_fullscreen {
            "fa-compress"
        } else {
            "fa-expand"
        };

        html! {
            <main
                ref = {&self.body_node}
                class = {side_class}
            >
                <canvas ref = {&self.canvas_node}></canvas>
                <nav class="ui">
                    <h1>
                        { format!("Kreida animation demo :: Rust + WAsm + Canvas 2D ({} side)", side) }
                    </h1>
                    <menu class="top">
                        <div class="fps-counter">
                            { self.frames.len() }
                        </div>
                        <div class="mode">
                            <div class="mode" onclick={toggle_dark}>
                                <i class={["fa", side_icon]}></i>
                            </div>
                           <div class="fullscreen" onclick={toggle_fullscreen}>
                                <i class={["fa", fullscreen_icon]}></i>
                            </div>
                        </div>
                    </menu>
                    <menu class="bottom">
                        <div onclick={set_sinusoid1}>
                            { "Sinusoid1" }
                        </div>
                        <div onclick={set_sinusoid2}>
                            { "Sinusoid2" }
                        </div>
                        <div onclick={set_lines}>
                            { "Lines" }
                        </div>
                        <div onclick={set_spirograph}>
                            { "Spirograph" }
                        </div>
                        <div onclick={set_wave}>
                            { "Wave" }
                        </div>
                    </menu>
                </nav>
            </main>
        }
    }
}

impl Demo {
    fn request_frame(&mut self, ctx: &Context<Self>) {
        let link = ctx.link().clone();
        let handle = request_animation_frame(move |tm| link.send_message(Msg::Step(tm)));
        self.raf = Some(handle);
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
        self.is_fullscreen = !self.is_fullscreen;
        if self.is_fullscreen {
            let body: HtmlElement = self.body_node.cast().unwrap();
            if let Err(_e) = body.request_fullscreen() {
                self.is_fullscreen = false;
            }
        } else {
            let _ = self.document.exit_fullscreen();
        };
    }

    fn render_frame(&mut self) {
        if let Some(canvas_el) = self.canvas_node.cast::<HtmlCanvasElement>() {
            let ctx = canvas_el
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            let (width, height) = (self.canvas.width(), self.canvas.height());
            let mut buf = self.canvas.buffer();
            match &mut self.effect {
                EffectFun::Wave(o) => o.update(&mut buf, self.time),
                EffectFun::Spirograph(o) => o.update(&mut buf, self.time),
                EffectFun::Lines(o) => o.update(&mut buf, self.time),
                EffectFun::Sinusoid1(o) => o.update(&mut buf, self.time),
                EffectFun::Sinusoid2(o) => o.update(&mut buf, self.time),
            };
            let clamped = Clamped(buf.as_bytes());
            let image_data =
                ImageData::new_with_u8_clamped_array_and_sh(clamped, width, height).unwrap();

            ctx.put_image_data(&image_data, 0.0, 0.0).unwrap();
        }
    }

    fn update_fps(&mut self) {
        self.frames.push(self.time);
        let low_limit = self.time - 1.0;
        self.frames.retain(|e| *e > low_limit);
    }
}
