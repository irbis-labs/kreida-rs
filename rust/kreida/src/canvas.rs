use crate::Color;
use crate::Buffer;

pub struct Canvas {
    buf: Vec<Color>,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        let len = (width * height) as usize;
        let mut buf = Vec::with_capacity(len);
        buf.resize(len, Color::transparent());
        Canvas { buf, width, height }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if self.width != width || self.height != height {
            // TODO resize image
            self.width = width;
            self.height = height;
            let len = (width * height) as usize;
            self.buf.resize(len, Color::transparent())
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn buffer(&mut self) -> Buffer<'_> {
        Buffer::new(&mut self.buf, self.width)
    }

    // pub fn buf_as_ptr(&self) -> *const Color {
    //     self.buf.as_ptr()
    // }
    //
    // pub fn buf_as_mut_ptr(&mut self) -> *mut Color {
    //     self.buf.as_mut_ptr()
    // }
}
//
// #[no_mangle]
// pub extern "C" fn canvas_new(width: usize, height: usize) -> *mut Canvas {
//     let model = Box::new(Canvas::new(width, height));
//     Box::into_raw(model)
// }
//
// #[no_mangle]
// pub extern "C" fn canvas_resize(canvas: *mut Canvas, width: usize, height: usize) {
//     let canvas = unsafe { canvas.as_mut() }.unwrap();
//     canvas.resize(width, height);
// }
//
// #[no_mangle]
// pub extern "C" fn canvas_buf_as_ptr(canvas: *const Canvas) -> *const Color {
//     let chart = unsafe { canvas.as_ref() }.unwrap();
//     chart.buf_as_ptr()
// }
