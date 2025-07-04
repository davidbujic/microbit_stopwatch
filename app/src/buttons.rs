use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;

use cortex_m::interrupt::Mutex;
use microbit::{hal::gpiote::Gpiote, pac::{self, interrupt}, Board};

static GPIO: Mutex<RefCell<Option<Gpiote>>> = Mutex::new(RefCell::new(None));

pub fn init_buttons() {
    let board = Board::take().unwrap();

    let gpiote = Gpiote::new(board.GPIOTE);

    let channel0 = gpiote.channel0();
    channel0
        .input_pin(&board.buttons.button_a.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel0.reset_events();

    let channel1 = gpiote.channel1();
    channel1
        .input_pin(&board.buttons.button_b.degrade())
        .hi_to_lo()
        .enable_interrupt();
    channel1.reset_events();

    cortex_m::interrupt::free(move |cs| {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::GPIOTE);
        }
        pac::NVIC::unpend(pac::Interrupt::GPIOTE);

        *GPIO.borrow(cs).borrow_mut() = Some(gpiote);
    });
}

#[interrupt]
fn GPIOTE() {
    cortex_m::interrupt::free(|cs| {
        if let Some(gpiote) = GPIO.borrow(cs).borrow().as_ref() {
            let button_a_pressed = gpiote.channel0().is_event_triggered();
            let button_b_pressed = gpiote.channel1().is_event_triggered();

            defmt::info!(
                "Button pressed {:?}",
                match (button_a_pressed, button_b_pressed) {
                    (false, false) => "",
                    (true, false) => "A",
                    (false, true) => "B",
                    (true, true) => "A + B",
                }
            );

            gpiote.channel0().reset_events();
            gpiote.channel1().reset_events();
        }
    })
}