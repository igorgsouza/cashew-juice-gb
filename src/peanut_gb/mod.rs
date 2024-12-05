use std::ffi::{c_char, c_void, CStr};
use std::mem::{self, ManuallyDrop};
use std::ptr::{copy_nonoverlapping, null, null_mut};

use svc::hal::gpio::{Gpio4, Gpio5};
use svc::sys::tm;

use crate::drivers::{Display, DisplayMessage};
use crate::*;

const WRAM_SIZE: usize = 0x8000;
const VRAM_SIZE: usize = 0x4000;
const HRAM_IO_SIZE: usize = 0x0100;
const OAM_SIZE: usize = 0x00A0;

pub const LCD_WIDTH: u8 = 160;
pub const LCD_HEIGHT: u8 = 144;
pub const LCD_PALETTE_ALL: u8 = 0x30;
pub const JOYPAD_A: u8 = 0x01;
pub const JOYPAD_B: u8 = 0x02;
pub const JOYPAD_SELECT: u8 = 0x04;
pub const JOYPAD_START: u8 = 0x08;
pub const JOYPAD_RIGHT: u8 = 0x10;
pub const JOYPAD_LEFT: u8 = 0x20;
pub const JOYPAD_UP: u8 = 0x40;
pub const JOYPAD_DOWN: u8 = 0x80;

#[repr(C)]
#[derive(Debug)]
struct CpuRegisters {
    f: u8,
    a: u8,

    bc: u16,
    de: u16,
    hl: u16,

    sp: u16,
    pc: u16,
}

#[repr(C)]
#[derive(Debug)]
struct CountS {
    lcd_count: u16,    // uint_fast16_t
    div_count: u16,    // uint_fast16_t
    tima_count: u16,   // uint_fast16_t
    serial_count: u16, // uint_fast16_t
}

#[repr(u32)]
enum GbInitError {
    NoError = 0,
    CartridgeUnsupported,
    InvalidChecksum,
}

#[repr(u32)]
enum GbError {
    UnknownError = 0,
    InvalidOpcode,
    InvalidRead,
    InvalidWrite,
    HaltForever,
    InvalidMax,
}

#[repr(u32)] // Ensure the enum has the same underlying representation as in C
enum GbSerialRxRet {
    Success = 0,
    NoConnection = 1,
}

#[repr(C)]
#[derive(Debug)]
struct GbDisplay {
    lcd_draw_line: Option<unsafe extern "C" fn(&GbS, *const u8, u8)>,
    bg_palette: [u8; 4],
    sp_palette: [u8; 8],
    window_clear: u8,
    wy: u8,
    frame_skip_count: u8,
    interlace_count: u8,
}

#[repr(C)]
#[derive(Debug)]
struct GbCgb {
    cgb_mode: u8,
    double_speed: u8,
    double_speed_prep: u8,
    wram_bank: u8,
    wram_bank_offset: u16,
    vram_bank: u8,
    vram_bank_offset: u16,
    fix_palette: [u16; 0x40],
    oam_palette: [u8; 0x40],
    bg_palette: [u8; 0x40],
    oam_palette_id: u8,
    bg_palette_id: u8,
    oam_palette_inc: u8,
    bg_palette_inc: u8,
    dma_active: u8,
    dma_mode: u8,
    dma_size: u8,
    dma_source: u16,
    dma_dest: u16,
}
#[repr(C)]
#[derive(Debug)]
pub struct Direct {
    pub interlace: u8,
    pub frame_skip: u8,
    pub joypad: u8,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Debug)]
struct GbS {
    // Function pointers
    gb_rom_read: Option<unsafe extern "C" fn(&GbS, u32) -> u8>,
    gb_cart_ram_read: Option<unsafe extern "C" fn(&GbS, u32) -> u8>,
    gb_cart_ram_write: Option<unsafe extern "C" fn(&GbS, u32, u8)>,
    gb_error: Option<unsafe extern "C" fn(&GbS, GbError, u16)>,
    gb_serial_tx: Option<unsafe extern "C" fn(&GbS, u8)>,
    gb_serial_rx: Option<unsafe extern "C" fn(&GbS, *mut u8) -> GbSerialRxRet>,
    gb_bootrom_read: Option<unsafe extern "C" fn(&GbS, u16) -> u8>,

