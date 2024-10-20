#![feature(thread_spawn_unchecked)]
use drivers::{Display, DisplayMessage};
use peanut_gb::PeanutGb;
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use svc::hal;
use svc::hal::delay::FreeRtos;
use svc::hal::gpio::Gpio4;
use svc::hal::gpio::Gpio5;
use svc::hal::spi::{config::DriverConfig, Dma, SpiDriver};
use svc::sys::{self, esp_timer_get_time};

static ROM: &[u8] = include_bytes!("../rom.gbc");
static mut RAM: &mut [u8; 0x2000] = &mut [0; 0x2000];
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
    let display = drivers::Display::new(
        spi_driver,
        drivers::DisplayPins::new(
            peripherals.pins.gpio1,
            peripherals.pins.gpio4,
            peripherals.pins.gpio5,
        ),
    );

    let (clock_channel_sender, clock_channel_receiver) = channel();
    let _clock_thread = thread::Builder::new()
        .spawn(move || loop {
            clock_channel_sender.send(Some(())).unwrap();
            FreeRtos::delay_ms(17);
        })
        .unwrap();

    let mut gb = PeanutGb::new(display);

    // println!("{}", gb.get_rom_name());

    loop {
        // let st = unsafe { esp_timer_get_time() as f64 } / 1_000_000.0;
        match clock_channel_receiver.recv() {
            Ok(_) => {
                let input = controller.read_gb();
                gb.frame(input);
            }
            Err(_) => {
                break;
            }
        }
        // let et = unsafe { esp_timer_get_time() as f64 } / 1_000_000.0;
        // let dt = et - st;
        // thread::Builder::new().spawn(|| println!("?")).unwrap();
        // println!("FPS: {:?}", fps_buffer as f64 / dt)
        // FreeRtos::delay_ms(20);
    }
}
