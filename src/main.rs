#![no_std]
#![no_main]

#[macro_use] extern crate novusk;
use novusk::arm::libbmu::app_main;
use novusk::kernel::printk::printk;

fn main() -> ! {
    printk!("\nHello, world!");

    loop {  }
}

app_main!(main);
