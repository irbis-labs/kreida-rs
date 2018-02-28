#![recursion_limit="128"]

#![feature(drain_filter)]
#![feature(iterator_step_by)]
#![feature(link_args)]

// (Default TOTAL_MEMORY is 16777216)
#[cfg_attr(target_arch="wasm32", link_args = "\
    -s TOTAL_MEMORY=268435456 \
    -s ASSERTIONS=1 \
")]

extern {}

extern crate kreida;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;

pub mod effects;

use kreida::*;

use stdweb::unstable::TryInto;
use stdweb::web::{
    document,
    window,
    CanvasRenderingContext2d,
//    Element,
    IEventTarget,
    IHtmlElement,
//    ImageData,
};
use stdweb::web::event::{
//    MouseMoveEvent,
    ResizeEvent,
};
use stdweb::web::html_element::CanvasElement;

use yew::html::ComponentUpdate;
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
    canvas_element: CanvasElement,
    ctx2d: CanvasRenderingContext2d,
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
    Resize,
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
                self.toggle_fullscreen();
            }
            Resize => {
                self.resize();
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
                            <i class=("fa",
                                if self.dark_side { "fa-sun-o" } else { "fa-moon-o" }),></i>
                        </div>
                        <div class="fullscreen",>
//                        <div class="fullscreen", onclick=|_| Msg::ToggleFullscreen,>
                            <i class=("fa", "fa-expand"),></i>
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
        let canvas_element: CanvasElement = document().create_element( "canvas" ).unwrap().try_into().unwrap();
        // FIXME use stdweb api instead of inline js
        js!{
            @{&canvas_element}.setAttribute("id", "canvas");
            document.body.appendChild(@{&canvas_element});
        }

        let ctx2d: CanvasRenderingContext2d = canvas_element.get_context().unwrap();

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

        let offset_width = self.canvas_element.offset_width() as u32;
        let offset_height = self.canvas_element.offset_height() as u32;

        let width = self.canvas_element.width();
        let height = self.canvas_element.height();

        if width != offset_width {
            self.canvas_element.set_width(offset_width);
        }

        if height != offset_height {
            self.canvas_element.set_height(offset_height);
        }

        if width != offset_width || height != offset_height {
            self.canvas.resize(width as usize, height as usize);
        }
    }

    fn toggle_fullscreen(&mut self) {
        println!("toggle_fullscreen");
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
        println!("buf_len: {}, w * h: {}; w x h: {} x {}", buf_len, width * height, width, height);
        // FIXME use stdweb api instead of inline js
//        let image_data = ImageData(Reference());
//        self.ctx2d.put_image_data(image_data);
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
    let mut app: App<_, Model> = App::new(context);

    let env = app.get_env();
    window().add_event_listener( move |_: ResizeEvent| {
        env.sender().send(ComponentUpdate::Message(Msg::Resize));
    });
    js! {
        document.body.addEventListener("click", function(event) {
            if (!event.target.matches(".fullscreen,.fullscreen *")) return;
            var element = document.body;
            var isFullscreen = document.webkitIsFullScreen || document.mozFullScreen || false;

            element.requestFullScreen = element.requestFullScreen || element.webkitRequestFullScreen || element.mozRequestFullScreen || function () { return false; };
            document.cancelFullScreen = document.cancelFullScreen || document.webkitCancelFullScreen || document.mozCancelFullScreen || function () { return false; };

            isFullscreen ? document.cancelFullScreen() : element.requestFullScreen();
        });
    };

    app.mount_to_body();
    yew::run_loop();
}
