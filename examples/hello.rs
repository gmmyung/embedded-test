//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

use core::cell::RefCell;

use cortex_m::asm::delay;
use cortex_m::interrupt::Mutex;
use panic_halt as _;

use cortex_m_rt::entry;
// use cortex_m_semihosting::{debug, hprintln};
use cortex_m_semihosting::hprintln;
use embedded_hal::PwmPin;
use stm32f3xx_hal::pac::{Peripherals, self};
use stm32f3xx_hal::{prelude::*, timer};
use stm32f3xx_hal::pwm::tim1;

static TIMER: Mutex<RefCell<Option<timer::Timer<pac::TIM2>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    let dp = Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(16.MHz()).freeze(&mut flash.acr);

    let (ch1_no_pins, _, _, _) = tim1(dp.TIM1, 9000, 50.Hz(), &clocks);

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let pe9 = gpioe
        .pe9
        // .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        .into_af_push_pull(&mut gpioe.moder, &mut gpioe.otyper, &mut gpioe.afrh);

    let mut ch1 = ch1_no_pins.output_to_pe9(pe9);

    ch1.enable();
    

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {
        ch1.set_duty(ch1.get_max_duty() / 2);
        delay(3000000);
        ch1.set_duty(ch1.get_max_duty() / 10);
        delay(3000000);
    }
}

