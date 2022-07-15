use crate::core::backend::Backend;

pub struct Raft {}
impl Backend for Raft {
    fn sqrt_f32(&self, x: f32) -> f32 {
        return x.sqrt();
    }
    fn sqrt_f64(&self, x: f64) -> f64 {
        return x.sqrt();
    }
}