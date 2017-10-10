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

use kreida::*;
use kreida::fun::*;


fn main() {
    let document = webplatform::init();
    let body = Arc::new(document.element_query("body").unwrap());
    body.html_append("\
        <h1>Kreida demo :: Rust + WAsm + Canvas 2D</h1>\
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

//    let chart = {
//        let width = canvas_el.prop_get_i32("width") as usize;
//        let height = canvas_el.prop_get_i32("height") as usize;
//        Arc::new(RefCell::new(Chart::new(width, height)))
//    };
//    {
//        let body = body.clone();
//        let chart = chart.clone();
//        let mut tm = 0 as f64;
//        document.on("tick", move |_| {
//            tm += 0.016;
//            let mut chart = chart.borrow_mut();
//            let (w, h) = (chart.width(), chart.height());
//            rotating(w * h, chart.as_mut_ptr(), w, tm * 1000.0 as u64);
//        });
//    }
//    {
//        let canvas_el = canvas_el.clone();
//        let chart = chart.clone();
//        document.on("resize", move |_| {
//            let width = canvas_el.prop_get_i32("width") as usize;
//            let height = canvas_el.prop_get_i32("height") as usize;
//            let need_update = {
//                let chart = chart.borrow();
//                chart.width() != width || chart.height() != height
//            };
//            if need_update {
//                let mut chart = chart.borrow_mut();
//                println!("resize; ({}, {}) => ({}, {})", width, height, chart.width(), chart.height());
//                chart.update_size(width, height);
//            }
//        });
//    }
//    println!("canvas width: {:?}", canvas_el.prop_get_i32("width"));
    webplatform::spin();
}


#[no_mangle]
pub extern "C" fn rotating(len: usize, ptr: *mut Color, width: usize, phi: u64) {
    let mut buffer = Buffer::from_row(ptr, len, width);
    let h_2 = (buffer.height() / 2) as isize;
    let w_2 = (buffer.width() / 2) as isize;

    let phi = phi as f64 / 1000.0;

    buffer.fade(200);

    const N: isize = 80;
    use std::f64::consts::PI;
    const PI2: f64 = PI * 2.0;
    use std::cmp::min;
    let size = min(h_2 * 2, w_2) as f64 * 1.5;
    let size_kx = size / N as f64;
    let size_ky = size / N as f64 / 2.0;

    let ta = phi * PI2 / 10.0;
    let pa_4 = PI * (ta / 4.0).sin();

    for rn in 0 .. N {
        let r = rn as f64 / N as f64;
        let rkx = rn as f64 * size_kx;
        let rky = rn as f64 * size_ky;

        let m = (rkx * PI2 / 24.0) as isize;
        let pi2_m = PI2 / (m as f64);

        let rkb = (2.0 + (pa_4 / 10.0).sin()) / 3.0;

        for dn in 0 .. m {
            let d = dn as f64 * pi2_m;
            let dkb = 2.0 * (ta * 0.25 - r * PI2 * 0.5).sin() * (2.0 - r);
            let dkr = 1.5 * (ta * 3.0 - r * PI2 * 3.0).sin();

            let wings = 7.0;

            let a = d + dkb * (1.0 - r / 2.0);
            let b = (d * wings).sin();
            let k = rkb + (d * wings).cos().abs() / N as f64;

            let c = a * 2.0 + r * 4.0 - ta * 2.0;
            let color = Color::rgb(
                ((0.5 + 0.5 * (0.03 * ta + c).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (0.05 * ta + c).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (0.07 * ta + c).sin()) * 255.0) as u8,
            );

            let x = a.cos();
            let y = a.sin();

            let ty = - (r * b + dkr) * size * 0.015;

            let x = w_2 + (     rkx * k * x).round() as isize;
            let y = h_2 + (ty + rky * k * y).round() as isize;
            buffer.bar((x, y, x + 2, y + 2), color);
//            canvas.point(x, y, color);
        }
    }
}

#[no_mangle]
pub extern "C" fn lines(len: usize, ptr: *mut Color, width: usize, phi: usize) {
    let mut buffer = Buffer::from_row(ptr, len, width);
    let w_2: isize = buffer.width() as isize / 2;
    let h_2: isize = buffer.height() as isize / 2;
    let phi = phi as f64 / 1000.0;

    use std::f64::consts::PI;
    use std::cmp::min;

    buffer.fade(245);

    let r = min(h_2, w_2) as f64;
    const N: isize = 32;
    for i in 0 .. N {
        let tt = phi * PI * 0.5 * 5.0 + PI * i as f64 / (N * 4) as f64;

        let a = Point::new(w_2 + (r * (tt * 0.5).cos()) as isize, h_2 + (r * (tt * 0.52).sin()) as isize);
        let b = Point::new(w_2 + (r * (tt * 1.52).cos()) as isize, h_2 + (r * (tt * 1.5).sin()) as isize);

        let c1 = PI  / 2.0 * (i % N) as f64 / N as f64;
        let color = Color::rgb(
            ((0.5 + 0.5 * (0.003 * tt + c1).sin()) * 255.0) as u8,
            ((0.5 + 0.5 * (0.005 * tt + c1).sin()) * 255.0) as u8,
            ((0.5 + 0.5 * (0.007 * tt + c1).sin()) * 255.0) as u8,
        );

        buffer.line(a, b, color);
//        canvas.point(a.x, a.y, fg);
//        canvas.point(b.x, b.y, fg);
    }
}

#[no_mangle]
pub extern "C" fn spirograph(len: usize, ptr: *mut Color, width: usize, phi: usize) {
    let mut buffer = Buffer::from_row(ptr, len, width);
    let w_2: isize = buffer.width() as isize / 2;
    let h_2: isize = buffer.height() as isize / 2;
    let phi = phi as f64 / 1000.0;
    let r = min(h_2, w_2) as f64;

    use std::f64::consts::PI;
    use std::cmp::min;

    buffer.fade(254);

    let t = phi * PI;

    const M: isize = 1;
    const N: isize = 2000;
    for m in 0..M {
        for n in 0..N {
            let tt = t - n as f64 / N as f64 / 5.0;

            let tr1 = step(tt / 13.3, 7.7) / 7.7;
            let tr2 = step(tt / 17.7, 3.3) / 3.3;

            let r0 = 1.0;

            let r1 = r0 * step(map_sin((3.0/23.0, 22.0/23.0), tr1).sqrt(), 1.0 / 19.0);
            let r2 = r1 * step(map_sin((2.0/17.0, 16.0/17.0), tr2).sqrt(), 1.0 / 14.0);
//            let r1 = r0 * (0.05 + 0.9 * (tr1.sin() * 0.5 + 0.5));
//            let r2 = r1 * (0.05 + 0.9 * (tr2.sin() * 0.5 + 0.5));

            let tc = tt * 0.005;

            let tm = PI * m as f64 / M as f64;
            let t1 = tt * 10.0;
            let t2 = - (t1 * r0 / r1);

            let x = (r0 - r1) * (tm + t1).cos() + (r2) * t2.cos();
            let y = (r0 - r1) * (tm + t1).sin() + (r2) * t2.sin();
            let a = Point::new(w_2 + (r * x) as isize, h_2 + (r * y) as isize);

            let color = Color::rgb(
                ((0.5 + 0.5 * (3.0 * tc).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (5.0 * tc).sin()) * 255.0) as u8,
                ((0.5 + 0.5 * (7.0 * tc).sin()) * 255.0) as u8,
            );

            buffer.bar((a.x, a.y, a.x + 2, a.y + 2), color);
        }
    }
}

#[no_mangle]
pub extern "C" fn sinusoid1(len: usize, ptr: *mut Color, width: usize, phi: usize) {
    let mut buffer = Buffer::from_row(ptr, len, width);
    let h_2 = buffer.height() / 2;
    let h_3 = buffer.height() / 5;
    let bg = Color::transparent();
    let fg = Color::black().red(255).green(191);
    let phi = phi as f64 / 1000.0;

    buffer.clear(bg);

    const N: usize = 0;
    const M: usize = 30;

    for i in (N .. M).rev() {
        let ii = i as f64 / M as f64;
        let phi = phi * 2.0 - (ii * 2.0);
        let fg = fg.alpha(255 - (ii * 255.0) as u8);

        for ix in (10 .. width - 10).rev().step_by(10) {
            let x = (ix as f64) * 2.0 / (width as f64) - 1.0;
            let y = h_3 as f64 * ((x * 12.0 - phi).cos() * (x * 4.0 - phi * 0.9).cos());
            let iy = h_2 as isize - y.round() as isize;
            let ix = (ix + i * 2) as isize;
            let d = (M - i) as isize / 1;
            let (x1, y1, x2, y2) = (ix, iy, ix + d, iy + d);
            let rect: Rect<_> = (x1, y1, x2, y2).into();
            buffer.bar(rect, fg);
            buffer.bar(rect.shrink(2), fg.alpha(fg.a / 4));
        }
    }
}

#[no_mangle]
pub extern "C" fn sinusoid2(len: usize, ptr: *mut Color, width: usize, phi: usize) {
    let mut buffer = Buffer::from_row(ptr, len, width);
    let h_2 = (buffer.height() / 2) as isize;
    let h_5 = (buffer.height() / 5) as isize;
    let bg = Color::transparent();
    let fg1 = Color::rgb(255, 31, 0);
    let fg2 = Color::rgb(63, 191, 0);
    let phi = phi as f64 / 1000.0;

//    buffer.clear(bg);
    buffer.fade(240);

    for ix in (10 .. width - 10).step_by(10) {
        let x = (ix as f64) * 2.0 / (width as f64) - 1.0;
        let y = h_5 as f64 * ((x * 12.0 + phi * 2.5).cos() + (x * 4.0 - phi * 1.7).cos());
        let iy = h_2 - y.round() as isize;
        let ix = ix as isize;
        let color = if y < 0.0 { fg1 } else { fg2 };
        let rect = Rect::new((ix - 3, h_2), (ix + 4, iy));
        buffer.bar(rect, color);
        buffer.bar(rect.shrink(2), bg);
    }
}
