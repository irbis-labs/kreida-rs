use num::Num;

use self::FixResult::*;

pub enum FixResult<T> {
    Original(T),
    Fixed(T),
}

impl<T> FixResult<T> {
    pub fn into_inner(self) -> T {
        match self {
            FixResult::Original(value) => value,
            FixResult::Fixed(value) => value,
        }
    }

    pub fn into_inner_checked(self) -> (T, bool) {
        match self {
            FixResult::Original(value) => (value, true),
            FixResult::Fixed(value) => (value, false),
        }
    }

    pub fn is_original(&self) -> bool {
        match *self {
            FixResult::Original(_) => true,
            _ => false,
        }
    }

    pub fn is_fixed(&self) -> bool {
        match *self {
            FixResult::Fixed(_) => true,
            _ => false,
        }
    }

    pub fn into_original(self) -> Option<T> {
        match self {
            FixResult::Original(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_fixed(self) -> Option<T> {
        match self {
            FixResult::Fixed(value) => Some(value),
            _ => None,
        }
    }

    /// Maps an CheckResult<T> to CheckResult<U> by applying a function to a contained value.
    pub fn map<U, F>(self, f: F) -> FixResult<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            FixResult::Original(value) => FixResult::Original(f(value)),
            FixResult::Fixed(value) => FixResult::Fixed(f(value)),
        }
    }

    pub fn zip(self, other: Self) -> FixResult<(T, T)> {
        match self.is_original() && other.is_original() {
            true => Original((self.into_inner(), other.into_inner())),
            false => Fixed((self.into_inner(), other.into_inner())),
        }
    }
}

#[inline]
pub fn sort<T>(a: T, b: T) -> FixResult<(T, T)>
where
    T: PartialOrd,
{
    if a > b {
        Fixed((b, a))
    } else {
        Original((a, b))
    }
}

#[inline]
pub fn clip_value<T>(value: T, a: T, b: T) -> FixResult<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    let (min, max) = sort(a, b).into_inner();
    match value {
        v if v < min => Fixed(min),
        v if v >= max => Fixed(max - T::one()),
        v => Original(v),
    }
}

#[inline]
pub fn shrink_range<T>((a, b): (T, T), n: T) -> (T, T)
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    if b - a < n + n {
        let t = (b + a) / (T::one() + T::one());
        (t, t)
    } else {
        (a + n, b - n)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Point<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    fn from((x, y): (T, T)) -> Self {
        Point { x, y }
    }
}

impl<T> Point<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Point { x, y }
    }

    pub fn into_tuple(self) -> (T, T) {
        (self.x, self.y)
    }

    pub fn clip(self, rect: Rect<T>) -> FixResult<Self> {
        let x = clip_value(self.x, rect.a.x, rect.b.x);
        let y = clip_value(self.y, rect.a.y, rect.b.y);
        x.zip(y).map(Self::from)
    }

    pub fn transpose(self) -> Self {
        Point::new(self.y, self.x)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    pub a: Point<T>,
    pub b: Point<T>,
}

impl<T, A, B> From<(A, B)> for Rect<T>
where
    A: Into<Point<T>>,
    B: Into<Point<T>>,
    T: Num + PartialOrd + PartialEq + Copy,
{
    fn from((a, b): (A, B)) -> Self {
        Rect::new(a, b)
    }
}

impl<T> From<(T, T, T, T)> for Rect<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    fn from((ax, ay, bx, by): (T, T, T, T)) -> Self {
        Rect::new((ax, ay), (bx, by))
    }
}

impl<T> Rect<T>
where
    T: Num + PartialOrd + PartialEq + Copy,
{
    pub fn new<A, B>(a: A, b: B) -> Self
    where
        A: Into<Point<T>>,
        B: Into<Point<T>>,
    {
        return Rect {
            a: a.into(),
            b: b.into(),
        }
        .norm();
    }

    pub fn norm(self) -> Self {
        let (ax, bx) = sort(self.a.x, self.b.x).into_inner();
        let (ay, by) = sort(self.a.y, self.b.y).into_inner();
        Rect {
            a: Point::new(ax, ay),
            b: Point::new(bx, by),
        }
    }

    pub fn shrink(self, n: T) -> Self {
        let (a, b) = self.into_tuple();
        let ((ax, ay), (bx, by)) = (a.into_tuple(), b.into_tuple());
        let (ax, bx) = shrink_range((ax, bx), n);
        let (ay, by) = shrink_range((ay, by), n);
        Rect {
            a: Point::new(ax, ay),
            b: Point::new(bx, by),
        }
    }

    pub fn into_tuple(self) -> (Point<T>, Point<T>) {
        (self.a, self.b)
    }

    pub fn clip(self, clip_rect: impl Into<Rect<T>>) -> FixResult<Self> {
        let clip_rect = clip_rect.into();
        let a = self.a.clip(clip_rect);
        let b = self.b.clip(clip_rect);
        a.zip(b).map(Self::from)
    }

    pub fn is_collapsed(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shrink_range_works() {
        assert_eq!(shrink_range::<isize>((1, 5), 0), (1, 5));
        assert_eq!(shrink_range::<isize>((1, 5), 1), (2, 4));
        assert_eq!(shrink_range::<isize>((1, 5), 2), (3, 3));
        assert_eq!(shrink_range::<isize>((1, 5), 4), (3, 3));

        assert_eq!(shrink_range::<isize>((2, 5), 1), (3, 4));
        assert_eq!(shrink_range::<isize>((2, 5), 2), (3, 3));
        assert_eq!(shrink_range::<isize>((2, 5), 3), (3, 3));

        assert_eq!(shrink_range::<isize>((3, 5), 1), (4, 4));
        assert_eq!(shrink_range::<isize>((3, 5), 2), (4, 4));
    }

    #[test]
    fn rect_shrink_works() {
        let rect = Rect::<isize>::new((1, 2), (5, 4));

        assert_eq!(rect.shrink(0), Rect::new((1, 2), (5, 4)));
        assert_eq!(rect.shrink(1), Rect::new((2, 3), (4, 3)));
        assert_eq!(rect.shrink(2), Rect::new((3, 3), (3, 3)));
        assert_eq!(rect.shrink(3), Rect::new((3, 3), (3, 3)));
    }
}
