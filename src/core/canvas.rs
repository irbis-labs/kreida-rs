use ::*;


pub struct Canvas {
    buf: Vec<Color>,
    width: usize,
    height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let len = width * height;
        let mut buf = Vec::with_capacity(len);
        buf.resize(len, Color::transparent());
        Canvas { buf, width, height }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width != width || self.height != height {
            // TODO resize image
            self.width = width;
            self.height = height;
            self.buf.resize(width * height, Color::transparent())
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn buffer(&mut self) -> Buffer<'static> {
        Buffer::from_raw(self.buf.as_mut_ptr(), self.buf.len(), self.width)
    }

    pub fn buf_as_ptr(&self) -> *const Color {
        self.buf.as_ptr()
    }

    pub fn buf_as_mut_ptr(&mut self) -> *mut Color {
        self.buf.as_mut_ptr()
    }
}


#[no_mangle]
pub extern "C" fn canvas_new(width: usize, height: usize) -> *mut Canvas {
    let model = Box::new(Canvas::new(width, height));
    Box::into_raw(model)
}

#[no_mangle]
pub extern "C" fn canvas_resize(canvas: *mut Canvas, width: usize, height: usize) {
    let canvas = unsafe {canvas.as_mut()}.unwrap();
    canvas.resize(width, height);
}

#[no_mangle]
pub extern "C" fn canvas_buf_as_ptr(canvas: *const Canvas) -> *const Color {
    let chart = unsafe {canvas.as_ref()}.unwrap();
    chart.buf_as_ptr()
}
