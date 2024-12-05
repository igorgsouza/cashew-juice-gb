#![feature(thread_spawn_unchecked)]
use peanut_gb::{PeanutGb, JOYPAD_SELECT, JOYPAD_START};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::sync::mpsc::channel;
use std::thread;
use svc::fs::fatfs::Fatfs;
use svc::hal;
use svc::hal::delay::FreeRtos;
use svc::hal::gpio::AnyIOPin;
use svc::hal::sd::spi::SdSpiHostDriver;
use svc::hal::sd::{SdCardConfiguration, SdCardDriver};
use svc::hal::spi::{config::DriverConfig, Dma, SpiDriver};
use svc::io::vfs::MountedFatfs;
use svc::sys;

static mut ROM: &mut [u8] = &mut [];
static mut CART_RAM: &mut [u8] = &mut [];
static mut FRAME_BUFFER: &mut [u8; 160 * 144] = &mut [0; 160 * 144];

mod drivers;
mod peanut_gb;
fn main() -> () {
    sys::link_patches();
    svc::log::EspLogger::initialize_default();
    let peripherals: hal::peripherals::Peripherals = hal::peripherals::Peripherals::take().unwrap();

    let mut controller = drivers::SNESController::new(
        peripherals.pins.gpio15,
        peripherals.pins.gpio7,
        peripherals.pins.gpio6,
    );

    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio12,
        peripherals.pins.gpio11,
        Some(peripherals.pins.gpio13),
        &DriverConfig::default().dma(Dma::Auto(4096)),
    )
    .unwrap();

    let mut cfg = SdCardConfiguration::new();
    cfg.speed_khz = 16 * 1024 as u32;

    let sd_card_driver = SdCardDriver::new_spi(
        SdSpiHostDriver::new(
            &spi_driver,
            Some(peripherals.pins.gpio2),
            AnyIOPin::none(),
            AnyIOPin::none(),
            AnyIOPin::none(),
            None,
        )
        .unwrap(),
        &cfg,
    )
    .unwrap();

    let _mounted_fatfs =
        MountedFatfs::mount(Fatfs::new_sdcard(0, sd_card_driver).unwrap(), "/sdcard", 4).unwrap();

    let mut rom_file = OpenOptions::new()
        .read(true)
        .open("/sdcard/rom.gbc")
        .unwrap();
    let mut rom_bytes: Vec<u8> = vec![];
    rom_file.read_to_end(&mut rom_bytes);
    unsafe {
        ROM = Box::leak(rom_bytes.into_boxed_slice());
    }
    drop(rom_file);

    match OpenOptions::new().read(true).open("/sdcard/sram.sav") {
        Ok(mut sram_file) => {
            let mut sram_bytes: Vec<u8> = vec![];
            sram_file.read_to_end(&mut sram_bytes).unwrap();
            unsafe {
                CART_RAM = Box::leak(sram_bytes.into_boxed_slice());
            };
        }
        Err(_) => {}
    }

    let display = drivers::Display::new(
        &spi_driver,
        drivers::DisplayPins::new(
            peripherals.pins.gpio1,
            peripherals.pins.gpio4,
            peripherals.pins.gpio5,
        ),
    );

    // let (clock_channel_sender, clock_channel_receiver) = channel();
    // let _clock_thread = thread::Builder::new()
    //     .spawn(move || loop {
    //         clock_channel_sender.send(Some(())).unwrap();
    //         FreeRtos::delay_ms(17);
    //     })
    //     .unwrap();

    let mut gb = PeanutGb::new(display);

    // println!("{}", gb.get_rom_name());

    loop {
        // let st = unsafe { esp_timer_get_time() as f64 } / 1_000_000.0;
        // match clock_channel_receiver.recv() {
        //     Ok(_) => {
        let input = controller.read_gb();
        if input & (JOYPAD_START | JOYPAD_SELECT) == 0 {
            break;
        }
        gb.frame(input);
        //     }
        //     Err(_) => {
        //         break;
        //     }
        // }
        // let et = unsafe { esp_timer_get_time() as f64 } / 1_000_000.0;
        // let dt = et - st;
        // thread::Builder::new().spawn(|| println!("?")).unwrap();
        // println!("FPS: {:?}", fps_buffer as f64 / dt)
        // FreeRtos::delay_ms(20);
    }
    let mut sram_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("/sdcard/sram.sav")
        .unwrap();

    unsafe {
        sram_file.write_all(&CART_RAM);
    }
}
