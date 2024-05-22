#![deny(unsafe_code)]
// #![deny(warnings)]
#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(proc_macro_hygiene)]

extern crate alloc;
extern crate core;

use defmt_rtt as _;
use panic_probe as _;

use defmt::{info, println};
use rtic_monotonics::systick::*;
use stm32f4xx_hal::gpio::{Edge, ExtiPin};
use stm32f4xx_hal::syscfg::SysCfgExt;
use stm32f4xx_hal::{gpio::GpioExt, prelude::_stm32f4xx_hal_rcc_RccExt};

// defining our own modules
mod error;
mod prelude;
mod setup;
mod types;
mod utils;

// our own crate libraries
use prelude::*;
use setup::*;
use utils::{busdriver::BusDriver, buspins::BusPins};

#[rtic::app(device=stm32f4xx_hal::pac, peripherals=true, dispatchers=[USART1, USART2, USART6])]
mod app {

    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local<T> {
        led: Led,
        done: Done,
        bus_driver: BusDriver,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        println!("Setting up heap...");
        setup::heap_setup();
        println!("Done!");

        println!("Taking ownership of peripherals...");
        let mut pp = ctx.device; // device peripherals
        println!("Done!");

        println!("Configuring clocks...");
        let rcc = pp.RCC.constrain();
        let _clocks = clock_config(rcc);
        let mut syscfg = pp.SYSCFG.constrain();
        println!("Done!");

        println!("Configuring systick token...");
        // Initialize the systick interrupt & obtain the token to prove that we did
        let systick_mono_token = rtic_monotonics::create_systick_token!();
        Systick::start(ctx.core.SYST, 84_000_000, systick_mono_token);
        println!("Done!");

        println!("Setting up gpio pins...");
        let gpioa = pp.GPIOA.split();
        let gpiob = pp.GPIOB.split();
        let gpioc = pp.GPIOC.split();

        let led: Led = gpioc.pc13.into_push_pull_output();
        let clk: Clk = gpiob.pb0.into_push_pull_output();
        let set: Set = gpiob.pb1.into_push_pull_output();
        let mut done = gpiob.pb2.into_pull_up_input();
        done.make_interrupt_source(&mut syscfg);
        done.enable_interrupt(&mut pp.EXTI);
        done.trigger_on_edge(&mut pp.EXTI, Edge::RisingFalling);

        let pa0 = gpioa.pa0.into_push_pull_output().erase();
        let pa1 = gpioa.pa1.into_push_pull_output().erase();
        let pa2 = gpioa.pa2.into_push_pull_output().erase();
        let pa3 = gpioa.pa3.into_push_pull_output().erase();
        let pa4 = gpioa.pa4.into_push_pull_output().erase();
        let pa5 = gpioa.pa5.into_push_pull_output().erase();
        let pa6 = gpioa.pa6.into_push_pull_output().erase();
        let pa7 = gpioa.pa7.into_push_pull_output().erase();

        println!("Done!");

        println!("Creating busdriver...");

        let bus = BusDriver::new(
            clk,
            set,
            BusPins::new([pa0, pa1, pa2, pa3, pa4, pa5, pa6, pa7]),
        );

        println!("Done!");

        println!("Spawning tasks...");
        active_main::spawn().ok();
        println!("Done!");

        info!("FINISH INIT");

        (
            Shared {},
            Local {
                led,
                done,
                bus_driver: bus,
            },
        )
    }

    #[task(binds=EXTI2, local=[led, done])]

    fn passive_main(ctx: passive_main::Context) {
        let led = ctx.local.led;
        let done = ctx.local.done;

        if done.is_high() {
            info!("DONE!");
            led.set_low();
        } else {
            led.set_high();
        }

        done.clear_interrupt_pending_bit();
    }

    #[task(local=[bus_driver])]
    async fn active_main(ctx: active_main::Context) {
        let bus_driver = ctx.local.bus_driver;

        loop {
            bus_driver.write_cycle_looped(0x7F, 0x00).await;
            Systick::delay(750.millis()).await;

            bus_driver.write_cycle_looped(0xFF, 0x7F).await;
            Systick::delay(750.millis()).await;

            bus_driver.write_cycle_looped(0x7F, 0xFF).await;
            Systick::delay(750.millis()).await;
        }
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    println!("DEFMT PANIC!!!");
    cortex_m::asm::udf()
}
