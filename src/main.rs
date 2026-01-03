#![no_std]
#![no_main]

// use defmt_rtt as _; // Disabled
use panic_probe as _;

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::bind_interrupts;
use cortex_m_rt as _; // Ensure cortex-m-rt is linked

bind_interrupts!(struct Irqs {
    // No specific interrupts needed for simple blinky
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    // Use Internal RC Oscillator for LFCLK to avoid hanging if no external crystal is present
    config.lfclk_source = embassy_nrf::config::LfclkSource::InternalRC;
    
    let p = embassy_nrf::init(config);
    
    // P0.26 is Red LED on Seeed XIAO nRF52840 (Active Low)
    let mut led = Output::new(p.P0_26, Level::High, OutputDrive::Standard);

    loop {
        // defmt::info!("Blink ON");
        led.set_low();
        Timer::after(Duration::from_millis(200)).await;
        
        // defmt::info!("Blink OFF");
        led.set_high();
        Timer::after(Duration::from_millis(800)).await;
    }
}