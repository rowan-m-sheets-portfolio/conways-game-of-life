#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
#[rustfmt::skip]
use microbit::{
    board::{Board, Buttons},
    display::blocking::Display,
    hal::{
        Rng as HwRng,
        timer::Timer,
    },
};
use nanorand::{Rng, SeedableRng};

mod life;
use life::*;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let _board = Board::take().unwrap();
    let mut counter = 0u64;
    loop {
        rprintln!("{}", counter);
        counter += 1;
    }
}
