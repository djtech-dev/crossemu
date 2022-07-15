pub trait Backend {
    fn sqrt_f32(&self, x: f32) -> f32;
    fn sqrt_f64(&self, x: f64) -> f64;
}