#![no_std]
#![no_main]

mod common;

use core::mem;
use defmt::{debug, info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::config;
use nrf_softdevice::{
    Softdevice,
    ble::{
        advertisement_builder::{
            Flag, LegacyAdvertisementBuilder, LegacyAdvertisementPayload, ServiceList,
            ServiceUuid16,
        },
        peripheral,
    },
    raw,
};

#[embassy_executor::task]
async fn softdevice_task(soft_device: &'static Softdevice) -> ! {
    soft_device.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let _peripherals = embassy_nrf::init(config::Config::default());

    let config = nrf_softdevice::Config::default();

    debug!("enabling Softdevice");
    let soft_device = Softdevice::enable(&config);

    unwrap!(spawner.spawn(softdevice_task(soft_device)));

    let mut config = peripheral::Config::default();
    config.interval = 50;

    static ADV_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .flags(&[Flag::GeneralDiscovery, Flag::LE_Only])
        .services_16(ServiceList::Complete, &[ServiceUuid16::HEALTH_THERMOMETER])
        .short_name("Hello")
        .build();

    static SCAN_DATA: LegacyAdvertisementPayload = LegacyAdvertisementBuilder::new()
        .full_name("Hello, Rust!")
        .build();

    let adv = peripheral::NonconnectableAdvertisement::ScannableUndirected {
        adv_data: &ADV_DATA,
        scan_data: &SCAN_DATA,
    };

    info!("starting advertising");
    unwrap!(peripheral::advertise(&soft_device, adv, &config).await);

    loop {}
}
