use std::thread::sleep;

use crate::core::{cpu::{CPU, Stack}, backend::Backend, frontend::Graphics};
use super::timer::Timer;

enum CHIP8Opcode {
    Clear = 0x00E0,
    Jump = 0x1000,
    SetVX = 0x6000,
    AddVX = 0x7000,
    SetI = 0xA000,
    Draw = 0xD000
}

pub struct CHIP8Cpu {
    memory: Vec<u8>,
    pc: usize,
    i: usize,
    stack: Stack<usize>,
    delay_timer: Timer,
    sound_timer: Timer,

    backend: Box<dyn Backend>,
    graphics: Box<dyn Graphics>,
}
impl CPU for CHIP8Cpu {
    fn fetch_opcode(self) -> u16 {
        self.memory[self.pc] as u16
    }

    fn execute_opcode(&mut self, _opcode: u16) {
        match _opcode {
            0x00E0 => {
                self.graphics.clear_screen();

                self.memory[0xF] = 0;
                self.pc += 2;
            },
            0x1000 => {
                self.pc = _opcode as usize & 0x0FFF; //?
            },           
            
            _ => {panic!("Unimplemented opcode: {:X}", _opcode);}
        }

    }
}
impl CHIP8Cpu {
    pub fn timer_loop(&mut self) {
        let dur = std::time::Duration::from_secs_f64(1.0 / 60.0);
        loop {
            sleep(dur);
            self.delay_timer.tick();
            self.sound_timer.tick();
        }        
    }
}