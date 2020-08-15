#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let tick = 50_u16;


    let mut start = true;
    let mut dir = 0;

    loop {
        if start {
            leds[dir].on();
        } else {
            leds[(dir + 8 - 1) % 8].off();
            dir = (dir + 1) % 8;
        }
        start = !start;
        delay.delay_ms(tick);
    }
}