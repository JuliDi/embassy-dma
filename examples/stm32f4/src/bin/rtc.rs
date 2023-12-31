#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use chrono::{NaiveDate, NaiveDateTime};
use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::rtc::{Rtc, RtcConfig};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let now = NaiveDate::from_ymd_opt(2020, 5, 15)
        .unwrap()
        .and_hms_opt(10, 30, 15)
        .unwrap();

    let mut rtc = Rtc::new(p.RTC, RtcConfig::default());

    rtc.set_datetime(now.into()).expect("datetime not set");

    // In reality the delay would be much longer
    Timer::after(Duration::from_millis(20000)).await;

    let _then: NaiveDateTime = rtc.now().unwrap().into();
}
