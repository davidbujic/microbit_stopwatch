#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;
use defmt::info;
use defmt_rtt as _;
use microbit as _;

mod buttons;
mod timer;

#[entry]
fn main() -> ! {
    info!("Hello world!");

    // buttons::init_buttons();
    timer::init_timer();
    timer::start_stopwatch();

    loop {
        
    }
}