    // Bit-fields
    gb_halt: u8,
    gb_ime: u8,
    gb_frame: u8,
    lcd_blank: u8,

    // Cartridge information
    mbc: i8,
    cart_ram: u8,
    num_rom_banks_mask: u16,
    num_ram_banks: u8,
    selected_rom_bank: u16,
    cart_ram_bank: u8,
    enable_cart_ram: u8,
    cart_mode_select: u8,

    // Real-Time Clock (RTC) data
    cart_rtc: [u8; 5],

    // CPU registers, count, etc.
    cpu_reg: CpuRegisters,
    counter: CountS,

    // Memory arrays
    wram: [u8; WRAM_SIZE],
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    hram_io: [u8; HRAM_IO_SIZE],

    // Display settings
    display: GbDisplay,

    // Game Boy Color Mode (conditionally compiled)
    cgb: GbCgb,

    // Direct modification variables
    pub direct: Direct,
}

extern "C" {
    fn gb_init(
        gb: *mut GbS,
        gb_rom_read: extern "C" fn(*mut GbS, u32) -> u8,
        gb_cart_ram_read: extern "C" fn(*mut GbS, u32) -> u8,
        gb_cart_ram_write: extern "C" fn(*mut GbS, u32, u8),
        gb_error: extern "C" fn(*mut GbS, GbError, u16),
        private: *mut c_void,
    ) -> GbInitError;

    fn gb_run_frame(gb: *mut GbS);

    fn gb_reset(gb: *mut GbS);

    fn gb_init_lcd(gb: *mut GbS, lcd_draw_line: extern "C" fn(*mut GbS, *const u8, u8));

    fn gb_init_serial(
        gb: *mut GbS,
        gb_serial_tx: extern "C" fn(*mut GbS, u8),
        gb_serial_rx: extern "C" fn(*mut GbS, *mut u8) -> GbSerialRxRet,
    );

    fn gb_get_save_size(gb: *mut GbS) -> u32;

    fn gb_colour_hash(gb: *mut GbS) -> u8;

    fn gb_get_rom_name(gb: *mut GbS, title_str: *mut c_char) -> *const c_char;

    fn gb_tick_rtc(gb: *mut GbS);

    fn gb_set_rtc(gb: *mut GbS, time: *const tm);

    fn gb_set_bootrom(gb: *mut GbS, gb_bootrom_read: extern "C" fn(*mut GbS, u16) -> u8);
}

