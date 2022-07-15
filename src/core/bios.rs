pub trait BIOS {
    fn load_file(&mut self, path: &str) -> Result<(), String>;
    fn start(&mut self) -> Result<(), String>;
}