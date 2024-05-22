// use stm32_hal2::clocks::{ApbPrescaler, Clocks, HclkPrescaler, InputSrc, PllSrc, Pllp, Pllq};

use rtic_monotonics::systick::fugit::RateExtU32;
use stm32f4xx_hal::rcc;

#[global_allocator]
static HEAP_ALLOCATOR: alloc_cortex_m::CortexMHeap = alloc_cortex_m::CortexMHeap::empty();

#[allow(unsafe_code)]
pub fn heap_setup() {
    let start = cortex_m_rt::heap_start() as usize;
    let size = 64_000;
    unsafe { HEAP_ALLOCATOR.init(start, size) }
}

// clock config for stm32-hal2
// pub const CLOCKS: Clocks = Clocks {
//     input_src: InputSrc::Pll(PllSrc::Hse(25_000_000)),
//     pllm: 25,
//     plln: 168,
//     pllp: Pllp::Div2,
//     pllq: Pllq::Div4, // unused...? For 48 MHz clocks
//     hclk_prescaler: HclkPrescaler::Div1,
//     apb1_prescaler: ApbPrescaler::Div2,
//     apb2_prescaler: ApbPrescaler::Div1,
//     hse_bypass: false, // defaults to PH0, PH1
//     security_system: false,
// };

// pub fn clock_config() {
//     let hclk = CLOCKS.hclk();

//     assert!(hclk == 84_000_000); // double check with DwtMono type

//     defmt::println!("Setting up mono with hclk = {}", hclk);

//     match CLOCKS.setup() {
//         Ok(_) => defmt::info!("Speed ok!"),
//         Err(_) => defmt::error!("Incorrect speed!"),
//     }
// }

// clock config for stm32_f4xx_hal
pub fn clock_config(rcc_val: rcc::Rcc) -> rcc::Clocks {
    // setup system clock so we can use delays
    rcc_val
        .cfgr
        .use_hse(25.MHz())
        .sysclk(84.MHz())
        .pclk1(25.MHz())
        .pclk2(25.MHz())
        .require_pll48clk()
        .freeze()
}
