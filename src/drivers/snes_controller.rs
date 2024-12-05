use crate::peanut_gb::{
    JOYPAD_A, JOYPAD_B, JOYPAD_DOWN, JOYPAD_LEFT, JOYPAD_RIGHT, JOYPAD_SELECT, JOYPAD_START,
    JOYPAD_UP,
};
use std::{thread, time::Duration};

use svc::hal::{
    delay::Ets,
    gpio::{Input, InputPin, Output, OutputPin, PinDriver},
};

pub struct SNESController<'a, CLK, LATCH, DATA>
where
    CLK: OutputPin,
    LATCH: OutputPin,
    DATA: InputPin,
{
    clock: PinDriver<'a, CLK, Output>,
    latch: PinDriver<'a, LATCH, Output>,
    data: PinDriver<'a, DATA, Input>,
}

impl<'a, CLK, LATCH, DATA> SNESController<'a, CLK, LATCH, DATA>
where
    CLK: OutputPin,
    LATCH: OutputPin,
    DATA: InputPin,
{
    pub fn new(clock: CLK, latch: LATCH, data: DATA) -> SNESController<'a, CLK, LATCH, DATA> {
        SNESController {
            clock: PinDriver::output(clock).unwrap(),
            latch: PinDriver::output(latch).unwrap(),
            data: PinDriver::input(data).unwrap(),
        }
    }
    pub fn read(&mut self) -> u16 {
        self.latch.set_high().unwrap();
        Ets::delay_us(12);
        self.latch.set_low().unwrap();
        let mut state = 0_u16;
        Ets::delay_us(12);
        for n in 0..16_u32 {
            self.clock.set_high().unwrap();
            let b = self.data.is_low() as u16;
            state |= b << (15 - n);
            Ets::delay_us(6);
            self.clock.set_low().unwrap();
            Ets::delay_us(6);
        }
        state
    }
    // 1100000000000000
    // 0010000000000000
    // BBssudlraa
    pub fn read_gb(&mut self) -> u8 {
        let input = self.read();
        let mut state = 0;
        if input & 0xC000 != 0 {
            // Y = B
            state |= JOYPAD_B;
        }
        if input & 0x2000 != 0 {
            state |= JOYPAD_SELECT;
        }
        if input & 0x1000 != 0 {
            state |= JOYPAD_START;
        }
        if input & 0x800 != 0 {
            state |= JOYPAD_UP;
        }
        if input & 0x400 != 0 {
            state |= JOYPAD_DOWN;
        }
        if input & 0x200 != 0 {
            state |= JOYPAD_LEFT;
        }
        if input & 0x100 != 0 {
            state |= JOYPAD_RIGHT;
        }
        if input & 0xC0 != 0 {
            state |= JOYPAD_A;
        }
        !state
    }
}
