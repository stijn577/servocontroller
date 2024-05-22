use stm32f4xx_hal::gpio::{ErasedPin, PinState};
use stm32f4xx_hal::gpio::{Output, PushPull};

pub struct BusPins {
    pins: [ErasedPin<Output<PushPull>>; 8],
}

impl BusPins {
    pub fn new(pins: [ErasedPin<Output<PushPull>>; 8]) -> Self {
        Self { pins }
    }

    pub(super) fn write(&mut self, mut value: u8) {
        self.pins.iter_mut().for_each(|pin| {
            pin.set_state(PinState::from(value & 1 == 1));
            value >>= 1;
        })
    }
}

// impl Iter
