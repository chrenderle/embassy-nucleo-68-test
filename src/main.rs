#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]


use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::{gpio::{Level, Output, Speed}, Config, rcc::ClockSrc, time::Hertz};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config: Config = Default::default();
    config.rcc.mux = ClockSrc::HSE(Hertz::mhz(32));
    let p = embassy_stm32::init(config);
    info!("Hello World!");

    loop {
        Timer::after(Duration::from_secs(1)).await;
        info!("second passed");
    }
}
