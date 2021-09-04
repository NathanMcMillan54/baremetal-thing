#![no_std]
#![no_main]

#[macro_use] extern crate novusk;
use novusk::arm::libbmu::{app_main, Time};
use novusk::kernel::printk::printk;
use novusk::novusk::led::blink;

fn main() -> ! {
    let mut time = Time::new();
    time.sleepc(1500);

    printk!("\nHello, world!");

    printk!("Blinking and looping for ever...");
    loop {
        blink(999999);
    }

    panic!("App shouldn't have ended");
}

app_main!(main);
