#![no_std]
#![no_main]

#[macro_use] extern crate novusk;
use novusk::arm::libbmu::app_main;
use novusk::kernel::printk::printk;
use core::panic::Location;

fn main() -> ! {
    printk!("Hello, world!");
    printk!("From: {}", Location::file());

    loop {  }
}

app_main!(main);
