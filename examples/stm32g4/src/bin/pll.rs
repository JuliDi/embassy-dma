#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::rcc::{ClockSrc, PllM, PllN, PllR, PllSrc};
use embassy_stm32::Config;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();

    // Configure PLL to max frequency of 170 MHz
    config.rcc.mux = ClockSrc::PLLCLK(PllSrc::HSI16, PllM::Div4, PllN::Mul85, PllR::Div2);

    let _p = embassy_stm32::init(config);
    info!("Hello World!");

    loop {
        Timer::after(Duration::from_millis(1000)).await;
        info!("1s elapsed");
    }
}
