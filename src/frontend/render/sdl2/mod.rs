use sdl2::render::Canvas;


pub struct Game {
    pub sdl2_context: Box<sdl2::Sdl>,
    pub timer: Box<sdl2::TimerSubsystem>,
    pub video: Box<sdl2::VideoSubsystem>,
    pub keyboard: Box<sdl2::keyboard::KeyboardUtil>,
    pub mouse: Box<sdl2::mouse::MouseUtil>,
    pub audio: Box<sdl2::AudioSubsystem>,
    pub event: Box<sdl2::EventSubsystem>,
    pub event_pump: Box<sdl2::EventPump>,
    pub joystick: Box<sdl2::JoystickSubsystem>,
    pub haptic: Box<sdl2::HapticSubsystem>,
    pub canvas: Box<Canvas<sdl2::video::Window>>,

    //pub liquidworld: Option<Box<salva2d::LiquidWorld>>,
}
impl Game {
    pub fn new() -> Game {
        println!("SDL Version: {:?}", sdl2::version::version());
        
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let timer = sdl_context.timer().unwrap();
        let window = video
            .window("nova", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let canvas = window.into_canvas().build().unwrap();
        let event_pump = sdl_context.event_pump().unwrap();
        let keyboard = sdl_context.keyboard();
        let mouse = sdl_context.mouse();
        let audio = sdl_context.audio().unwrap();
        let joystick = sdl_context.joystick().unwrap();
        let haptic = sdl_context.haptic().unwrap();
        let event = sdl_context.event().unwrap();

        Game {
            sdl2_context: Box::new(sdl_context),
            timer: Box::new(timer),
            video: Box::new(video),
            canvas: Box::new(canvas),
            event: Box::new(event),
            event_pump: Box::new(event_pump),
            keyboard: Box::new(keyboard),
            mouse: Box::new(mouse),
            audio: Box::new(audio),
            joystick: Box::new(joystick),
            haptic: Box::new(haptic),
            //liquidworld: None,
        }
    }
    pub fn get_window(&mut self) -> &sdl2::video::Window {
        return self.canvas.window_mut();
    }
    pub fn set_win_title(&mut self, _title: &str) {
        self.canvas.window_mut().set_title(_title).unwrap();
    }
    pub fn get_canvas(&mut self) -> &mut Canvas<sdl2::video::Window> {
        return &mut self.canvas;
    }
    pub fn set_win_height(&mut self, _height: u32) {
        self.canvas.window_mut().set_size(_height, _height).unwrap();
    }
    pub fn set_win_width(&mut self, _width: u32) {
        self.canvas.window_mut().set_size(_width, _width).unwrap();
    }

    /*pub fn draw_sprite(&mut self, _sprite: &Sprite, _pos: Position) {
        self.canvas
            .copy(
                &_sprite
                    .surface
                    .as_texture(&self.canvas.texture_creator())
                    .unwrap(),
                None,
                Some(sdl2::rect::Rect::new(
                    _pos.x as i32,
                    _pos.y as i32,
                    _sprite.surface.width() as u32,
                    _sprite.surface.height() as u32,
                )),
            )
            .unwrap();
    }

    pub fn make_timer(&mut self, _delay: u32, _callback: Box<dyn FnMut() -> u32 + Send>) -> Timer {
        /*Timer {
            sdl2_timer: self.sdl2_context.timer().unwrap().add_timer(_delay, _callback)
        }*/
        unimplemented!()
    }*/
}