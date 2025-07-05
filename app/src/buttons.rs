use defmt::info;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;

use cortex_m::interrupt::{free, Mutex};
use microbit::{
    board::Buttons, hal::gpiote::Gpiote, pac::{self, interrupt}
};

use crate::timer;

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

pub fn init_buttons(board_gpiote: pac::GPIOTE, board_buttons: Buttons) {
    let gpiote = Gpiote::new(board_gpiote);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board_buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board_buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    free(move |cs| {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);

        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
    });
}

#[interrupt]
fn GPIOTE() {
    free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let button_a_pressed = gpiote.channel0().is_event_triggered();
            let button_b_pressed = gpiote.channel1().is_event_triggered();

            match (button_a_pressed, button_b_pressed) {
                (false, false) => {},
                (true, false) => {
                    if timer::is_running() {
                        timer::stop_stopwatch();
                        info!("Stopwatch stopped!");
                    } else {
                        timer::start_stopwatch();
                        info!("Stopwatch started...");
                    }
                },
                (false, true) => {
                    if timer::is_running() {
                        timer::stop_stopwatch();
                    }
                    timer::reset_stopwatch();
                    info!("Stopwatch reset.");
                    timer::start_stopwatch();
                    info!("Starting stopwatch again...");
                },
                (true, true) => {},
            };

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    })
}