pub struct PeanutGb<'a> {
    gbs: Box<GbS>,
    display: Display<'a, Gpio4, Gpio5>,
}
impl<'a> PeanutGb<'a> {
    pub fn new(display: Display<'a, Gpio4, Gpio5>) -> PeanutGb<'a> {
        let mut gbs = Box::new(GbS {
            // Initialize function pointers to None
            gb_rom_read: None,
            gb_cart_ram_read: None,
            gb_cart_ram_write: None,
            gb_error: None,
            gb_serial_tx: None,
            gb_serial_rx: None,
            gb_bootrom_read: None,

            // Initialize bit-fields
            gb_halt: 0,
            gb_ime: 0,
            gb_frame: 0,
            lcd_blank: 0,

            // Initialize cartridge information to 0
            mbc: 0,
            cart_ram: 0,
            num_rom_banks_mask: 0,
            num_ram_banks: 0,
            selected_rom_bank: 0,
            cart_ram_bank: 0,
            enable_cart_ram: 0,
            cart_mode_select: 0,

            // Initialize RTC data to 0
            cart_rtc: [0; 5],

            // Initialize CPU registers and counters
            cpu_reg: CpuRegisters {
                f: 0,
                a: 0,
                bc: 0,
                de: 0,
                hl: 0,
                sp: 0,
                pc: 0,
            },
            counter: CountS {
                lcd_count: 0,
                div_count: 0,
                tima_count: 0,
                serial_count: 0,
            },

            // Initialize memory arrays to 0
            wram: [0; WRAM_SIZE],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            hram_io: [0; HRAM_IO_SIZE],

            // Initialize display settings
            display: GbDisplay {
                lcd_draw_line: None,
                bg_palette: [0; 4],
                sp_palette: [0; 8],
                window_clear: 0,
                wy: 0,
                frame_skip_count: 0,
                interlace_count: 0,
            },

            cgb: GbCgb {
                cgb_mode: 0,
                double_speed: 0,
                double_speed_prep: 0,
                wram_bank: 0,
                wram_bank_offset: 0,
                vram_bank: 0,
                vram_bank_offset: 0,
                fix_palette: [0; 0x40],
                oam_palette: [0; 0x40],
                bg_palette: [0; 0x40],
                oam_palette_id: 0,
                bg_palette_id: 0,
                oam_palette_inc: 0,
                bg_palette_inc: 0,
                dma_active: 0,
                dma_mode: 0,
                dma_size: 0,
                dma_source: 0,
                dma_dest: 0,
            },

            // Initialize direct modification variables
            direct: Direct {
                interlace: 0,
                frame_skip: 0,
                joypad: 0,
                private: null_mut(),
            },
        });
        unsafe {
            gb_init(
                &mut *gbs as *mut GbS,
                gb_rom_read,
                gb_cart_ram_read,
                gb_cart_ram_write,
                gb_error,
                null_mut(),
            );
            if CART_RAM.len() == 0 {
                let size = gb_get_save_size(&mut *gbs as *mut GbS) as usize;
                CART_RAM = Box::leak(vec![0; size].into_boxed_slice());
            }
            gb_init_lcd(&mut *gbs as *mut GbS, lcd_draw_line);
            gbs.direct.frame_skip = 1;
        }
        PeanutGb { gbs, display }
    }
    pub fn get_rom_name(&mut self) -> String {
        let mut buffer = vec![0 as c_char; 128];
        unsafe {
            let result_ptr = gb_get_rom_name(&mut *self.gbs as *mut GbS, buffer.as_mut_ptr());

            let c_str = CStr::from_ptr(result_ptr);

            c_str
                .to_str()
                .map(|s| s.to_string())
                .expect("Error converting Rom Name")
        }
    }
    pub fn get_cart_ram_size(&mut self) -> usize {
        unsafe { gb_get_save_size(&mut *self.gbs) as usize }
    }
    pub fn frame(&mut self, joypad: u8) -> () {
        unsafe {
            self.gbs.direct.joypad = joypad;
            gb_run_frame(&mut *self.gbs as *mut GbS);
        }
        if self.gbs.display.frame_skip_count == 0 {
            self.display.draw(&self.gbs.cgb.fix_palette);
        }
    }
}

extern "C" fn gb_rom_read(_gbs: *mut GbS, address: u32) -> u8 {
    unsafe { ROM[address as usize] }
}

extern "C" fn gb_cart_ram_read(gbs: *mut GbS, address: u32) -> u8 {
    unsafe { CART_RAM[address as usize] }
}

extern "C" fn gb_cart_ram_write(gbs: *mut GbS, address: u32, value: u8) {
    unsafe {
        CART_RAM[address as usize] = value;
    }
}

extern "C" fn gb_error(gbs: *mut GbS, error: GbError, code: u16) {
    return;
}

extern "C" fn lcd_draw_line(gbs: *mut GbS, pixels: *const u8, line: u8) {
    if line < 144 {
        unsafe {
            copy_nonoverlapping(
                pixels,
                FRAME_BUFFER.as_mut_ptr().add((line as usize) * 160),
                160,
            );
        }
    }
    return;
}
