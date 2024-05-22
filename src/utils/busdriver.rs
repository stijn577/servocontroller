use defmt::println;
use rtic_monotonics::stm32::fugit::ExtU32;
use rtic_monotonics::systick::Systick;

use crate::{Clk, Set};

pub struct BusDriver {
    clk: Clk,
    set: Set,
    bus: super::buspins::BusPins,
}

impl BusDriver {
    pub fn new(clk_pin: Clk, set_pin: Set, bus_pins: super::buspins::BusPins) -> Self {
        Self {
            clk: clk_pin,
            set: set_pin,
            bus: bus_pins,
        }
    }

    pub async fn write_cycle(&mut self, addr: u8, data: u8) {
        println!("writing {} to address {}...", data, addr);

        self.set.set_high();
        self.clk.set_high();

        Systick::delay(10.millis()).await;

        self.clk.set_low();
        self.bus.write(addr);

        Systick::delay(10.millis()).await;

        self.clk.set_high();

        Systick::delay(10.millis()).await;

        self.clk.set_low();
        self.bus.write(data);

        Systick::delay(10.millis()).await;

        self.clk.set_high();
        self.set.set_low();

        println!("done!");
    }

    pub async fn write_cycle_looped(&mut self, addr: u8, data: u8) {
        self.write_cycle(addr, data).await;

        Systick::delay(10.millis()).await;
        self.clk.set_low();
        Systick::delay(10.millis()).await;
        self.clk.set_high();
        Systick::delay(10.millis()).await;
        self.clk.set_low();
        Systick::delay(10.millis()).await;
    }
}
