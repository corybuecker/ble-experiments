#![no_std]
#![no_main]
#![macro_use]

use defmt_rtt as _;
use embassy_nrf as _;
use panic_probe as _;

use defmt::info;
use embassy_executor::Spawner;
use embassy_nrf::config;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let _peripherals = embassy_nrf::init(config::Config::default());
    info!("Hello, world!");

    #[allow(clippy::empty_loop)]
    loop {}
}
