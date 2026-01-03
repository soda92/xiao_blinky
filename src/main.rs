#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_nrf::interrupt::Priority;
use nrf_softdevice::{
    ble::{gatt_server, peripheral},
    Softdevice,
};
use defmt_rtt as _;
use panic_probe as _;
use heapless::Vec;
use defmt::unwrap;
use embassy_futures::select::{select, Either};

#[nrf_softdevice::gatt_service(uuid = "6E400001-B5A3-F393-E0A9-E50E24DCCA9E")]
struct Nus {
    #[characteristic(uuid = "6E400002-B5A3-F393-E0A9-E50E24DCCA9E", write)]
    rx: Vec<u8, 20>,

    #[characteristic(uuid = "6E400003-B5A3-F393-E0A9-E50E24DCCA9E", notify)]
    tx: Vec<u8, 20>,
}

#[nrf_softdevice::gatt_server]
struct Server {
    nus: Nus,
}

#[embassy_executor::task]
async fn softdevice_task(sd: &'static Softdevice) -> ! {
    sd.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.gpiote_interrupt_priority = Priority::P2;
    config.time_interrupt_priority = Priority::P2;
    let p = embassy_nrf::init(config);

    // --- SoftDevice Config ---
    let config = nrf_softdevice::Config {
        clock: Some(nrf_softdevice::raw::nrf_clock_lf_cfg_t {
            source: nrf_softdevice::raw::NRF_CLOCK_LF_SRC_RC as u8,
            rc_ctiv: 16,
            rc_temp_ctiv: 2,
            accuracy: nrf_softdevice::raw::NRF_CLOCK_LF_ACCURACY_500_PPM as u8,
        }),
        conn_gap: Some(nrf_softdevice::raw::ble_gap_conn_cfg_t {
            conn_count: 6,
            event_length: 6,
        }),
        conn_gatt: Some(nrf_softdevice::raw::ble_gatt_conn_cfg_t { att_mtu: 256 }),
        gatts_attr_tab_size: Some(nrf_softdevice::raw::ble_gatts_cfg_attr_tab_size_t {
            attr_tab_size: nrf_softdevice::raw::BLE_GATTS_ATTR_TAB_SIZE_DEFAULT,
        }),
        gap_role_count: Some(nrf_softdevice::raw::ble_gap_cfg_role_count_t {
            adv_set_count: 1,
            periph_role_count: 3,
            central_role_count: 3,
            central_sec_count: 0,
            _bitfield_1: nrf_softdevice::raw::ble_gap_cfg_role_count_t::new_bitfield_1(0),
        }),
        gap_device_name: Some(nrf_softdevice::raw::ble_gap_cfg_device_name_t {
            p_value: b"XiaoBLE" as *const u8 as _,
            current_len: 7,
            max_len: 7,
            write_perm: unsafe { core::mem::zeroed() },
            _bitfield_1: nrf_softdevice::raw::ble_gap_cfg_device_name_t::new_bitfield_1(nrf_softdevice::raw::BLE_GATTS_VLOC_STACK as u8),
        }),
        ..Default::default()
    };

    let sd = Softdevice::enable(&config);
    let server = unwrap!(Server::new(sd));

    spawner.spawn(softdevice_task(sd).unwrap());

    let mut red = Output::new(p.P0_26, Level::High, OutputDrive::Standard);
    let mut green = Output::new(p.P0_30, Level::High, OutputDrive::Standard);
    let mut blue = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    // Advertisement Data
    // Flags (General Discovery, LE Only) + Complete Local Name
    #[rustfmt::skip]
    static ADV_DATA: [u8; 12] = [
        0x02, 0x01, 0x06, // Flags: 0x06 = General Disc | LE Only
        0x08, 0x09, b'X', b'i', b'a', b'o', b'B', b'L', b'E', // Name: XiaoBLE
    ];
    static SCAN_DATA: [u8; 0] = [];

    loop {
        let config = peripheral::Config::default();
        let adv = peripheral::ConnectableAdvertisement::ScannableUndirected {
            adv_data: &ADV_DATA,
            scan_data: &SCAN_DATA,
        };

        defmt::info!("Advertising...");
        let conn = unwrap!(peripheral::advertise_connectable(sd, adv, &config).await);
        defmt::info!("Connected!");

        let gatt_fut = gatt_server::run(&conn, &server, |e| match e {
            ServerEvent::Nus(NusEvent::RxWrite(val)) => {
                defmt::info!("Received over BLE: {:?}", val.as_slice());
            }
            ServerEvent::Nus(NusEvent::TxCccdWrite { notifications }) => {
                defmt::info!("Notifications: {}", notifications);
            }
        });

        let blink_fut = async {
            loop {
                // Blink Red
                red.set_low();
                if let Err(_) = server.nus.tx_notify(&conn, &Vec::from_slice(b"Red LED On").unwrap()) {
                   // Ignore error
                }
                Timer::after(Duration::from_millis(500)).await;
                red.set_high();

                // Blink Green
                green.set_low();
                 if let Err(_) = server.nus.tx_notify(&conn, &Vec::from_slice(b"Green LED On").unwrap()) {
                   // Ignore
                }
                Timer::after(Duration::from_millis(500)).await;
                green.set_high();

                // Blink Blue
                blue.set_low();
                 if let Err(_) = server.nus.tx_notify(&conn, &Vec::from_slice(b"Blue LED On").unwrap()) {
                   // Ignore
                }
                Timer::after(Duration::from_millis(500)).await;
                blue.set_high();
                
                Timer::after(Duration::from_millis(500)).await;
            }
        };

        match select(gatt_fut, blink_fut).await {
            Either::First(e) => {
                defmt::info!("Disconnected: {:?}", e);
            }
            Either::Second(_) => {
                // Should not happen
            }
        }
    }
}

// defmt::timestamp!("{=u64:us}", embassy_time::Instant::now().as_micros());

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}