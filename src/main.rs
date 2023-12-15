#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_executor::Spawner;
use embassy_stm32::usart::{Config, UartTx};
use embassy_time::{Instant, Timer};
use heapless::String;
use {defmt_rtt as _, panic_probe as _};

static COUNT_ATOMIC: AtomicU32 = AtomicU32::new(0);

#[embassy_executor::task]
async fn task() {
    // Count at 1kHz, but not to be precise; do it to detect thread starvation.
    // So always expect close to, but less than 1000 counts per 1 second real time
    let mut c: u32 = 0;
    loop {
        c += 1;
        Timer::after_millis(1).await;
        COUNT_ATOMIC.store(c, Ordering::Relaxed);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let mut uart = UartTx::new(p.USART3, p.PD8, p.DMA1_CH1, Config::default()).unwrap();
    let mut msg: String<128> = String::new();
    msg.clear();
    core::fmt::write(&mut msg, format_args!("#Boot\n")).unwrap();
    _ = uart.write(msg.as_bytes()).await;

    _ = spawner.spawn(task());

    const MAX_DATA: usize = 1000;
    for ticks in 60..=100 {
        let t0 = Instant::now();
        let c0 = COUNT_ATOMIC.load(Ordering::Relaxed);
        for _ in 0..MAX_DATA {
            Timer::after_ticks(ticks).await;
        }
        let t1 = Instant::now();
        let c1 = COUNT_ATOMIC.load(Ordering::Relaxed);

        // Wall clock elapsed time for 1000 sleeps of `ticks` us.
        let dt = (t1 - t0).as_millis();

        // Number of 1ms loops `task` could complete during that same interval.
        let dc = c1 - c0;

        // Since we are doing 1000 sleeps of `ticks` us, we expect at least
        // `ticks` ms to elapse in real time, but we don't wish to see a lot
        // more than that.

        // And if `task` isn't starved, its count will be close to the elapsed
        // real time in ms.

        // But if `task` is starved, its count may be much closer to `ticks`.
        msg.clear();
        core::fmt::write(&mut msg, format_args!("{} {} {}\n", ticks, dc, dt)).unwrap();
        _ = uart.write(msg.as_bytes()).await;
    }
    _ = uart.write("#Done\n".as_bytes()).await;
    loop {
        Timer::after_ticks(1000).await;
    }
}
