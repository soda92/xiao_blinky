#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_nrf::bind_interrupts;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::peripherals::USBD;
use embassy_nrf::interrupt;
use embassy_nrf::usb::{Driver, vbus_detect::HardwareVbusDetect};
use embassy_time::{Duration, Timer};
use log::info;
use panic_probe as _;

// Bind USB interrupts
bind_interrupts!(struct Irqs {
    USBD => embassy_nrf::usb::InterruptHandler<USBD>;
    CLOCK_POWER => embassy_nrf::usb::vbus_detect::InterruptHandler;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, HardwareVbusDetect>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.lfclk_source = embassy_nrf::config::LfclkSource::InternalRC;
    let p = embassy_nrf::init(config);

    // --- USB Setup ---
    let driver = Driver::new(p.USBD, Irqs, HardwareVbusDetect::new(Irqs));
    spawner.spawn(logger_task(driver)).unwrap();

    // --- GPIO Setup ---
    let mut red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    let mut green = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    info!("USB Serial Initialized! Starting Blinky...");

    loop {
        info!("Red LED On");
        red.set_low();
        Timer::after(Duration::from_millis(500)).await;
        red.set_high();

        info!("Green LED On");
        green.set_low();
        Timer::after(Duration::from_millis(500)).await;
        green.set_high();

        info!("Blue LED On");
        blue.set_low();
        Timer::after(Duration::from_millis(500)).await;
        blue.set_high();
        
        Timer::after(Duration::from_millis(500)).await;
    }
}