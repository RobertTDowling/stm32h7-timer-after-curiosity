#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Instant, Timer};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut uart = UartTx::new(p.USART3, p.PD8, p.DMA1_CH1, Config::default()).unwrap();
    let mut msg: String<128> = String::new();
    msg.clear();
    core::fmt::write(&mut msg, format_args!("#Boot\n")).unwrap();
    _ = uart.write(msg.as_bytes()).await;

    const MAX_DATA: usize = 1000;
    let mut data: [u32; MAX_DATA] = [0; MAX_DATA];
    for ticks in 0..=240 {
        let mut t0 = Instant::now();
        for d in data.iter_mut() {
            Timer::after_ticks(ticks).await;
            let t1 = Instant::now();
            *d = (t1 - t0).as_micros() as u32;
            t0 = t1;
        }
        for d in data.iter() {
            msg.clear();
            core::fmt::write(&mut msg, format_args!("{} {}\n", ticks, d)).unwrap();
            _ = uart.write(msg.as_bytes()).await;
        }
    }
    _ = uart.write("#Done\n".as_bytes()).await;
    loop {
        Timer::after_ticks(1000).await;
    }
}
