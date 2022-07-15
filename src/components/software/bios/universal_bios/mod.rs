use crate::core::bios::BIOS;

pub struct UniversalBIOS {}
impl BIOS for UniversalBIOS {
    fn load_file(&mut self, _path: &str) -> Result<(), String> {
        unimplemented!()
    }
    fn start(&mut self) -> Result<(), String> {
        unimplemented!()
    }
}