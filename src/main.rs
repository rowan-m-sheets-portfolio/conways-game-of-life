#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::InputPin};
#[rustfmt::skip]
use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::{pcg64::Pcg64, Rng};

mod life;
use life::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut hw_rng = HwRng::new(board.RNG);
    let mut rng = Pcg64::new_seed(hw_rng.random_u64() as u128);

    let mut timer0 = Timer::new(board.TIMER0);
    let mut timer1 = Timer::new(board.TIMER1);

    let mut display = Display::new(board.display_pins);
    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;

    let mut state: [[u8; 5]; 5] = [[0; 5]; 5];
    randomize_state(&mut state, &mut rng);
    loop {
        if button_a.is_low().unwrap() {
            while button_a.is_low().unwrap() {
                randomize_state(&mut state, &mut rng);
                display.show(&mut timer0, state, 10);
            }
        } else if button_b.is_low().unwrap() && timer1.read() == 0 {
            invert(&mut state);
            timer1.start(500_000);
        }
        if !done(&state) {
            life(&mut state);
            display.show(&mut timer0, state, 100);
        } else {
            timer0.delay_ms(50);
            randomize_state(&mut state, &mut rng);
        }
    }
}

fn randomize_state(state: &mut [[u8; 5]; 5], rng: &mut Pcg64) {
    for row in state {
        for index in row {
            *index = rng.generate_range(0_u8..=1);
        }
    }
}

fn invert(state: &mut [[u8; 5]; 5]) {
    for row in state {
        for index in row {
            *index = if *index == 0 {
                1
            } else {
                0
            }
        }
    }
}
