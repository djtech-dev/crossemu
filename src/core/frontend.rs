pub trait Graphics {
    /// Build and show a new window.
    fn build_window(&mut self, width: u32, height: u32, title: &str);
    
    /// Start the default loading animation. 
    fn start_la(&mut self);
    /// Stop the default loading animation.
    fn stop_la(&mut self);

    /// Clear the entire screen.
    fn clear_screen(&mut self);

}