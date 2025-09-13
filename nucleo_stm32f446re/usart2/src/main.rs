#![no_std]
#![no_main]

use cortex_m::delay::Delay;
use panic_halt as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{self as hal, time::Hertz};
use crate::hal::{pac, prelude::*};

use core::fmt::Write; 

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let core = cortex_m::Peripherals::take().unwrap();

    // Get access to the device's peripherals
    // Configure the clock
    let rcc = dp.RCC.constrain();
    let mut clocks = rcc.cfgr.sysclk(Hertz::MHz(180)).freeze();

    // Set up delay abstraction based on SysTick
    let mut delay = Delay::new(core.SYST, clocks.sysclk().to_Hz());
    let gpioa = dp.GPIOA.split();

    // define RX/TX pins
    let tx_pin = gpioa.pa2;

    let mut tx = dp.USART2.tx(tx_pin, 9600.bps(), &mut clocks).unwrap();

    let mut value: u8 = 0;

    loop {
        writeln!(tx, "value: {value:02}\r").unwrap();
        value = value.wrapping_add(1);
        delay.delay_ms(2000_u32);
    }

}
