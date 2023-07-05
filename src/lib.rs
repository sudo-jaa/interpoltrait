use std::f32::consts::PI;

type EasingFunction = fn(f32) -> f32;

#[inline]
fn ease_in_sine(t: f32) -> f32 {
    1.0 - f32::cos((t * PI) / 2.0)
}
#[inline]
fn ease_out_sine(t: f32) -> f32 {
    f32::sin((t * PI) / 2.0)
}
#[inline]
fn ease_in_out_sine(t: f32) -> f32 {
    -(f32::cos(PI * t) - 1.0) / 2.0
}

pub trait Interpolatable<Other = Self> {
    fn lerp(&self, other: Other, t: f32) -> Other;

    fn interpolate_custom(&self, other: Other, t: f32, func: EasingFunction) -> Other {
        self.lerp(other, func(t))
    }

    fn interpolate_ease_in_sine(&self, other: Other, t: f32) -> Other {
        self.lerp(other, ease_in_sine(t))
    }
    fn interpolate_ease_out_sine(&self, other: Other, t: f32) -> Other {
        self.lerp(other, ease_out_sine(t))
    }
    fn interpolate_ease_in_out_sine(&self, other: Other, t: f32) -> Other {
        self.lerp(other, ease_in_out_sine(t))
    }
}

#[cfg(feature = "monotonic")]
impl<T: num_traits::Float> Interpolatable for T {
    #[inline(always)]
    /// Imprecise method, which does not guarantee v = v1 when t = 1, due to floating-point arithmetic error.
    /// This method is monotonic. This form may be used when the hardware has a native fused multiply-add instruction.
    fn lerp(&self, other: Self, t: f32) -> Self {
        (*self) + (other - (*self)) * t
    }
}

#[cfg(not(feature = "monotonic"))]
impl<T: num_traits::Float> Interpolatable for T {
    #[inline(always)]
    /// Precise method, which guarantees v = v1 when t = 1. This method is monotonic only when v0 * v1 < 0.
    /// Lerping between same values might not produce the same value
    fn lerp(&self, other: Self, t: f32) -> Self {
        let unwrapped_t = T::from(t).unwrap();
        (T::from(1.0).unwrap() - unwrapped_t) * (*self) + unwrapped_t * (other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a: f32 = 1.0;
        let b: f32 = 2.0;
        let c = a.lerp(b, 0.1);
        println!("{c}");

        let z: f64 = 1.0;
        let y: f64 = 2.0;
        let x = z.lerp(y, 0.8);
        println!("{x}");
    }
}
