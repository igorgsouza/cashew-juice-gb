use cashew_gb::{Gb, GbError, GbInitError};
use std::cell::RefCell;
use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::thread;
use svc::hal;
use svc::hal::delay::FreeRtos;
use svc::hal::gpio::Gpio4;
use svc::hal::gpio::Gpio5;
use svc::hal::spi::{config::DriverConfig, Dma, SpiDriver};
use svc::sys::{self, esp_timer_get_time};

mod cashew_gb;
mod drivers;

const KB: usize = 1024;

struct Context {
    rom: Box<Vec<u8>>,
    ram: RefCell<Vec<u8>>,
    display_channel_sender: Sender<Option<([u8; 160], u8, [u16; 0x40])>>,
}

fn main() -> () {
    sys::link_patches();
    let peripherals: hal::peripherals::Peripherals = hal::peripherals::Peripherals::take().unwrap();

    println!(
        "starting - HEAP: {}B, STACK: {}B",
        unsafe { sys::esp_get_free_heap_size() },
        unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
    );

    let (display_channel_sender, display_channel_receiver) = channel();

    let spi_driver = SpiDriver::new(
        peripherals.spi2,
        peripherals.pins.gpio12,
        peripherals.pins.gpio11,
        Some(peripherals.pins.gpio13),
        &DriverConfig::default().dma(Dma::Auto(4096)),
    )
    .unwrap();
    println!(
        "OK - spi_driver - HEAP: {}B, STACK: {}B",
        unsafe { sys::esp_get_free_heap_size() },
        unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
    );

    thread::scope(|main_scope| {
        println!(
            "OK - main_scope - HEAP: {}B, STACK: {}B",
            unsafe { sys::esp_get_free_heap_size() },
            unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        );

        let rom = Box::new(Vec::from(include_bytes!("../rom.gbc")));

        println!(
            "OK - rom - HEAP: {}B, STACK: {}B",
            unsafe { sys::esp_get_free_heap_size() },
            unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        );

        // let mut cfg = SdCardConfiguration::new();
        // cfg.speed_khz = 16 * KB as u32;

        // let sd_card_driver = SdCardDriver::new_spi(
        //     SdSpiHostDriver::new(
        //         &spi_driver,
        //         Some(peripherals.pins.gpio2),
        //         AnyIOPin::none(),
        //         AnyIOPin::none(),
        //         AnyIOPin::none(),
        //         None,
        //     )
        //     .unwrap(),
        //     &cfg,
        // )
        // .unwrap();
        // println!(
        //     "OK - sd_card_driver - HEAP: {}B, STACK: {}B",
        //     unsafe { sys::esp_get_free_heap_size() },
        //     unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        // );

        // let _mounted_fatfs =
        //     MountedFatfs::mount(Fatfs::new_sdcard(0, sd_card_driver).unwrap(), "/sdcard", 4)
        //         .unwrap();
        // println!(
        //     "OK - _mounted_fatfs - HEAP: {}B, STACK: {}B",
        //     unsafe { sys::esp_get_free_heap_size() },
        //     unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        // );

        // let mut rom_file = OpenOptions::new()
        //     .read(true)
        //     .open("/sdcard/rom.gbc")
        //     .unwrap();
        // println!(
        //     "OK - rom_file - HEAP: {}B, STACK: {}B",
        //     unsafe { sys::esp_get_free_heap_size() },
        //     unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        // );

        // let mut rom = Vec::new();
        // rom_file.read_to_end(&mut rom).unwrap();
        // println!(
        //     "OK - rom - HEAP: {}B, STACK: {}B",
        //     unsafe { sys::esp_get_free_heap_size() },
        //     unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        // );

        let display = drivers::Display::new(
            &spi_driver,
            drivers::DisplayPins::new(
                peripherals.pins.gpio1,
                peripherals.pins.gpio4,
                peripherals.pins.gpio5,
            ),
        );
        thread::Builder::new()
            .stack_size(64 * KB)
            .spawn_scoped(main_scope, move || {
                display_channel_listener(display_channel_receiver, display)
            })
            .unwrap();
        println!(
            "OK - display - HEAP: {}B, STACK: {}B",
            unsafe { sys::esp_get_free_heap_size() },
            unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        );

        let context = Context {
            rom,
            ram: RefCell::new(vec![]),
            display_channel_sender,
        };

        let mut controller = drivers::SNESController::new(
            peripherals.pins.gpio15,
            peripherals.pins.gpio7,
            peripherals.pins.gpio6,
        );
        println!(
            "OK - controller - HEAP: {}B, STACK: {}B",
            unsafe { sys::esp_get_free_heap_size() },
            unsafe { svc::sys::uxTaskGetStackHighWaterMark(core::ptr::null_mut()) }
        );

        let mut gb =
            match cashew_gb::Gb::new(&context, rom_read, ram_read, ram_write, Some(gb_error)) {
                GbInitError::GbInitNoError(mut gb) => {
                    context
                        .ram
                        .borrow_mut()
                        .append(&mut vec![0; gb.get_save_size()]);
                    gb.gb_init_lcd(draw_line);
                    gb
                }
                _ => {
                    panic!("Failed to create Gameboy instance")
                }
            };
        loop {
            let ts = unsafe { esp_timer_get_time() } / 1_000_000;
            for _ in 0..60 {
                let input = controller.read_gb();
                gb.joypad = !input;
                gb.run_frame();
                gb.get_context().display_channel_sender.send(None).unwrap();
            }
            let ts = unsafe { esp_timer_get_time() } / 1_000_000 - ts;
            println!("FPS:{}", 60.0 / (ts as f64))
        }
    });

    println!("DONE!");
}

fn rom_read(gb: &Gb<Context>, addr: usize) -> u8 {
    gb.get_context().rom[addr]
}

fn ram_read(gb: &Gb<Context>, addr: usize) -> u8 {
    return gb.get_context().ram.borrow()[addr];
}

fn ram_write(gb: &Gb<Context>, addr: usize, val: u8) -> () {
    gb.get_context().ram.borrow_mut()[addr] = val;
}

fn gb_error(_gb: &Gb<Context>, gb_error: GbError, addr: u16) -> () {
    let error = match gb_error {
        GbError::GbHaltForever => "GbHaltForever",
        GbError::GbInvalidMax => "GbInvalidMax",
        GbError::GbInvalidOpcode => "GbInvalidOpcode",
        GbError::GbInvalidRead => "GbInvalidRead",
        GbError::GbInvalidWrite => "GbInvalidWrite",
        GbError::GbUnknownError => "GbUnknownError",
    };
    log::error!("GbError: {}. Address: {:b}", error, addr);
    panic!()
}

fn draw_line(gb: &Gb<Context>, pixels: [u8; 160], line: u8) -> () {
    gb.get_context()
        .display_channel_sender
        .send(Some((pixels, line, gb.get_palette().clone())))
        .unwrap()
}

fn display_channel_listener(
    display_channel_receiver: Receiver<Option<([u8; 160], u8, [u16; 0x40])>>,
    mut display: drivers::Display<Gpio4, Gpio5>,
) -> () {
    loop {
        match display_channel_receiver.recv() {
            Ok(Some((pixels, line, palette))) => {
                display.buffer_line_gbc(pixels, line, palette);
            }
            Ok(None) => {
                display.draw();
            }
            Err(RecvError) => {
                println!("RecvError");
                break;
            }
        }
    }
}
