use crate::core::backend::Backend;

use super::heaven::Heaven;

/// Implementation of the Heaven JIT compile 
/// for the x86 architecture.
impl Backend for Heaven {
    fn sqrt_f32(&self, x: f32) -> f32 {
        // This is a small function,
        // so I didn't bother to implement
        // the JIT compiler.
        // Same for sqrt_f64.
        return x.sqrt();
    }
    fn sqrt_f64(&self, x: f64) -> f64 {
        return x.sqrt();
    }
}