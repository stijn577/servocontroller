use stm32f4xx_hal::gpio::{Output, Pin, PushPull};

pub type Led = Pin<'C', 13, Output<PushPull>>;

pub type Clk = Pin<'B', 0, Output<PushPull>>;
pub type Set = Pin<'B', 1, Output<PushPull>>;
pub type Done = Pin<'B', 2>;

