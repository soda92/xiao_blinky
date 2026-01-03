#![no_std]
#![no_main]

use defmt_rtt as _;
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
    
    // LEDs are Active Low on Seeed Xiao nRF52840
    let mut red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    let mut green = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    defmt::info!("Starting Colorful Blinky!");

    loop {
        defmt::info!("Red");
        red.set_low();
        Timer::after(Duration::from_millis(300)).await;
        red.set_high();
        
        defmt::info!("Green");
        green.set_low();
        Timer::after(Duration::from_millis(300)).await;
        green.set_high();

        defmt::info!("Blue");
        blue.set_low();
        Timer::after(Duration::from_millis(300)).await;
        blue.set_high();
        
        // Brief pause with all off
        Timer::after(Duration::from_millis(300)).await;
    }
}

defmt::timestamp!("{=u64:us}", embassy_time::Instant::now().as_micros());

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
