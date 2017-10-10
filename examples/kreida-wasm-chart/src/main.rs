#![feature(iterator_step_by)]

#![feature(link_args)]
//    (Default TOTAL_MEMORY is 16777216)
#[cfg_attr(target_arch="wasm32", link_args = "\
    -s TOTAL_MEMORY=268435456\
")]
//    --js-library site/utilities.js\

extern {}

extern crate kreida;
extern crate webplatform;

use std::sync::Arc;

pub use kreida::*;


fn main() {
    let document = webplatform::init();
    let body = Arc::new(document.element_query("body").unwrap());
    body.html_append("\
        <h1>Kreida Chart :: Rust + WAsm + Canvas 2D</h1>\
    ");
//    let canvas_el = Arc::new(document.element_create("canvas").unwrap());
//    canvas_el.prop_set_str("id", "canvas");
//    body.append(canvas_el.as_ref());
    {
        let body = body.clone();
        document.on("click", move |_| {
            body.class_toggle("black");
        });
    }

    webplatform::spin();
}


#[derive(Debug)]
pub struct Model<T> {
    data: Vec<T>,
    nonce: usize,
}

impl<T> Model<T> {
    pub fn new() -> Self {
        Model {
            data: Default::default(),
            nonce: 0,
        }
    }

    pub fn from_raw<'a>(model: *mut Model<T>) -> &'a mut Self {
        unsafe {model.as_mut()}.unwrap()
    }

    pub fn touch(&mut self) {
        self.nonce += 1;
    }

    pub fn nonce(&self) -> usize {
        self.nonce
    }

    pub fn push(&mut self, v: T) {
        self.data.push(v);
        self.touch();
    }
}


#[no_mangle]
pub extern "C" fn row_new() -> *mut Model<f64> {
    let model = Box::new(Model::new());
    Box::into_raw(model)
}

#[no_mangle]
pub extern "C" fn row_push(model: *mut Model<f64>, value: f64) {
    let model = Model::from_raw(model);
    model.push(value);
}

#[no_mangle]
pub extern "C" fn drop_model(model: *mut Model<f64>) {
    unsafe { Box::from_raw(model) };
}


#[no_mangle]
pub extern "C" fn sinusoid(canvas: *mut Canvas, model: *mut Model<f64>) {
    let model = Model::from_raw(model);
    let canvas = unsafe {canvas.as_mut()}.unwrap();
    let mut buffer = canvas.buffer();
    let h_2 = (buffer.height() / 2) as isize;
    let h_5 = (buffer.height() / 5) as isize;
    let bg = Color::transparent();
    let fg1 = Color::rgb(255, 31, 0);
    let fg2 = Color::rgb(63, 191, 0);

    buffer.clear(bg);

    let view_width = buffer.width() / 10 - 2;
    use std::cmp::min;
    let w = min(model.data.len(), view_width);
    let start = view_width - w;

    for (column, value) in (start .. view_width)
        .zip(model.data[model.data.len() - w ..].iter())
    {
        let ix = (column + 1) * 10;
        //let x = (ix as f64) * 2.0 / (buffer.width() as f64) - 1.0;
        let y = h_5 as f64 * value;
        let iy = h_2 - y.round() as isize;
        let ix = ix as isize;
        let color = if y < 0.0 { fg1 } else { fg2 };
        let rect = Rect::new((ix - 3, h_2), (ix + 4, iy));
        buffer.bar(rect, color);
        buffer.bar(rect.shrink(2), bg);
    }
}
