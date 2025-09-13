#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _; // Halts on panic
use stm32f4xx_hal::{
    gpio::GpioExt, rcc::RccExt, time::Hertz
    
};
#[entry]
fn main() -> ! {
    // Get access to the device's peripherals
    let dp = stm32f4xx_hal::pac::Peripherals::take().unwrap();
    // Configure the clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(Hertz::MHz(180)).freeze();
    // Set up the GPIO pin
    let gpioa = dp.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    // Blink the LED
    loop {
        led.set_low();
        cortex_m::asm::delay(clocks.sysclk().to_Hz() / 4); // Delay
        led.set_high();
        cortex_m::asm::delay(clocks.sysclk().to_Hz() / 4); // Delay
    }
}
