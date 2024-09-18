use drivers::DisplayMessage;
use peanut_gb::{Context, PeanutGb};
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use svc::hal;
use svc::hal::delay::FreeRtos;
use svc::hal::gpio::Gpio4;
use svc::hal::gpio::Gpio5;
use svc::hal::spi::{config::DriverConfig, Dma, SpiDriver};
use svc::sys::{self, esp_timer_get_time};

mod drivers;
mod peanut_gb;

fn main() -> () {
    sys::link_patches();
    svc::log::EspLogger::initialize_default();
    let peripherals: hal::peripherals::Peripherals = hal::peripherals::Peripherals::take().unwrap();

    let (display_channel_sender, display_channel_receiver) = channel::<DisplayMessage>();

    let (clock_channel_sender, clock_channel_receiver) = channel::<Option<()>>();

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
    let mut display = drivers::Display::new(
        spi_driver,
        drivers::DisplayPins::new(
            peripherals.pins.gpio1,
            peripherals.pins.gpio4,
            peripherals.pins.gpio5,
        ),
    );
    let _display_thread = thread::Builder::new()
        .stack_size(32 * 1024)
        .spawn(move || loop {
            match display_channel_receiver.recv() {
                Ok(DisplayMessage::Buffer((pixels, line))) => {
                    display.buffer_line_gbc(pixels, line);
                }
                Ok(DisplayMessage::Draw(palette)) => {
                    display.draw(palette);
                }
                _ => {
                    break;
                }
            }
        })
        .unwrap();
    let _clock_thread = thread::Builder::new()
        .stack_size(32 * 1024)
        .spawn(move || loop {
            match clock_channel_sender.send(Some(())) {
                Ok(_) => {}
                Err(_) => {
                    break;
                }
            }
            FreeRtos::delay_ms(16);
        })
        .unwrap();

    let rom = include_bytes!("../rom.gbc").to_vec();
    let ram = vec![0; 0x40];

    let mut gb = PeanutGb::new(Context {
        rom,
        ram,
        display_channel_sender,
    });

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
    }
}
