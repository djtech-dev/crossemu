use crate::core::{backend::Backend, frontend::Graphics};

pub struct GenericArmCPU {
    backend: Box<dyn Backend>,
    graphics: Box<dyn Graphics>,
}