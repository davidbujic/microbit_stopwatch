#![no_std]
#![no_main]

use cortex_m as _;
use cortex_m_rt::entry;
use panic_halt as _;
use defmt::info;
use defmt_rtt as _;
use microbit as _;

#[entry]
fn main() -> ! {
    info!("Hello world!");

    loop {
        // your code goes here
    }
}
