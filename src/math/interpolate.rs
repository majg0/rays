use std::ops::{Add, Mul};

pub fn lerp<T>(a: T, b: T, t: f64) -> <<f64 as Mul<T>>::Output as Add>::Output
where
    f64: Mul<T>,
    <f64 as Mul<T>>::Output: Add,
{
    (1.0 - t) * a + t * b
}
