use defmt_rtt as _;
use panic_halt as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;

use microbit::{
    display::nonblocking::{Display, Frame, MicrobitFrame}, 
    gpio::DisplayPins,
    hal::{
        clocks::Clocks,
        rtc::{Rtc, RtcInterrupt},
    },
    pac::{self, interrupt, RTC0, TIMER0},
};

use microbit_text::scrolling::Animate;
use microbit_text::scrolling_text::ScrollingStaticText;

use crate::timer;

static DISPLAY: Mutex<RefCell<Option<Display<TIMER0>>>> = Mutex::new(RefCell::new(None));
static ANIM_TIMER: Mutex<RefCell<Option<Rtc<RTC0>>>> = Mutex::new(RefCell::new(None));
static SCROLLER: Mutex<RefCell<Option<ScrollingStaticText>>> = Mutex::new(RefCell::new(None));

static mut SECONDS_BUFFER: [u8; 2] = [b'0', b'0'];

pub fn init_display(board_clock: pac::CLOCK, board_rtc: pac::RTC0, board_timer: TIMER0, board_display_pins: DisplayPins, mut board_nvic: pac::NVIC) {
    Clocks::new(board_clock).start_lfclk();

    let mut rtc0 = Rtc::new(board_rtc, 2047).unwrap();
    rtc0.enable_event(RtcInterrupt::Tick);
    rtc0.enable_interrupt(RtcInterrupt::Tick, None);
    rtc0.enable_counter();

    let display = Display::new(board_timer, board_display_pins);

    let scroller = ScrollingStaticText::default();

    cortex_m::interrupt::free(|cs| {
        *DISPLAY.borrow(cs).borrow_mut() = Some(display);
        *ANIM_TIMER.borrow(cs).borrow_mut() = Some(rtc0);
        *SCROLLER.borrow(cs).borrow_mut() = Some(scroller);
    });
    unsafe {
        board_nvic.set_priority(pac::Interrupt::RTC0, 64);
        board_nvic.set_priority(pac::Interrupt::TIMER0, 128);
        pac::NVIC::unmask(pac::Interrupt::RTC0);
        pac::NVIC::unmask(pac::Interrupt::TIMER0);
    }
}

#[interrupt]
fn TIMER0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
            display.handle_display_event();
        }
    });
}

#[interrupt]
unsafe fn RTC0() {
    cortex_m::interrupt::free(|cs| {
        if let Some(rtc) = ANIM_TIMER.borrow(cs).borrow_mut().as_mut() {
            rtc.reset_event(RtcInterrupt::Tick);
            if let Some(scroller) = SCROLLER.borrow(cs).borrow_mut().as_mut() {
                if !scroller.is_finished() {
                    scroller.tick();
                    let mut frame = MicrobitFrame::default();
                    frame.set(scroller);
                    if let Some(display) = DISPLAY.borrow(cs).borrow_mut().as_mut() {
                        display.show_frame(&frame);
                    }
                } else {
                    let seconds = timer::get_seconds();
                    let tens = (seconds / 10) + b'0';
                    let ones = (seconds % 10) + b'0';
                    SECONDS_BUFFER[0] = tens;
                    SECONDS_BUFFER[1] = ones;
                    scroller.set_message(&SECONDS_BUFFER);
                }
            }
        }
    });
}