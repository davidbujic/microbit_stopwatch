use defmt::info;
use defmt_rtt as _;
use panic_halt as _;

use core::cell::{RefCell};
use cortex_m::interrupt::{free, Mutex};


use microbit::{
    hal::{
        pac::{interrupt, TIMER1},
        timer::{Timer}
    },
    pac::{self},
};

static TIMER: Mutex<RefCell<Option<Timer<TIMER1>>>> = Mutex::new(RefCell::new(None));
static SECONDS: Mutex<RefCell<u32>> = Mutex::new(RefCell::new(0));
static IS_RUNNING: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

pub fn init_timer(timer1: TIMER1) {
    let mut timer = Timer::new(timer1);
    timer.enable_interrupt();
    timer.start(1_000_000u32);

    free(move |cs| {
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER1);
        }
        pac::NVIC::unpend(pac::Interrupt::TIMER1);
    
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });
}

#[interrupt]
fn TIMER1() {
    free(|cs| {
        if let Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().as_mut() {
            timer.reset_event();

            let is_running = *IS_RUNNING.borrow(cs).borrow();
            if is_running {
                let mut seconds = SECONDS.borrow(cs).borrow_mut();
                *seconds += 1;
                info!("Seconds: {}", *seconds);

                if *seconds == 60 {
                    *seconds = 0;
                }
            }

            timer.start(1_000_000u32);
        }
    })
}

pub fn start_stopwatch() {
    free(|cs| {
        *IS_RUNNING.borrow(cs).borrow_mut() = true;
    })
}

pub fn stop_stopwatch() {
    free(|cs| {
        *IS_RUNNING.borrow(cs).borrow_mut() = false;
    })
}

pub fn reset_stopwatch() {
    free(|cs| {
        *SECONDS.borrow(cs).borrow_mut() = 0;
        *IS_RUNNING.borrow(cs).borrow_mut() = false;
    })
}

pub fn get_seconds() -> u32 {
    free(|cs| {
        *SECONDS.borrow(cs).borrow()
    })
}

pub fn is_running() -> bool {
    free(|cs| {
        *IS_RUNNING.borrow(cs).borrow()
    })
}