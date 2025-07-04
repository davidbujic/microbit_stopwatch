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
    info!("Stopwatch example.");

    let board = microbit::board::Board::take().unwrap();

    timer::init_timer(board.TIMER1);
    timer::start_stopwatch();
    buttons::init_buttons(board.GPIOTE, board.buttons);

    loop {
        
    }
}
