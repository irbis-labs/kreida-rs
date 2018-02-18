#![recursion_limit="128"]

#![feature(drain_filter)]
#![feature(iterator_step_by)]
#![feature(link_args)]

// (Default TOTAL_MEMORY is 16777216)
#[cfg_attr(target_arch="wasm32", link_args = "\
    -s TOTAL_MEMORY=268435456\
")]

extern {}

extern crate kreida;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

pub mod effects;

use kreida::*;
use stdweb::web::{
    document,
    Element,
};
use stdweb::unstable::TryInto;

use yew::prelude::*;
use yew::services::Task;
use yew::services::animation::AnimationService;


struct Context {
    animation: AnimationService,
}

struct Model {
    dark_side: bool,
    time: f64,
    canvas: kreida::Canvas,
    canvas_element: Element,
    ctx2d: stdweb::Value,
    job: Option<Box<Task>>,
    frames: Vec<f64>,
    fun: Fun,
}

enum Fun {
    Sinusoid1,
    Sinusoid2,
    Lines,
    Spirograph,
    Wave,
}

enum Msg {
    ToggleDark,
    ToggleFullscreen,
    Select(Fun),
    Start,
    Step(f64),
}

impl Component<Context> for Model {
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model::new()
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<Context, Self>) -> ShouldRender {
        use Msg::*;
        match msg {
            ToggleDark => {
                self.dark_side = !self.dark_side;
                js! {
                    document.body.classList.toggle("dark");
                };
            }
            ToggleFullscreen => {
            }
            Select(fun) => {
                self.fun = fun;
                context.send_back(|_| Start).emit(());
            }
            Start => {
                if self.job.is_none() {
                    let callback = context.send_back(|tm| Step(tm));
                    let handle = context.animation.spawn(callback);
                    self.job = Some(Box::new(handle));
                }
            },
            Step(tm) => {
                self.time = tm / 1000.0;
                self.render_frame();
                self.update_fps();
            }
        };
        true
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <nav class="ui",>
                <h1>
                    { "Kreida demo :: Rust + WAsm + Canvas 2D (" }
                    { if self.dark_side { "dark side" } else { "light side" } }
                    { ")" }
                </h1>
                <menu class="top",>
                    <div class="fps-counter",>
                        { self.frames.len() }
                    </div>
                    <div class="mode",>
                        <div class="mode", onclick=|_| Msg::ToggleDark,>
                            { if self.dark_side { "|" } else { "_" } }
                        </div>
                        <div class="fullscreen", onclick=|_| Msg::ToggleFullscreen,>
                            { "X" }
                        </div>
                    </div>
                </menu>
                <menu class="bottom",>
                    <div onclick=|_| Msg::Select(Fun::Sinusoid1),>
                        { "Sinusoid1" }
                    </div>
                    <div onclick=|_| Msg::Select(Fun::Sinusoid2),>
                        { "Sinusoid2" }
                    </div>
                    <div onclick=|_| Msg::Select(Fun::Lines),>
                        { "Lines" }
                    </div>
                    <div onclick=|_| Msg::Select(Fun::Spirograph),>
                        { "Spirograph" }
                    </div>
                    <div onclick=|_| Msg::Select(Fun::Wave),>
                        { "Wave" }
                    </div>
                </menu>
            </nav>
        }
    }
}

impl Model {
    fn new() -> Self {
        let canvas_element: Element = document().create_element( "canvas" );

        let ctx2d = js! {
            @{&canvas_element}.setAttribute("id", "canvas");
            document.body.appendChild(@{&canvas_element});
            return @{&canvas_element}.getContext("2d");
        };

        let mut model = Model {
            dark_side: false,
            time: 0.0,
            canvas: Canvas::new(320, 240),
            canvas_element,
            ctx2d,
            job: None,
            frames: Vec::with_capacity(64),
            fun: Fun::Wave,
        };

        model.resize();

        model
    }

    pub fn resize(&mut self) {
        let resized: bool = js! {
            var el = @{&self.canvas_element};
            var width = ~~(el.offsetWidth);
            var height = ~~(el.offsetHeight);

            if (
                width !== el.width ||
                height !== el.height
            ) {
                el.width = width;
                el.height = height;
                return true;
            }
            return false;
        }
            .try_into().unwrap();
        if resized {
            let width: u32 = js!( return @{&self.canvas_element}.width; ).try_into().unwrap();
            let height: u32 = js!( return @{&self.canvas_element}.height; ).try_into().unwrap();
            self.canvas.resize(width as usize, height as usize);
        }
    }

    fn render_frame(&mut self) {
        let buf_len = {
            use Fun::*;
            let mut buf = self.canvas.buffer();
            match self.fun {
                Wave => effects::wave(&mut buf, self.time),
                Spirograph => effects::spirograph(&mut buf, self.time),
                Lines => effects::lines(&mut buf, self.time),
                Sinusoid1 => effects::sinusoid1(&mut buf, self.time),
                Sinusoid2 => effects::sinusoid2(&mut buf, self.time),
            };
            buf.len() as u32 * 4
        };
        let buf_ptr = self.canvas.buf_as_ptr() as u32;
        let (width, height) = (self.canvas.width() as u32, self.canvas.height() as u32);
        println!("buf_len: {}, w * h: {}", buf_len, width * height);
        js! {
            var buf = new Uint8ClampedArray(Module.HEAPU8.buffer, @{buf_ptr}, @{buf_len});
            var image_data = new ImageData(buf, @{width}, @{height});
            @{&self.ctx2d}.putImageData(image_data, 0, 0);
        };
    }

    fn update_fps(&mut self) {
        self.frames.push(self.time);
        let low_limit = self.time - 1.0;
        self.frames.drain_filter(|e| *e < low_limit);
    }
}


fn main() {
    yew::initialize();
    let context = Context {
        animation: AnimationService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();
    yew::run_loop();
}
