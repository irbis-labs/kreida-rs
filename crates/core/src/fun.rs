pub fn step(v: f64, step: f64) -> f64 {
    (v / step).round() * step
}

pub fn map_range((ra, rb): (f64, f64), (ma, mb): (f64, f64), v: f64) -> f64 {
    let r = rb - ra;
    if r == 0.0 {
        return (ma + mb) / 2.0;
    }
    ma + (v - ra) / r * (mb - ma)
}

pub fn map_sin((ma, mb): (f64, f64), v: f64) -> f64 {
    map_range((-1.0, 1.0), (ma, mb), v.sin())
}

pub fn map_cos((ma, mb): (f64, f64), v: f64) -> f64 {
    map_range((-1.0, 1.0), (ma, mb), v.cos())
}
