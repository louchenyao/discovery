#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{self, Write};
use heapless::{consts, Vec, String};

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}

struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            while self.usart1.isr.read().txe().bit_is_clear() {}
            self.usart1.tdr.write(|w| w.tdr().bits(u16::from(c)));
        }
        Ok(())
    }
}

impl SerialPort {
    fn recv_byte(&mut self) -> u8 {
        while self.usart1.isr.read().rxne().bit_is_clear() {}
        self.usart1.rdr.read().rdr().bits() as u8
    }
}

#[entry]
fn main() -> ! {
    let (usart1, /* mono_timer, */ mut itm) = aux11::init();
    let mut serial = SerialPort { usart1 };
    let mut buffer: Vec<u8, consts::U32> = Vec::new();

    loop {
        buffer.clear();
        loop {
            let b = serial.recv_byte() as char;
            iprintln!(&mut itm.stim[0], "received {}", b);
            match b {
                '\n' => {
                    let s = String::from_utf8(buffer).unwrap();
                    uprintln!(serial, "{}", s.as_str());
                    buffer = s.into_bytes();
                    break;
                }
                b => {
                    match buffer.push(b as u8) {
                        Ok(_) => (),
                        Err(e) => {
                            uprintln!(serial, "cannot push {}", e as char);
                            break;
                        }
                    }
                }
            }

        }
    }
}
