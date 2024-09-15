use std::cmp;

const LOG_CYCLE: u32 = 0;
const LOG_EVERY: u32 = 10000;
const LOG_SIZE: u32 = 100000;
pub const MAX_CYCLE: u32 = LOG_CYCLE + LOG_EVERY * LOG_SIZE;

const VBLANK_INTR: u8 = 0x01;
const LCDC_INTR: u8 = 0x02;
const TIMER_INTR: u8 = 0x04;
const SERIAL_INTR: u8 = 0x08;
const CONTROL_INTR: u8 = 0x10;
const ANY_INTR: u8 = 0x1F;

#[cfg(feature = "gbc")]
const WRAM_SIZE: usize = 0x8000;
#[cfg(feature = "gbc")]
const VRAM_SIZE: usize = 0x4000;
#[cfg(not(feature = "gbc"))]
const WRAM_SIZE: usize = 0x2000;
#[cfg(not(feature = "gbc"))]
const VRAM_SIZE: usize = 0x2000;

const HRAM_IO_SIZE: usize = 0x2000;
const OAM_SIZE: usize = 0x00A0;

const ROM_0_ADDR: usize = 0x0000;
const ROM_N_ADDR: usize = 0x4000;
const VRAM_ADDR: usize = 0x8000;
const CART_RAM_ADDR: usize = 0xA000;
const WRAM_0_ADDR: usize = 0xC000;
const WRAM_1_ADDR: usize = 0xD000;
const ECHO_ADDR: usize = 0xE000;
const OAM_ADDR: usize = 0xFE00;
const UNUSED_ADDR: usize = 0xFEA0;
const IO_ADDR: usize = 0xFF00;
const HRAM_ADDR: usize = 0xFF80;
const INTR_EN_ADDR: usize = 0xFFFF;

const ROM_BANK_SIZE: usize = 0x4000;
const WRAM_BANK_SIZE: usize = 0x1000;
const CRAM_BANK_SIZE: usize = 0x2000;
const VRAM_BANK_SIZE: usize = 0x2000;

const DIV_CYCLES: u16 = 256;

const SERIAL_CYCLES: u16 = 4096;
#[cfg(feature = "gbc")]
const SERIAL_CYCLES_1KB: u16 = SERIAL_CYCLES / 1_16;
#[cfg(feature = "gbc")]
const SERIAL_CYCLES_2KB: u16 = SERIAL_CYCLES / 2_16;
#[cfg(feature = "gbc")]
const SERIAL_CYCLES_32KB: u16 = SERIAL_CYCLES / 32_16;
#[cfg(feature = "gbc")]
const SERIAL_CYCLES_64KB: u16 = SERIAL_CYCLES / 64_16;

const DMG_CLOCK_FREQ: f32 = 4194304.0;
const SCREEN_REFRESH_CYCLES: f32 = 70224.0;
const VERTICAL_SYNC: f32 = DMG_CLOCK_FREQ / SCREEN_REFRESH_CYCLES;

const RTC_CYCLES: u32 = DMG_CLOCK_FREQ as u32;

const SERIAL_SC_TX_START: u8 = 0x80;
const SERIAL_SC_CLOCK_SRC: u8 = 0x01;

const STAT_LYC_INTR: u8 = 0x40;
const STAT_MODE_2_INTR: u8 = 0x20;
const STAT_MODE_1_INTR: u8 = 0x10;
const STAT_MODE_0_INTR: u8 = 0x08;
const STAT_LYC_COINC: u8 = 0x04;
const STAT_MODE: u8 = 0x03;
const STAT_USER_BITS: u8 = 0xF8;

const LCDC_ENABLE: u8 = 0x80;
const LCDC_WINDOW_MAP: u8 = 0x40;
const LCDC_WINDOW_ENABLE: u8 = 0x20;
const LCDC_TILE_SELECT: u8 = 0x10;
const LCDC_BG_MAP: u8 = 0x08;
const LCDC_OBJ_SIZE: u8 = 0x04;
const LCDC_OBJ_ENABLE: u8 = 0x02;
const LCDC_BG_ENABLE: u8 = 0x01;

const LCD_LINE_CYCLES: u16 = 456;
const LCD_MODE_0_CYCLES: u16 = 372;
const LCD_MODE_2_CYCLES: u16 = 204;
const LCD_MODE_3_CYCLES: u16 = 284;
const LCD_VERT_LINES: u8 = 154;
pub const LCD_WIDTH: u8 = 160;
pub const LCD_HEIGHT: u8 = 144;

const VRAM_TILES_1: u16 = (0x8000 - VRAM_ADDR) as u16;
const VRAM_TILES_2: u16 = (0x8800 - VRAM_ADDR) as u16;
const VRAM_BMAP_1: u16 = (0x9800 - VRAM_ADDR) as u16;
const VRAM_BMAP_2: u16 = (0x9C00 - VRAM_ADDR) as u16;
const VRAM_TILES_3: u16 = (0x8000 - VRAM_ADDR + VRAM_BANK_SIZE) as u16;
const VRAM_TILES_4: u16 = (0x8800 - VRAM_ADDR + VRAM_BANK_SIZE) as u16;

const VBLANK_INTR_ADDR: u8 = 0x0040;
const LCDC_INTR_ADDR: u8 = 0x0048;
const TIMER_INTR_ADDR: u8 = 0x0050;
const SERIAL_INTR_ADDR: u8 = 0x0058;
const CONTROL_INTR_ADDR: u8 = 0x0060;

const NUM_SPRITES: u8 = 0x28;
const MAX_SPRITES_LINE: u8 = 0x0A;
const OBJ_PRIORITY: u8 = 0x80;
const OBJ_FLIP_Y: u8 = 0x40;
const OBJ_FLIP_X: u8 = 0x20;
const OBJ_PALETTE: u8 = 0x10;

#[cfg(feature = "gbc")]
const OBJ_BANK: u8 = 0x03;
#[cfg(feature = "gbc")]
const OBJ_CGB_PALETTE: u8 = 0x07;

pub const JOYPAD_A: u8 = 0x01;
pub const JOYPAD_B: u8 = 0x02;
pub const JOYPAD_SELECT: u8 = 0x04;
pub const JOYPAD_START: u8 = 0x08;
pub const JOYPAD_RIGHT: u8 = 0x10;
pub const JOYPAD_LEFT: u8 = 0x20;
pub const JOYPAD_UP: u8 = 0x40;
pub const JOYPAD_DOWN: u8 = 0x80;

const ROM_HEADER_CHECKSUM_LOC: u16 = 0x014D;

const IO_JOYP: usize = 0x00;
const IO_SB: usize = 0x01;
const IO_SC: usize = 0x02;
const IO_DIV: usize = 0x04;
const IO_TIMA: usize = 0x05;
const IO_TMA: usize = 0x06;
const IO_TAC: usize = 0x07;
const IO_IF: usize = 0x0F;
const IO_LCDC: usize = 0x40;
const IO_STAT: usize = 0x41;
const IO_SCY: usize = 0x42;
const IO_SCX: usize = 0x43;
const IO_LY: usize = 0x44;
const IO_LYC: usize = 0x45;
const IO_DMA: usize = 0x46;
const IO_BGP: usize = 0x47;
const IO_OBP0: usize = 0x48;
const IO_OBP1: usize = 0x49;
const IO_WY: usize = 0x4A;
const IO_WX: usize = 0x4B;
const IO_BANK: usize = 0x50;
const IO_IE: usize = 0xFF;

const IO_TAC_RATE_MASK: u8 = 0x3;
const IO_TAC_ENABLE_MASK: u8 = 0x4;

const IO_STAT_MODE_HBLANK: u8 = 0;
const IO_STAT_MODE_VBLANK: u8 = 1;
const IO_STAT_MODE_SEARCH_OAM: u8 = 2;
const IO_STAT_MODE_SEARCH_TRANSFER: u8 = 3;
const IO_STAT_MODE_VBLANK_OR_TRANSFER_MASK: u8 = 0x1;

#[cfg(feature = "lcd")]
const LCD_COLOUR: u8 = 0x03;

#[cfg(feature = "12-colour")]
const LCD_PALETTE_OBJ: u8 = 0x10;
#[cfg(feature = "12-colour")]
const LCD_PALETTE_BG: u8 = 0x20;
#[cfg(feature = "12-colour")]
pub const LCD_PALETTE_ALL: u8 = 0x30;

pub enum GbError {
    GbUnknownError,
    GbInvalidOpcode,
    GbInvalidRead,
    GbInvalidWrite,
    GbHaltForever,
    GbInvalidMax,
}

pub enum GbInitError<'a, T> {
    GbInitNoError(Gb<'a, T>),
    GbInitCartridgeUnsupported,
    GbInitInvalidChecksum,
}

enum GbSerialRxRet {
    GbSerialRxSuccess,
    GbSerialRxNoConnection,
}

struct Flags {
    byte: u8,
}
impl Flags {
    pub(super) fn new() -> Flags {
        Flags { byte: 0 }
    }
    fn get_c(&self) -> u8 {
        self.byte & 0b0001
    }
    fn get_h(&self) -> u8 {
        (self.byte & 0b0010) >> 1
    }
    fn get_n(&self) -> u8 {
        (self.byte & 0b0100) >> 2
    }
    fn get_z(&self) -> u8 {
        (self.byte & 0b1000) >> 3
    }
    fn set_c(&mut self, b: bool) -> () {
        let b = b as u8;
        self.byte &= b | 0b1110;
        self.byte |= b & 0b0001
    }
    fn set_h(&mut self, b: bool) -> () {
        let b = (b as u8) << 1;
        self.byte &= b | 0b1101;
        self.byte |= b & 0b0010
    }
    fn set_n(&mut self, b: bool) -> () {
        let b = (b as u8) << 2;
        self.byte &= b | 0b1011;
        self.byte |= b & 0b0100
    }
    fn set_z(&mut self, b: bool) -> () {
        let b = (b as u8) << 3;
        self.byte &= b | 0b0111;
        self.byte |= b & 0b1000
    }
}

struct Register {
    bytes: u16,
}
impl Register {
    fn new() -> Register {
        Register { bytes: 0 }
    }
    fn get_hi(&self) -> u8 {
        (self.bytes >> 8) as u8
    }
    fn get_lo(&self) -> u8 {
        (self.bytes & 0b11111111) as u8
    }
    fn set_hi(&mut self, b: u8) -> () {
        self.bytes &= 0b11111111;
        self.bytes |= (b as u16) << 8;
    }
    fn set_lo(&mut self, b: u8) -> () {
        self.bytes &= 0b11111111 << 8;
        self.bytes |= b as u16;
    }
}

struct CpuRegisters {
    f: Flags,
    a: u8,
    bc: Register,
    de: Register,
    hl: Register,
    sp: Register,
    pc: Register,
}
impl CpuRegisters {
    fn new() -> CpuRegisters {
        CpuRegisters {
            f: Flags::new(),
            a: 0,
            bc: Register::new(),
            de: Register::new(),
            hl: Register::new(),
            sp: Register::new(),
            pc: Register::new(),
        }
    }
}

struct Count {
    lcd_count: u16,
    div_count: u16,
    tima_count: u16,
    serial_count: u16,
    rtc_count: u32,
}
impl Count {
    fn new() -> Count {
        Count {
            lcd_count: 0,
            div_count: 0,
            tima_count: 0,
            serial_count: 0,
            rtc_count: 0,
        }
    }
}

struct CartRtc {
    bytes: [u8; 5],
}
impl CartRtc {
    fn new() -> CartRtc {
        CartRtc { bytes: [0; 5] }
    }
    fn get_sec(&self) -> u8 {
        self.bytes[0]
    }
    fn get_min(&self) -> u8 {
        self.bytes[1]
    }
    fn get_hour(&self) -> u8 {
        self.bytes[2]
    }
    fn get_yday(&self) -> u8 {
        self.bytes[3]
    }
    fn get_high(&self) -> u8 {
        self.bytes[4]
    }
    fn set_sec(&mut self, b: u8) -> () {
        self.bytes[0] = b;
    }
    fn set_min(&mut self, b: u8) -> () {
        self.bytes[1] = b;
    }
    fn set_hour(&mut self, b: u8) -> () {
        self.bytes[2] = b;
    }
    fn set_yday(&mut self, b: u8) -> () {
        self.bytes[3] = b;
    }
    fn set_high(&mut self, b: u8) -> () {
        self.bytes[4] = b;
    }
}

struct Display<T> {
    bg_palette: [u8; 4],
    sp_palette: [u8; 8],
    window_clear: u8,
    wy: u8,
    frame_skip_count: bool,
    interlace_count: bool,
    lcd_draw_line: Option<fn(&Gb<T>, [u8; 160], u8) -> ()>,
}
impl<T> Display<T> {
    fn new() -> Display<T> {
        Display {
            bg_palette: [0; 4],
            sp_palette: [0; 8],
            window_clear: 0,
            wy: 0,
            frame_skip_count: false,
            interlace_count: false,
            lcd_draw_line: None,
        }
    }
}

pub struct Direct {
    interlace: bool,
    frame_skip: bool,
    joypad: u8,
}
impl Direct {
    fn new() -> Direct {
        Direct {
            interlace: false,
            frame_skip: false,
            joypad: 0,
        }
    }
}

#[cfg(feature = "gbc")]
struct Cgb {
    mode: u8,
    double_speed: u8,
    double_speed_prep: u8,
    wram_bank: u8,
    wram_bank_offset: usize,
    vram_bank: u8,
    vram_bank_offset: usize,
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
impl Cgb {
    fn new(mode: u8) -> Cgb {
        Cgb {
            mode,
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
        }
    }
}

pub struct Gb<'a, T> {
    gb_halt: bool,   //true
    gb_ime: bool,    //true
    gb_frame: bool,  //true
    lcd_blank: bool, //true
    mbc: i8,
    cart_ram: u8,
    num_rom_banks_mask: u16,
    num_ram_banks: u8,
    selected_rom_bank: u16,
    cart_ram_bank: u8,
    enable_cart_ram: bool,
    cart_mode_select: u8,
    rtc_latched: CartRtc,
    rtc_real: CartRtc,
    cpu_reg: CpuRegisters,
    counter: Count,
    wram: Vec<u8>,
    vram: Vec<u8>,
    oam: [u8; OAM_SIZE],
    hram_io: [u8; HRAM_IO_SIZE],
    display: Display<T>,
    #[cfg(feature = "gbc")]
    cgb: Cgb,
    direct: Direct,
    gb_rom_read: fn(&Gb<T>, usize) -> u8,
    gb_cart_ram_read: fn(&Gb<T>, usize) -> u8,
    gb_cart_ram_write: fn(&Gb<T>, usize, u8) -> (),
    gb_error: Option<fn(&Gb<T>, GbError, u16) -> ()>,
    gb_serial_tx: Option<fn(&Gb<T>, u8) -> ()>,
    gb_serial_rx: Option<fn(&Gb<T>, &mut u8) -> GbSerialRxRet>,
    gb_bootrom_read: Option<fn(&Gb<T>, usize) -> u8>,
    pub cycle: u32, //rmv
    pub quit: bool, //rmv
    context: &'a T,
}

impl<'a, T> Gb<'a, T> {
    fn _adc(&mut self, r: u8, cin: u8) -> () {
        let temp = self.cpu_reg.a as u16 + r as u16 + cin as u16;
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg
            .f
            .set_h(((self.cpu_reg.a as u16 ^ r as u16 ^ temp) & 0x10) > 0);
        self.cpu_reg.f.set_n(false);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
        self.cpu_reg.a = (temp & 0xFF) as u8;
    }
    fn _adc_hl(&mut self, cin: u8) -> () {
        let temp =
            self.cpu_reg.a as u16 + self._read(self.cpu_reg.hl.bytes as usize) as u16 + cin as u16;
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg.f.set_h(
            ((self.cpu_reg.a as u16 ^ self._read(self.cpu_reg.hl.bytes as usize) as u16 ^ temp)
                & 0x10)
                > 0,
        );
        self.cpu_reg.f.set_n(false);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
        self.cpu_reg.a = (temp & 0xFF) as u8;
    }
    fn _sbc(&mut self, r: u8, cin: u8) -> () {
        let temp = (self.cpu_reg.a as i16 - (r as i16 + cin as i16)) as u16;
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg
            .f
            .set_h(((self.cpu_reg.a as u16 ^ r as u16 ^ temp) & 0x10) > 0);
        self.cpu_reg.f.set_n(true);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
        self.cpu_reg.a = (temp & 0xFF) as u8;
    }
    fn _sbc_hl(&mut self, cin: u8) -> () {
        let temp = (self.cpu_reg.a as i16
            - (self._read(self.cpu_reg.hl.bytes as usize) as i16 + cin as i16))
            as u16;
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg.f.set_h(
            ((self.cpu_reg.a as u16 ^ self._read(self.cpu_reg.hl.bytes as usize) as u16 ^ temp)
                & 0x10)
                > 0,
        );
        self.cpu_reg.f.set_n(true);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
        self.cpu_reg.a = (temp & 0xFF) as u8;
    }
    fn _and(&mut self, r: u8) -> () {
        self.cpu_reg.a &= r;
        self.cpu_reg.f.byte = 0;
        self.cpu_reg.f.set_z(self.cpu_reg.a == 0x00);
        self.cpu_reg.f.set_h(true);
    }
    fn _xor(&mut self, r: u8) -> () {
        self.cpu_reg.a ^= r;
        self.cpu_reg.f.byte = 0;
        self.cpu_reg.f.set_z(self.cpu_reg.a == 0x00);
    }
    fn _or(&mut self, r: u8) -> () {
        self.cpu_reg.a |= r;
        self.cpu_reg.f.byte = 0;
        self.cpu_reg.f.set_z(self.cpu_reg.a == 0x00);
    }
    fn _cp(&mut self, r: u8) -> () {
        let temp = (self.cpu_reg.a as u16).wrapping_sub(r as u16);
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg
            .f
            .set_h(((self.cpu_reg.a as u16 ^ r as u16 ^ temp) & 0x10) > 0);
        self.cpu_reg.f.set_n(true);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
    }
    fn _cp_hl(&mut self) -> () {
        let temp =
            (self.cpu_reg.a as u16).wrapping_sub(self._read(self.cpu_reg.hl.bytes as usize) as u16);
        self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
        self.cpu_reg.f.set_h(
            ((self.cpu_reg.a as u16 ^ self._read(self.cpu_reg.hl.bytes as usize) as u16 ^ temp)
                & 0x10)
                > 0,
        );
        self.cpu_reg.f.set_n(true);
        self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
    }
    fn _read(&self, addr: usize) -> u8 {
        match addr >> 12 {
            0x0 => {
                if self.hram_io[IO_BANK] == 0 && addr < 0x0100 {
                    return self.gb_bootrom_read.unwrap()(self, addr);
                } else {
                    return (self.gb_rom_read)(self, addr);
                }
            }
            0x1 | 0x2 | 0x3 => {
                return (self.gb_rom_read)(self, addr);
            }
            0x4 | 0x5 | 0x6 | 0x7 => {
                if self.mbc == 1 && self.cart_mode_select != 0 {
                    return (self.gb_rom_read)(
                        self,
                        addr + (((self.selected_rom_bank as usize & 0x1F) - 1) * ROM_BANK_SIZE),
                    );
                } else {
                    return (self.gb_rom_read)(
                        self,
                        addr + ((self.selected_rom_bank as usize - 1) * (ROM_BANK_SIZE)),
                    );
                }
            }
            0x8 | 0x9 => {
                #[cfg(feature = "gbc")]
                return self.vram[addr - self.cgb.vram_bank_offset];
                #[cfg(not(feature = "gbc"))]
                return self.vram[addr - VRAM_ADDR as usize];
            }
            0xA | 0xB => {
                if self.mbc == 3 && self.cart_ram_bank >= 0x08 {
                    return self.rtc_latched.bytes[self.cart_ram_bank as usize - 0x08];
                } else if self.cart_ram != 0 && self.enable_cart_ram {
                    if self.mbc == 2 {
                        return (self.gb_cart_ram_read)(self, addr & 0x1FF);
                    } else if (self.cart_mode_select != 0 || self.mbc != 1)
                        && self.cart_ram_bank < self.num_ram_banks
                    {
                        return (self.gb_cart_ram_read)(
                            self,
                            addr - CART_RAM_ADDR + (self.cart_ram_bank as usize * CRAM_BANK_SIZE),
                        );
                    } else {
                        return (self.gb_cart_ram_read)(self, addr - CART_RAM_ADDR);
                    }
                }
                return 0xFF;
            }
            0xC | 0xD => {
                #[cfg(feature = "gbc")]
                return self.wram[addr - self.cgb.wram_bank_offset];
                #[cfg(not(feature = "gbc"))]
                return self.wram[addr - WRAM_0_ADDR];
            }
            0xE => self.wram[addr - ECHO_ADDR],
            0xF => {
                if addr < OAM_ADDR {
                    #[cfg(feature = "gbc")]
                    return self.wram[(addr - 0x2000) - self.cgb.wram_bank_offset];
                    #[cfg(not(feature = "gbc"))]
                    return self.wram[addr - ECHO_ADDR];
                }
                if addr < UNUSED_ADDR {
                    return self.wram[addr - OAM_ADDR];
                }
                if addr < IO_ADDR {
                    return 0xFF;
                }
                if addr >= 0xFF10 && addr <= 0xFF3F {
                    #[cfg(feature = "sound")]
                    return audio_read(addr);
                    let ortab: [u8; 48] = [
                        0x80, 0x3f, 0x00, 0xff, 0xbf, 0xff, 0x3f, 0x00, 0xff, 0xbf, 0x7f, 0xff,
                        0x9f, 0xff, 0xbf, 0xff, 0xff, 0x00, 0x00, 0xbf, 0x00, 0x00, 0x70, 0xff,
                        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00,
                        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    ];
                    return self.hram_io[addr - IO_ADDR] | ortab[addr - IO_ADDR];
                }
                #[cfg(feature = "gbc")]
                {
                    match addr & 0xFF {
                        0x4D => {
                            return (self.cgb.double_speed << 7) + self.cgb.double_speed_prep;
                        }
                        0x4F => {
                            return self.cgb.vram_bank | 0xFE;
                            /* CGB DMA*/
                        }
                        0x51 => {
                            return (self.cgb.dma_source >> 8) as u8;
                        }
                        0x52 => {
                            return (self.cgb.dma_source & 0xF0) as u8;
                        }
                        0x53 => {
                            return (self.cgb.dma_dest >> 8) as u8;
                        }
                        0x54 => {
                            return (self.cgb.dma_dest & 0xF0) as u8;
                        }
                        0x55 => {
                            return (self.cgb.dma_active << 7) | (self.cgb.dma_size - 1);
                        }
                        0x56 => {
                            return self.hram_io[0x56];
                        }
                        0x68 => {
                            return (self.cgb.bg_palette_id & 0x3F)
                                + (self.cgb.bg_palette_inc << 7);
                        }
                        0x69 => {
                            return self.cgb.bg_palette[(self.cgb.bg_palette_id & 0x3F) as usize];
                        }
                        0x6A => {
                            return (self.cgb.oam_palette_id & 0x3F)
                                + (self.cgb.oam_palette_inc << 7);
                        }
                        0x6B => {
                            return self.cgb.oam_palette[(self.cgb.oam_palette_id & 0x3F) as usize];
                        }
                        0x70 => {
                            return self.cgb.wram_bank;
                        }
                        _ => {
                            if addr >= IO_ADDR {
                                return self.hram_io[addr - IO_ADDR];
                            }
                        }
                    }
                }
                if addr >= IO_ADDR {
                    return self.hram_io[addr - IO_ADDR];
                }
                panic!()
            }

            _ => {
                self.gb_error.unwrap()(self, GbError::GbInvalidRead, addr as u16);
                panic!()
            }
        }
    }
    fn gb_read_pc(&mut self) -> u8 {
        let pc = self.cpu_reg.pc.bytes as usize;
        self.cpu_reg.pc.bytes = self.cpu_reg.pc.bytes.wrapping_add(1);
        self._read(pc)
    }
    fn gb_read_sp(&mut self) -> u8 {
        let sp = self.cpu_reg.sp.bytes as usize;
        self.cpu_reg.sp.bytes += 1;
        self._read(sp)
    }
    fn _write(&mut self, addr: usize, val: u8) -> () {
        match addr >> 12 {
            0x0 | 0x1 => {
                if self.mbc > 0 && self.mbc != 2 && self.cart_ram != 0 {
                    self.enable_cart_ram = (val & 0x0F) == 0x0A;
                    return;
                }
                if self.mbc == 5 {
                    self.selected_rom_bank =
                        ((self.selected_rom_bank & 0x100) | val as u16) & self.num_rom_banks_mask;
                    return;
                }
                if self.mbc == 1 {
                    self.selected_rom_bank = (val as u16 & 0x1F) | (self.selected_rom_bank & 0x60);
                    if self.selected_rom_bank & 0x1F == 0x00 {
                        self.selected_rom_bank += 1;
                    }
                } else if self.mbc == 2 {
                    if addr & 0x100 != 0 {
                        self.selected_rom_bank = val as u16 & 0x0F;
                        if self.selected_rom_bank == 0 {
                            self.selected_rom_bank += 1;
                        }
                    } else {
                        self.enable_cart_ram = (val & 0x0F) == 0x0A;
                        return;
                    }
                } else if self.mbc == 3 {
                    self.selected_rom_bank = val as u16 & 0x7F;
                    if self.selected_rom_bank == 0 {
                        self.selected_rom_bank += 1;
                    }
                }
                self.selected_rom_bank = self.selected_rom_bank & self.num_rom_banks_mask;
                return;
            }
            0x2 => {
                if self.mbc == 5 {
                    self.selected_rom_bank =
                        ((self.selected_rom_bank & 0x100) | val as u16) & self.num_rom_banks_mask;
                    return;
                }
                if self.mbc == 1 {
                    self.selected_rom_bank = (val as u16 & 0x1F) | (self.selected_rom_bank & 0x60);
                    if self.selected_rom_bank & 0x1F == 0x00 {
                        self.selected_rom_bank += 1;
                    }
                } else if self.mbc == 2 {
                    if addr & 0x100 != 0 {
                        self.selected_rom_bank = val as u16 & 0x0F;
                        if self.selected_rom_bank == 0 {
                            self.selected_rom_bank += 1;
                        }
                    } else {
                        self.enable_cart_ram = (val & 0x0F) == 0x0A;
                        return;
                    }
                } else if self.mbc == 3 {
                    self.selected_rom_bank = val as u16 & 0x7F;
                    if self.selected_rom_bank == 0 {
                        self.selected_rom_bank += 1;
                    }
                }
                self.selected_rom_bank = self.selected_rom_bank & self.num_rom_banks_mask;
                return;
            }
            0x3 => {
                if self.mbc == 1 {
                    self.selected_rom_bank = (val as u16 & 0x1F) | (self.selected_rom_bank & 0x60);
                    if self.selected_rom_bank & 0x1F == 0x00 {
                        self.selected_rom_bank += 1;
                    }
                } else if self.mbc == 2 {
                    if addr & 0x100 != 0 {
                        self.selected_rom_bank = val as u16 & 0x0F;
                        if self.selected_rom_bank == 0 {
                            self.selected_rom_bank += 1;
                        }
                    } else {
                        self.enable_cart_ram = (val & 0x0F) == 0x0A;
                        return;
                    }
                } else if self.mbc == 3 {
                    self.selected_rom_bank = val as u16 & 0x7F;
                    if self.selected_rom_bank == 0 {
                        self.selected_rom_bank += 1;
                    }
                } else if self.mbc == 5 {
                    self.selected_rom_bank =
                        (val as u16 & 0x01) << 8 | (self.selected_rom_bank & 0xFF)
                }
                self.selected_rom_bank = self.selected_rom_bank & self.num_rom_banks_mask;
                return;
            }
            0x4 | 0x5 => {
                if self.mbc == 1 {
                    self.cart_ram_bank = val & 3;
                    self.selected_rom_bank = (((val as u16 & 3) << 5)
                        | (self.selected_rom_bank & 0x1F))
                        | self.num_rom_banks_mask;
                } else if self.mbc == 3 {
                    self.cart_ram_bank = val;
                } else if self.mbc == 5 {
                    self.cart_ram_bank = val & 0x0F;
                }
                return;
            }
            0x6 | 0x7 => {
                let val = val & 1;
                if self.mbc == 3 && val != 0 && self.cart_mode_select == 0 {
                    self.rtc_latched.bytes.copy_from_slice(&self.rtc_real.bytes)
                }
                self.cart_mode_select = val;
                return;
            }
            0x8 | 0x9 => {
                #[cfg(feature = "gbc")]
                {
                    self.vram[addr - self.cgb.vram_bank_offset] = val;
                }
                #[cfg(not(feature = "gbc"))]
                {
                    self.vram[addr - VRAM_ADDR] = val;
                }
                return;
            }
            0xA | 0xB => {
                if self.mbc == 3 && self.cart_ram_bank >= 0x08 {
                    let rtc_reg_mask: [u8; 5] = [0x3F, 0x3F, 0x1F, 0xFF, 0xC1];
                    let reg = (self.cart_ram_bank - 0x08) as usize;
                    self.rtc_real.bytes[reg] = val & rtc_reg_mask[reg]
                } else if self.cart_ram != 0 && self.enable_cart_ram {
                    if self.mbc == 2 {
                        let addr = addr & 0x1FF;
                        let val = val & 0x0F;
                        (self.gb_cart_ram_write)(self, addr, val);
                    } else if self.cart_mode_select != 0 && self.cart_ram_bank < self.num_ram_banks
                    {
                        (self.gb_cart_ram_write)(
                            self,
                            addr - CART_RAM_ADDR
                                + (self.cart_ram_bank as usize * CRAM_BANK_SIZE as usize),
                            val,
                        )
                    } else if self.num_ram_banks != 0 {
                        (self.gb_cart_ram_write)(self, addr - CART_RAM_ADDR, val)
                    }
                }
                return;
            }
            0xC => {
                self.wram[addr - WRAM_0_ADDR] = val;
                return;
            }
            0xD => {
                #[cfg(feature = "gbc")]
                {
                    self.wram[addr - self.cgb.wram_bank_offset] = val;
                }
                #[cfg(not(feature = "gbc"))]
                {
                    self.wram[addr - WRAM_1_ADDR + WRAM_BANK_SIZE] = val;
                }
                return;
            }
            0xE => {
                self.wram[addr - ECHO_ADDR] = val;
            }
            0xF => {
                if addr < OAM_ADDR {
                    #[cfg(feature = "gbc")]
                    {
                        self.wram[(addr - 0x2000) - self.cgb.wram_bank_offset] = val;
                    }
                    #[cfg(not(feature = "gbc"))]
                    {
                        self.wram[addr - ECHO_ADDR] = val;
                    }

                    return;
                }
                if addr < UNUSED_ADDR {
                    self.oam[addr - OAM_ADDR] = val;
                    return;
                }
                if addr < IO_ADDR {
                    return;
                }
                if HRAM_ADDR <= addr && addr < INTR_EN_ADDR {
                    self.hram_io[addr - IO_ADDR] = val;
                    return;
                }
                if addr >= 0xFF10 && addr <= 0xFF3F {
                    #[cfg(feature = "sound")]
                    {
                        audio_write(addr, val);
                        return;
                    }
                    self.hram_io[addr - IO_ADDR] = val;
                    return;
                }
                match addr & 0xFF {
                    0x00 => {
                        self.hram_io[IO_JOYP] = val;
                        if self.hram_io[IO_JOYP] & 0x10 == 0 {
                            self.hram_io[IO_JOYP] |= self.direct.joypad >> 4;
                        } else {
                            self.hram_io[IO_JOYP] |= self.direct.joypad & 0x0F;
                        }
                        return;
                    }
                    0x01 => {
                        self.hram_io[IO_SB] = val;
                        return;
                    }
                    0x02 => {
                        self.hram_io[IO_SC] = val;
                        return;
                    }
                    0x04 => {
                        self.hram_io[IO_DIV] = 0x00;
                        return;
                    }
                    0x05 => {
                        self.hram_io[IO_TIMA] = val;
                        return;
                    }
                    0x06 => {
                        self.hram_io[IO_TMA] = val;
                        return;
                    }
                    0x07 => {
                        self.hram_io[IO_TAC] = val;
                        return;
                    }
                    0x0F => {
                        self.hram_io[IO_IF] = val | 0xE0;
                        return;
                    }
                    0x40 => {
                        let lcd_enabled = (self.hram_io[IO_LCDC] & LCDC_ENABLE) != 0;

                        self.hram_io[IO_LCDC] = val;

                        if !lcd_enabled && (val & LCDC_ENABLE) != 0 {
                            self.lcd_blank = true;
                        } else if lcd_enabled && (val & LCDC_ENABLE) == 0 {
                            self.hram_io[IO_STAT] =
                                (self.hram_io[IO_STAT] & !STAT_MODE) | IO_STAT_MODE_HBLANK;
                            self.hram_io[IO_LY] = 0;
                            self.counter.lcd_count = 0;
                        }
                        return;
                    }
                    0x41 => {
                        self.hram_io[IO_STAT] =
                            (val & STAT_USER_BITS) | (self.hram_io[IO_STAT] & STAT_MODE);
                        return;
                    }
                    0x42 => {
                        self.hram_io[IO_SCY] = val;
                        return;
                    }
                    0x43 => {
                        self.hram_io[IO_SCX] = val;
                        return;
                    }
                    0x45 => {
                        self.hram_io[IO_LYC] = val;
                        return;
                    }
                    0x46 => {
                        let dma_addr: usize;
                        #[cfg(feature = "gbc")]
                        {
                            dma_addr = ((val % 0xF1) as usize) << 8;
                            self.hram_io[IO_DMA] = val % 0xF1;
                        }
                        #[cfg(not(feature = "gbc"))]
                        {
                            dma_addr = (val as usize) << 8;
                            self.hram_io[IO_DMA] = val;
                        }

                        for i in 0..OAM_SIZE {
                            self.oam[i] = self._read(dma_addr + i)
                        }
                        return;
                    }
                    0x47 => {
                        self.hram_io[IO_BGP] = val;
                        self.display.bg_palette[0] = self.hram_io[IO_BGP] & 0x03;
                        self.display.bg_palette[1] = (self.hram_io[IO_BGP] >> 2) & 0x03;
                        self.display.bg_palette[2] = (self.hram_io[IO_BGP] >> 4) & 0x03;
                        self.display.bg_palette[3] = (self.hram_io[IO_BGP] >> 6) & 0x03;
                        return;
                    }
                    0x48 => {
                        self.hram_io[IO_OBP0] = val;
                        self.display.sp_palette[0] = self.hram_io[IO_OBP0] & 0x03;
                        self.display.sp_palette[1] = (self.hram_io[IO_OBP0] >> 2) & 0x03;
                        self.display.sp_palette[2] = (self.hram_io[IO_OBP0] >> 4) & 0x03;
                        self.display.sp_palette[3] = (self.hram_io[IO_OBP0] >> 6) & 0x03;
                        return;
                    }
                    0x49 => {
                        self.hram_io[IO_OBP1] = val;
                        self.display.sp_palette[4] = self.hram_io[IO_OBP1] & 0x03;
                        self.display.sp_palette[5] = (self.hram_io[IO_OBP1] >> 2) & 0x03;
                        self.display.sp_palette[6] = (self.hram_io[IO_OBP1] >> 4) & 0x03;
                        self.display.sp_palette[7] = (self.hram_io[IO_OBP1] >> 6) & 0x03;
                        return;
                    }
                    0x4A => {
                        self.hram_io[IO_WY] = val;
                        return;
                    }
                    0x4B => {
                        self.hram_io[IO_WX] = val;
                        return;
                    }

                    0x50 => {
                        self.hram_io[IO_BANK] = 0x01;
                        return;
                    }
                    0xFF => {
                        self.hram_io[IO_IE] = val;
                        return;
                    }
                    _ => {
                        #[cfg(feature = "gbc")]
                        {
                            match addr & 0xFF {
                                0x4D => {
                                    self.cgb.double_speed_prep = val & 1;
                                    return;
                                }
                                0x4F => {
                                    self.cgb.vram_bank = val & 0x01;
                                    if self.cgb.mode != 0 {
                                        self.cgb.vram_bank_offset =
                                            VRAM_ADDR - ((self.cgb.vram_bank as usize) << 13);
                                    }
                                }

                                0x51 => {
                                    self.cgb.dma_source =
                                        (self.cgb.dma_source & 0xFF) + ((val as u16) << 8);
                                    return;
                                }
                                0x52 => {
                                    self.cgb.dma_source =
                                        (self.cgb.dma_source & 0xFF00) + val as u16;
                                    return;
                                }
                                0x53 => {
                                    self.cgb.dma_dest =
                                        (self.cgb.dma_dest & 0xFF) + ((val as u16) << 8);
                                    return;
                                }
                                0x54 => {
                                    self.cgb.dma_dest = (self.cgb.dma_dest & 0xFF00) + val as u16;
                                    return;
                                }
                                0x55 => {
                                    self.cgb.dma_size = (val & 0x7F) + 1;
                                    self.cgb.dma_mode = val >> 7;
                                    if self.cgb.dma_active != 0 {
                                        if self.cgb.mode != 0 && self.cgb.dma_mode == 0 {
                                            for i in 0..(self.cgb.dma_size << 4) as usize {
                                                self._write(
                                                    ((self.cgb.dma_dest as usize & 0x1FF0)
                                                        | 0x8000)
                                                        + i,
                                                    self._read(
                                                        (self.cgb.dma_source as usize & 0xFFF0) + i,
                                                    ),
                                                );
                                            }
                                            self.cgb.dma_source += (self.cgb.dma_size as u16) << 4;
                                            self.cgb.dma_dest += (self.cgb.dma_size as u16) << 4;
                                            self.cgb.dma_size = 0;
                                        }
                                    }
                                    self.cgb.dma_active = self.cgb.dma_mode ^ 1;
                                    return;
                                }
                                0x56 => {
                                    self.hram_io[0x56] = val;
                                    return;
                                }
                                0x68 => {
                                    self.cgb.bg_palette_id = val & 0x3F;
                                    self.cgb.bg_palette_inc = val >> 7;
                                    return;
                                }
                                0x69 => {
                                    self.cgb.bg_palette[(self.cgb.bg_palette_id & 0x3F) as usize] =
                                        val;
                                    let fix_palette_temp = ((self.cgb.bg_palette
                                        [(self.cgb.bg_palette_id as usize & 0x3E) + 1]
                                        as u16)
                                        << 8)
                                        + (self.cgb.bg_palette
                                            [self.cgb.bg_palette_id as usize & 0x3E])
                                            as u16;
                                    self.cgb.fix_palette
                                        [(self.cgb.bg_palette_id as usize & 0x3E) >> 1] =
                                        ((fix_palette_temp & 0x7C00) >> 10)
                                            | (fix_palette_temp & 0x03E0)
                                            | ((fix_palette_temp & 0x001F) << 10);
                                    if self.cgb.bg_palette_inc != 0 {
                                        self.cgb.bg_palette_id += 1;
                                        self.cgb.bg_palette_id = (self.cgb.bg_palette_id) & 0x3F;
                                    }
                                    return;
                                }
                                0x6A => {
                                    self.cgb.oam_palette_id = val & 0x3F;
                                    self.cgb.oam_palette_inc = val >> 7;
                                    return;
                                }
                                0x6B => {
                                    self.cgb.oam_palette[self.cgb.oam_palette_id as usize & 0x3F] =
                                        val;
                                    let fix_palette_temp = ((self.cgb.oam_palette
                                        [(self.cgb.oam_palette_id as usize & 0x3E) + 1]
                                        as u16)
                                        << 8)
                                        + (self.cgb.oam_palette
                                            [self.cgb.oam_palette_id as usize & 0x3E])
                                            as u16;
                                    self.cgb.fix_palette
                                        [0x20 + ((self.cgb.oam_palette_id as usize & 0x3E) >> 1)] =
                                        ((fix_palette_temp & 0x7C00) >> 10)
                                            | (fix_palette_temp & 0x03E0)
                                            | ((fix_palette_temp & 0x001F) << 10);
                                    if self.cgb.oam_palette_inc != 0 {
                                        self.cgb.oam_palette_id += 1;
                                        self.cgb.oam_palette_id = (self.cgb.oam_palette_id) & 0x3F;
                                    }
                                    return;
                                }
                                0x70 => {
                                    self.cgb.wram_bank = val;
                                    self.cgb.wram_bank_offset = WRAM_1_ADDR - (1 << 12);
                                    if self.cgb.mode != 0 && (self.cgb.wram_bank & 7) > 0 {
                                        self.cgb.wram_bank_offset =
                                            WRAM_1_ADDR - ((self.cgb.wram_bank as usize & 7) << 12);
                                    }
                                    return;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {
                return;
            }
        }
    }

    fn _execute_cb(&mut self) -> u8 {
        let mut inst_cycles = 8_u8;
        let mut writeback = 1_u8;
        let mut val = 0;
        let mut cbop = self.gb_read_pc();
        let r = cbop & 0x7;
        let b = (cbop >> 3) & 0x7;
        let d = (cbop >> 3) & 0x1;

        match cbop & 0xC7 {
            0x06 | 0x86 | 0xC6 => {
                inst_cycles += 8;
            }
            0x46 => {
                inst_cycles += 4;
            }
            _ => (),
        }

        match r {
            0 => {
                val = self.cpu_reg.bc.get_hi();
            }
            1 => {
                val = self.cpu_reg.bc.get_lo();
            }
            2 => {
                val = self.cpu_reg.de.get_hi();
            }
            3 => {
                val = self.cpu_reg.de.get_lo();
            }
            4 => {
                val = self.cpu_reg.hl.get_hi();
            }
            5 => {
                val = self.cpu_reg.hl.get_lo();
            }
            6 => {
                val = self._read(self.cpu_reg.hl.bytes as usize);
            }
            _ => val = self.cpu_reg.a,
        }

        match cbop >> 6 {
            0x0 => {
                cbop = (cbop >> 4) & 0x3;
                match cbop {
                    0x0 | 0x1 => {
                        if d != 0 {
                            let temp = val;
                            val = val >> 1;
                            val |= {
                                if cbop != 0 {
                                    self.cpu_reg.f.get_c() << 7
                                } else {
                                    temp << 7
                                }
                            };
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_z(val == 0x00);
                            self.cpu_reg.f.set_c(temp & 0x00 != 0);
                        } else {
                            let temp = val;
                            val = val << 1;
                            val |= {
                                if cbop != 0 {
                                    self.cpu_reg.f.get_c()
                                } else {
                                    temp >> 7
                                }
                            };
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_z(val == 0x00);
                            self.cpu_reg.f.set_c(temp >> 7 != 0);
                        }
                    }
                    0x2 => {
                        if d != 0 {
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_c(val & 0x01 != 0);
                            val = (val >> 1) | (val & 0x80);
                            self.cpu_reg.f.set_z(val == 0x00);
                        } else {
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_c(val >> 7 != 0);
                            val = val << 1;
                            self.cpu_reg.f.set_z(val == 0x00);
                        }
                    }
                    0x3 => {
                        if d != 0 {
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_c(val & 0x01 != 0);
                            val = val >> 1;
                            self.cpu_reg.f.set_z(val == 0x00);
                        } else {
                            let temp = ((val >> 4) & 0x0F) | ((val << 4) & 0xF0);
                            val = temp;
                            self.cpu_reg.f.byte = 0;
                            self.cpu_reg.f.set_z(val == 0x00);
                        }
                    }
                    _ => {}
                }
            }
            0x1 => {
                self.cpu_reg.f.set_z((val >> b) & 0x1 == 0);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(true);
                writeback = 0;
            }
            0x2 => {
                val &= (0xFE << b) | (0xFF_u8.checked_shr(8 - b as u32).unwrap_or(0));
                //checked_shr peformance?
            }
            0x3 => {
                val |= 0x1 << b;
            }
            _ => {}
        }

        if writeback != 0 {
            match r {
                0 => {
                    self.cpu_reg.bc.set_hi(val);
                }
                1 => {
                    self.cpu_reg.bc.set_lo(val);
                }
                2 => {
                    self.cpu_reg.de.set_hi(val);
                }
                3 => {
                    self.cpu_reg.de.set_lo(val);
                }
                4 => {
                    self.cpu_reg.hl.set_hi(val);
                }
                5 => {
                    self.cpu_reg.hl.set_lo(val);
                }
                6 => {
                    self._write(self.cpu_reg.hl.bytes as usize, val);
                }
                7 => {
                    self.cpu_reg.a = val;
                }
                _ => {}
            }
        }

        return inst_cycles;
    }

    #[cfg(all(feature = "lcd", not(feature = "gbc")))]
    fn _draw_line(&mut self) -> () {
        use std::cmp;

        let mut pixels = [0; 160];

        if self.display.lcd_draw_line.is_none() {
            return;
        }

        if self.direct.frame_skip && !self.display.frame_skip_count {
            return;
        }

        if self.direct.interlace {
            if (!self.display.interlace_count && (self.hram_io[IO_LY] & 1) == 0)
                || (self.display.interlace_count && (self.hram_io[IO_LY] & 1) == 1)
            {
                if (self.hram_io[IO_LCDC] & LCDC_WINDOW_ENABLE) != 0
                    && self.hram_io[IO_LY] >= self.display.wy
                    && self.hram_io[IO_WX] <= 166
                {
                    self.display.window_clear += 1;
                }
                return;
            }
        }

        if (self.hram_io[IO_LCDC] & LCDC_BG_ENABLE) != 0 {
            let bg_y = self.hram_io[IO_LY].wrapping_add(self.hram_io[IO_SCY]);
            let bg_map = {
                if self.hram_io[IO_LCDC] & LCDC_BG_MAP != 0 {
                    VRAM_BMAP_2
                } else {
                    VRAM_BMAP_1
                }
            } + (bg_y as u16 >> 3) * 0x20;
            let mut disp_x = LCD_WIDTH - 1;
            let mut bg_x = disp_x.wrapping_add(self.hram_io[IO_SCX]);
            let mut idx = self.vram[bg_map as usize + (bg_x as usize >> 3)];
            let py = bg_y & 0x07;
            let mut px = 7 - (bg_x & 0x07);

            let mut tile;
            if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                tile = VRAM_TILES_1 + (idx as u16) * 0x10;
            } else {
                tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
            }
            tile += 2 * py as u16;

            let mut t1 = self.vram[tile as usize] >> px;
            let mut t2 = self.vram[tile as usize + 1] >> px;

            while disp_x != 0xFF {
                if px == 8 {
                    px = 0;
                    bg_x = disp_x.wrapping_add(self.hram_io[IO_SCX]);
                    idx = self.vram[(bg_map + (bg_x as u16 >> 3)) as usize];

                    if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                        tile = VRAM_TILES_1 + idx as u16 * 0x10;
                    } else {
                        tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
                    }

                    tile += 2 * py as u16;
                    t1 = self.vram[tile as usize];
                    t2 = self.vram[tile as usize + 1];
                }

                let c = (t1 & 0x1) | ((t2 & 0x1) << 1);
                pixels[disp_x as usize] = self.display.bg_palette[c as usize];
                #[cfg(feature = "12-colour")]
                {
                    pixels[disp_x as usize] |= LCD_PALETTE_BG;
                }

                t1 = t1 >> 1;
                t2 = t2 >> 1;
                px += 1;

                disp_x = disp_x.wrapping_sub(1);
            }
        }

        if (self.hram_io[IO_LCDC] & LCDC_WINDOW_ENABLE) != 0
            && self.hram_io[IO_LY] >= self.display.wy
            && self.hram_io[IO_WX] <= 166
        {
            let mut win_line = {
                if (self.hram_io[IO_LCDC] & LCDC_WINDOW_MAP) != 0 {
                    VRAM_BMAP_2
                } else {
                    VRAM_BMAP_1
                }
            };
            win_line += (self.display.window_clear as u16 >> 3) * 0x20;

            let mut disp_x = LCD_WIDTH - 1;
            let mut win_x = disp_x - self.hram_io[IO_WX] + 7;

            let py = self.display.window_clear & 0x07;
            let mut px = 7 - (win_x & 0x07);
            let mut idx = self.vram[(win_line + (win_x as u16 >> 3)) as usize];

            let mut tile;
            if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                tile = VRAM_TILES_1 + idx as u16 * 0x10;
            } else {
                tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
            }

            tile += 2 * py as u16;

            let mut t1 = self.vram[tile as usize] >> px;
            let mut t2 = self.vram[tile as usize + 1] >> px;

            let end = {
                if self.hram_io[IO_WX] < 7 {
                    0
                } else {
                    self.hram_io[IO_WX] - 7
                }
            }
            .wrapping_sub(1);

            while disp_x != end {
                if px == 8 {
                    px = 0;
                    win_x = disp_x - self.hram_io[IO_WX] + 7;
                    idx = self.vram[(win_line + (win_x as u16 >> 3)) as usize];

                    if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                        tile = VRAM_TILES_1 + idx as u16 * 0x10;
                    } else {
                        tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
                    }

                    tile += 2 * py as u16;
                    t1 = self.vram[tile as usize];
                    t2 = self.vram[tile as usize + 1];
                }

                let c = (t1 & 0x1) | ((t2 & 0x1) << 1);
                pixels[disp_x as usize] = self.display.bg_palette[c as usize];
                #[cfg(feature = "12-colour")]
                {
                    pixels[disp_x as usize] |= LCD_PALETTE_BG;
                }
                t1 = t1 >> 1;
                t2 = t2 >> 1;
                px += 1;

                disp_x -= 1;
            }

            self.display.window_clear += 1;
        }

        if (self.hram_io[IO_LCDC] & LCDC_OBJ_ENABLE) != 0 {
            let mut sprite_number: u8;
            let mut sprites_to_render: [Option<SpriteData>; NUM_SPRITES as usize] =
                [Option::None; NUM_SPRITES as usize];
            #[cfg(feature = "high-lcd-accuracy")]
            {
                let mut number_of_sprites = 0_u8;

                for sprite_number in 0..sprites_to_render.len() {
                    let oy = self.oam[4 * sprite_number as usize + 0];
                    let ox = self.oam[4 * sprite_number as usize + 1];

                    if self.hram_io[IO_LY] + {
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            0
                        } else {
                            8
                        }
                    } >= oy
                        || self.hram_io[IO_LY] + 16 < oy
                    {
                        continue;
                    }

                    sprites_to_render[number_of_sprites as usize] = Some(SpriteData {
                        sprite_number: sprite_number as u8,
                        x: ox,
                    });
                    number_of_sprites += 1;
                }

                sprites_to_render.sort_unstable_by(|a, b| compare_sprites(a, b));
                if number_of_sprites > MAX_SPRITES_LINE {
                    number_of_sprites = MAX_SPRITES_LINE
                }

                sprite_number = number_of_sprites.wrapping_sub(1);
            }
            #[cfg(not(feature = "high-lcd-accuracy"))]
            {
                drop(sprites_to_render);
                sprite_number = NUM_SPRITES - 1;
            }
            while sprite_number != 0xFF {
                let s: u8;
                #[cfg(feature = "high-lcd-accuracy")]
                {
                    s = sprites_to_render[sprite_number as usize]
                        .unwrap()
                        .sprite_number
                }
                #[cfg(not(feature = "high-lcd-accuracy"))]
                {
                    s = sprite_number;
                }

                let oy = self.oam[4 * s as usize + 0];
                let ox = self.oam[4 * s as usize + 1];
                let ot = self.oam[4 * s as usize + 2]
                    & if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                        0xFE
                    } else {
                        0xFF
                    };
                let of = self.oam[4 * s as usize + 3];

                #[cfg(not(feature = "high-lcd-accuracy"))]
                {
                    if (self.hram_io[IO_LY] + {
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            0
                        } else {
                            8
                        }
                    } >= oy
                        || self.hram_io[IO_LY] + 16 < oy)
                    {
                        continue;
                    }
                }

                if ox == 0 || ox >= 168 {
                    sprite_number = sprite_number.wrapping_sub(1);
                    continue;
                }

                let mut py = self.hram_io[IO_LY].wrapping_sub(oy - 16);

                if of & OBJ_FLIP_Y != 0 {
                    py = ({
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            15
                        } else {
                            7
                        }
                    }) - py;
                }

                let mut t1 =
                    self.vram[VRAM_TILES_1 as usize + ot as usize * 0x10 + 2 * py as usize];
                let mut t2 =
                    self.vram[VRAM_TILES_1 as usize + ot as usize * 0x10 + 2 * py as usize + 1];

                let shift;
                let dir;
                let start;
                let end;
                if of & OBJ_FLIP_X != 0 {
                    dir = 1;
                    start = {
                        if ox < 8 {
                            0
                        } else {
                            ox - 8
                        }
                    };
                    end = cmp::min(ox, LCD_WIDTH);
                    shift = 8 - (ox - start);
                } else {
                    dir = u8::MAX;
                    start = cmp::min(ox, LCD_WIDTH) - 1;
                    end = ({
                        if ox < 8 {
                            0
                        } else {
                            ox - 8
                        }
                    })
                    .wrapping_sub(1);
                    shift = ox - (start + 1);
                }

                t1 >>= shift;
                t2 >>= shift;

                let mut disp_x = start;
                while disp_x != end {
                    let c: u8 = (t1 & 0x1) | ((t2 & 0x1) << 1);

                    if c != 0
                        && !((of & OBJ_PRIORITY) != 0
                            && !((pixels[disp_x as usize] & 0x3) == self.display.bg_palette[0]))
                    {
                        /* Set pixel colour. */
                        pixels[disp_x as usize] = {
                            if (of & OBJ_PALETTE) != 0 {
                                self.display.sp_palette[c as usize + 4]
                            } else {
                                self.display.sp_palette[c as usize]
                            }
                        };

                        #[cfg(feature = "12-colour")]
                        {
                            pixels[disp_x as usize] |= of & OBJ_PALETTE;
                        }
                    }

                    t1 = t1 >> 1;
                    t2 = t2 >> 1;

                    disp_x = disp_x.wrapping_add(dir);
                }

                sprite_number = sprite_number.wrapping_sub(1);
            }
        }

        self.display.lcd_draw_line.unwrap()(self, pixels, self.hram_io[IO_LY]);
    }

    #[cfg(all(feature = "lcd", feature = "gbc"))]
    fn _draw_line(&mut self) -> () {
        use std::cmp;

        let mut pixels = [0; 160];
        let mut pixels_prio = [0; 160];

        if self.display.lcd_draw_line.is_none() {
            return;
        }

        if self.direct.frame_skip && !self.display.frame_skip_count {
            return;
        }

        if self.direct.interlace {
            if (!self.display.interlace_count && (self.hram_io[IO_LY] & 1) == 0)
                || (self.display.interlace_count && (self.hram_io[IO_LY] & 1) == 1)
            {
                if (self.hram_io[IO_LCDC] & LCDC_WINDOW_ENABLE) != 0
                    && self.hram_io[IO_LY] >= self.display.wy
                    && self.hram_io[IO_WX] <= 166
                {
                    self.display.window_clear += 1;
                }
                return;
            }
        }

        if self.cgb.mode != 0 || (self.hram_io[IO_LCDC] & LCDC_BG_ENABLE) != 0 {
            let bg_y = self.hram_io[IO_LY].wrapping_add(self.hram_io[IO_SCY]);
            let bg_map = {
                if self.hram_io[IO_LCDC] & LCDC_BG_MAP != 0 {
                    VRAM_BMAP_2
                } else {
                    VRAM_BMAP_1
                }
            } + (bg_y as u16 >> 3) * 0x20;
            let mut disp_x = LCD_WIDTH - 1;
            let mut bg_x = disp_x.wrapping_add(self.hram_io[IO_SCX]);
            let mut idx = self.vram[bg_map as usize + (bg_x as usize >> 3)];
            let mut idx_att = self.vram[bg_map as usize + (bg_x as usize >> 3) + 0x2000];
            let py = bg_y & 0x07;
            let mut px = 7 - (bg_x & 0x07);

            let mut tile;
            if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                tile = VRAM_TILES_1 + (idx as u16) * 0x10;
            } else {
                tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
            }

            if self.cgb.mode != 0 {
                if idx_att & 0x08 != 0 {
                    tile += 0x2000;
                }
                if idx_att & 0x40 != 0 {
                    tile += 2 * (7 - py as u16);
                }
            }
            if idx_att & 0x40 == 0 {
                tile += 2 * py as u16;
            }

            let mut t1;
            let mut t2;
            if self.cgb.mode != 0 && (idx_att & 0x20) != 0 {
                t1 = self.vram[tile as usize] << px;
                t2 = self.vram[tile as usize + 1] << px;
            } else {
                t1 = self.vram[tile as usize] >> px;
                t2 = self.vram[tile as usize + 1] >> px;
            }

            while disp_x != 0xFF {
                if px == 8 {
                    px = 0;
                    bg_x = disp_x.wrapping_add(self.hram_io[IO_SCX]);
                    idx = self.vram[(bg_map + (bg_x as u16 >> 3)) as usize];

                    idx_att = self.vram[bg_map as usize + (bg_x as usize >> 3) + 0x2000];

                    if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                        tile = VRAM_TILES_1 + idx as u16 * 0x10;
                    } else {
                        tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
                    }

                    if self.cgb.mode != 0 {
                        if idx_att & 0x08 != 0 {
                            tile += 0x2000;
                        }
                        if idx_att & 0x40 != 0 {
                            tile += 2 * (7 - py as u16);
                        }
                    }
                    if idx_att & 0x40 == 0 {
                        tile += 2 * py as u16;
                    }
                    t1 = self.vram[tile as usize];
                    t2 = self.vram[tile as usize + 1];
                }

                if self.cgb.mode != 0 && (idx_att & 0x20) != 0 {
                    let c = (((t1 & 0x80) >> 1) | (t2 & 0x80)) >> 6;
                    pixels[disp_x as usize] = ((idx_att & 0x07) << 2) + c;
                    pixels_prio[disp_x as usize] = idx_att >> 7;
                    t1 = t1 << 1;
                    t2 = t2 << 1;
                } else {
                    let c = (t1 & 0x1) | ((t2 & 0x1) << 1);
                    if self.cgb.mode != 0 {
                        pixels[disp_x as usize] = ((idx_att & 0x07) << 2) + c;
                        pixels_prio[disp_x as usize] = idx_att >> 7;
                    } else {
                        pixels[disp_x as usize] = self.display.bg_palette[c as usize];
                        #[cfg(feature = "12-colour")]
                        {
                            pixels[disp_x as usize] |= LCD_PALETTE_BG;
                        }
                    }
                    t1 = t1 >> 1;
                    t2 = t2 >> 1;
                }

                px += 1;
                disp_x = disp_x.wrapping_sub(1);
            }
        }

        if (self.hram_io[IO_LCDC] & LCDC_WINDOW_ENABLE) != 0
            && self.hram_io[IO_LY] >= self.display.wy
            && self.hram_io[IO_WX] <= 166
        {
            let mut win_line = {
                if (self.hram_io[IO_LCDC] & LCDC_WINDOW_MAP) != 0 {
                    VRAM_BMAP_2
                } else {
                    VRAM_BMAP_1
                }
            };
            win_line += (self.display.window_clear as u16 >> 3) * 0x20;

            let mut disp_x = LCD_WIDTH - 1;
            let mut win_x = disp_x - self.hram_io[IO_WX] + 7;

            let py = self.display.window_clear & 0x07;
            let mut px = 7 - (win_x & 0x07);
            let mut idx = self.vram[(win_line + (win_x as u16 >> 3)) as usize];
            let mut idx_att = self.vram[win_line as usize + (win_x as usize >> 3) + 0x2000];

            let mut tile;
            if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                tile = VRAM_TILES_1 + idx as u16 * 0x10;
            } else {
                tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
            }

            if self.cgb.mode != 0 {
                if idx_att & 0x0 != 0 {
                    tile += 0x2000;
                }
                if idx_att & 0x40 != 0 {
                    tile += 2 * (7 - py as u16);
                }
            }
            if idx_att & 0x40 == 0 {
                tile += 2 * py as u16;
            }

            let mut t1;
            let mut t2;
            if self.cgb.mode != 0 && (idx_att & 0x20) != 0 {
                t1 = self.vram[tile as usize] << px;
                t2 = self.vram[tile as usize + 1] << px;
            } else {
                t1 = self.vram[tile as usize] >> px;
                t2 = self.vram[tile as usize + 1] >> px;
            }
            let end = {
                if self.hram_io[IO_WX] < 7 {
                    0
                } else {
                    self.hram_io[IO_WX] - 7
                }
            }
            .wrapping_sub(1);

            while disp_x != end {
                if px == 8 {
                    px = 0;
                    win_x = disp_x - self.hram_io[IO_WX] + 7;
                    idx = self.vram[(win_line + (win_x as u16 >> 3)) as usize];
                    idx_att = self.vram[win_line as usize + (win_x as usize >> 3) + 0x2000];

                    if (self.hram_io[IO_LCDC] & LCDC_TILE_SELECT) != 0 {
                        tile = VRAM_TILES_1 + idx as u16 * 0x10;
                    } else {
                        tile = VRAM_TILES_2 + ((idx as u16 + 0x80) % 0x100) * 0x10;
                    }

                    if self.cgb.mode != 0 {
                        if idx_att & 0x08 != 0 {
                            tile += 0x2000;
                        } //VRAM bank 2
                        if idx_att & 0x40 != 0 {
                            tile += 2 * (7 - py as u16)
                        };
                    }
                    if idx_att & 0x40 == 0 {
                        tile += 2 * py as u16;
                    }

                    t1 = self.vram[tile as usize];
                    t2 = self.vram[tile as usize + 1];
                }

                if idx_att & 0x20 != 0 {
                    let c = (((t1 & 0x80) >> 1) | (t2 & 0x80)) >> 6;
                    pixels[disp_x as usize] = ((idx_att & 0x07) << 2) + c;
                    pixels_prio[disp_x as usize] = idx_att >> 7;
                    t1 = t1 << 1;
                    t2 = t2 << 1;
                } else {
                    let c = (t1 & 0x1) | ((t2 & 0x1) << 1);
                    if self.cgb.mode != 0 {
                        pixels[disp_x as usize] = ((idx_att & 0x07) << 2) + c;
                        pixels_prio[disp_x as usize] = idx_att >> 7;
                    } else {
                        pixels[disp_x as usize] = self.display.bg_palette[c as usize];
                        #[cfg(feature = "12-colour")]
                        {
                            pixels[disp_x as usize] |= LCD_PALETTE_BG;
                        }
                    }
                    t1 = t1 >> 1;
                    t2 = t2 >> 1;
                }

                disp_x = disp_x.wrapping_sub(1);
            }

            self.display.window_clear += 1;
        }

        if (self.hram_io[IO_LCDC] & LCDC_OBJ_ENABLE) != 0 {
            let mut sprite_number: u8;
            let mut sprites_to_render: [Option<SpriteData>; NUM_SPRITES as usize] =
                [Option::None; NUM_SPRITES as usize];
            #[cfg(feature = "high-lcd-accuracy")]
            {
                let mut number_of_sprites = 0_u8;

                for sprite_number in 0..sprites_to_render.len() {
                    let oy = self.oam[4 * sprite_number as usize + 0];
                    let ox = self.oam[4 * sprite_number as usize + 1];

                    if self.hram_io[IO_LY] + {
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            0
                        } else {
                            8
                        }
                    } >= oy
                        || self.hram_io[IO_LY] + 16 < oy
                    {
                        continue;
                    }

                    sprites_to_render[number_of_sprites as usize] = Some(SpriteData {
                        sprite_number: sprite_number as u8,
                        x: ox,
                    });
                    number_of_sprites += 1;
                }

                if self.cgb.mode == 0 {
                    sprites_to_render.sort_unstable_by(|a, b| compare_sprites(a, b));
                    if number_of_sprites > MAX_SPRITES_LINE {
                        number_of_sprites = MAX_SPRITES_LINE
                    }
                }

                sprite_number = number_of_sprites.wrapping_sub(1);
            }
            #[cfg(not(feature = "high-lcd-accuracy"))]
            {
                drop(sprites_to_render);
                sprite_number = NUM_SPRITES - 1;
            }
            while sprite_number != 0xFF {
                let s: u8;
                #[cfg(feature = "high-lcd-accuracy")]
                {
                    s = sprites_to_render[sprite_number as usize]
                        .unwrap()
                        .sprite_number
                }
                #[cfg(not(feature = "high-lcd-accuracy"))]
                {
                    s = sprite_number;
                }

                let oy = self.oam[4 * s as usize + 0];
                let ox = self.oam[4 * s as usize + 1];
                let ot = self.oam[4 * s as usize + 2]
                    & if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                        0xFE
                    } else {
                        0xFF
                    };
                let of = self.oam[4 * s as usize + 3];

                #[cfg(not(feature = "high-lcd-accuracy"))]
                {
                    if (self.hram_io[IO_LY] + {
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            0
                        } else {
                            8
                        }
                    } >= oy
                        || self.hram_io[IO_LY] + 16 < oy)
                    {
                        continue;
                    }
                }

                if ox == 0 || ox >= 168 {
                    sprite_number = sprite_number.wrapping_sub(1);
                    continue;
                }

                let mut py = self.hram_io[IO_LY].wrapping_sub(oy - 16);

                if of & OBJ_FLIP_Y != 0 {
                    py = ({
                        if self.hram_io[IO_LCDC] & LCDC_OBJ_SIZE != 0 {
                            15
                        } else {
                            7
                        }
                    }) - py;
                }

                let mut t1;
                let mut t2;
                if self.cgb.mode != 0 {
                    t1 = self.vram[VRAM_TILES_1 as usize + (ot as usize * 0x10 + 2 * py as usize)];
                    t2 = self.vram
                        [VRAM_TILES_1 as usize + (ot as usize * 0x10 + 2 * py as usize) + 1];
                } else {
                    t1 = self.vram[VRAM_TILES_1 as usize + ot as usize * 0x10 + 2 * py as usize];
                    t2 =
                        self.vram[VRAM_TILES_1 as usize + ot as usize * 0x10 + 2 * py as usize + 1];
                }

                let shift;
                let dir;
                let start;
                let end;
                if of & OBJ_FLIP_X != 0 {
                    dir = 1;
                    start = {
                        if ox < 8 {
                            0
                        } else {
                            ox - 8
                        }
                    };
                    end = cmp::min(ox, LCD_WIDTH);
                    shift = 8 - (ox - start);
                } else {
                    dir = u8::MAX;
                    start = cmp::min(ox, LCD_WIDTH) - 1;
                    end = ({
                        if ox < 8 {
                            0
                        } else {
                            ox - 8
                        }
                    })
                    .wrapping_sub(1);
                    shift = ox - (start + 1);
                }

                t1 >>= shift;
                t2 >>= shift;

                let mut disp_x = start;
                while disp_x != end {
                    let c: u8 = (t1 & 0x1) | ((t2 & 0x1) << 1);
                    if self.cgb.mode != 0 {
                        let is_background_disabled =
                            c != 0 && (self.hram_io[IO_LCDC] & LCDC_BG_ENABLE) == 0;
                        let is_pixel_priority_non_conflicting = c != 0
                            && !(pixels_prio[disp_x as usize] != 0
                                && (pixels[disp_x as usize] & 0x3) != 0)
                            && !((of & OBJ_PRIORITY) != 0 && (pixels[disp_x as usize] & 0x3) != 0);

                        if is_background_disabled || is_pixel_priority_non_conflicting {
                            pixels[disp_x as usize] = ((of & OBJ_CGB_PALETTE) << 2) + c + 0x20;
                        }
                    } else if c != 0
                        && !((of & OBJ_PRIORITY) != 0
                            && !((pixels[disp_x as usize] & 0x3) == self.display.bg_palette[0]))
                    {
                        pixels[disp_x as usize] = {
                            if (of & OBJ_PALETTE) != 0 {
                                self.display.sp_palette[c as usize + 4]
                            } else {
                                self.display.sp_palette[c as usize]
                            }
                        };

                        #[cfg(feature = "12-colour")]
                        {
                            pixels[disp_x as usize] |= of & OBJ_PALETTE;
                        }
                        pixels[disp_x as usize] &= !LCD_PALETTE_BG
                    }

                    t1 = t1 >> 1;
                    t2 = t2 >> 1;

                    disp_x = disp_x.wrapping_add(dir);
                }

                sprite_number = sprite_number.wrapping_sub(1);
            }
        }

        self.display.lcd_draw_line.unwrap()(self, pixels, self.hram_io[IO_LY]);
    }

    //private this later
    pub fn _step_cpu(&mut self) -> () {
        const OP_CYCLES: [u8; 0x100] = [
            4, 12, 8, 8, 4, 4, 8, 4, 20, 8, 8, 8, 4, 4, 8, 4, 4, 12, 8, 8, 4, 4, 8, 4, 12, 8, 8, 8,
            4, 4, 8, 4, 8, 12, 8, 8, 4, 4, 8, 4, 8, 8, 8, 8, 4, 4, 8, 4, 8, 12, 8, 8, 12, 12, 12,
            4, 8, 8, 8, 8, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4,
            4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 8,
            8, 8, 8, 8, 8, 4, 8, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4,
            8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4,
            4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 4, 4, 4, 4, 4, 4, 8, 4, 8, 12, 12, 16, 12, 16,
            8, 16, 8, 16, 12, 8, 12, 24, 8, 16, 8, 12, 12, 0, 12, 16, 8, 16, 8, 16, 12, 0, 12, 0,
            8, 16, 12, 12, 8, 0, 0, 16, 8, 16, 16, 4, 16, 0, 0, 0, 8, 16, 12, 12, 8, 4, 0, 16, 8,
            16, 12, 8, 16, 4, 0, 0, 8, 16,
        ];
        const TAC_CYCLES: [u16; 4] = [1024, 16, 64, 256];

        while self.gb_halt
            || (self.gb_ime && (self.hram_io[IO_IF] & self.hram_io[IO_IE] & ANY_INTR) != 0)
        {
            self.gb_halt = false;

            if !self.gb_ime {
                break;
            }

            self.gb_ime = false;

            let addr = {
                self.cpu_reg.sp.bytes -= 1;
                self.cpu_reg.sp.bytes as usize
            };
            self._write(addr, self.cpu_reg.pc.get_hi());

            let addr = {
                self.cpu_reg.sp.bytes -= 1;
                self.cpu_reg.sp.bytes as usize
            };
            self._write(addr, self.cpu_reg.pc.get_lo());

            if (self.hram_io[IO_IF] & self.hram_io[IO_IE] & VBLANK_INTR) != 0 {
                self.cpu_reg.pc.bytes = VBLANK_INTR_ADDR as u16;
                self.hram_io[IO_IF] ^= VBLANK_INTR;
            } else if (self.hram_io[IO_IF] & self.hram_io[IO_IE] & LCDC_INTR) != 0 {
                self.cpu_reg.pc.bytes = LCDC_INTR_ADDR as u16;
                self.hram_io[IO_IF] ^= LCDC_INTR;
            } else if (self.hram_io[IO_IF] & self.hram_io[IO_IE] & TIMER_INTR) != 0 {
                self.cpu_reg.pc.bytes = TIMER_INTR_ADDR as u16;
                self.hram_io[IO_IF] ^= TIMER_INTR;
            } else if (self.hram_io[IO_IF] & self.hram_io[IO_IE] & SERIAL_INTR) != 0 {
                self.cpu_reg.pc.bytes = SERIAL_INTR_ADDR as u16;
                self.hram_io[IO_IF] ^= SERIAL_INTR;
            } else if (self.hram_io[IO_IF] & self.hram_io[IO_IE] & CONTROL_INTR) != 0 {
                self.cpu_reg.pc.bytes = CONTROL_INTR_ADDR as u16;
                self.hram_io[IO_IF] ^= CONTROL_INTR;
            }

            break;
        }

        let opcode = self.gb_read_pc();
        let mut inst_cycles = OP_CYCLES[opcode as usize];
        self.cycle += 1;

        match opcode {
            0x00 => {}
            0x01 => {
                let b = self.gb_read_pc();
                self.cpu_reg.bc.set_lo(b);
                let b = self.gb_read_pc();
                self.cpu_reg.bc.set_hi(b);
            }
            0x02 => {
                self._write(self.cpu_reg.bc.bytes as usize, self.cpu_reg.a);
            }
            0x03 => {
                self.cpu_reg.bc.bytes += 1;
            }
            0x04 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.bc.get_hi() + 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.bc.get_hi() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.bc.get_hi() == 0x00)
            }
            0x05 => {
                self.cpu_reg
                    .bc
                    .set_hi(self.cpu_reg.bc.get_hi().wrapping_sub(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.bc.get_hi() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.bc.get_hi() == 0x00)
            }
            0x06 => {
                let b = self.gb_read_pc();
                self.cpu_reg.bc.set_hi(b);
            }
            0x07 => {
                self.cpu_reg.a = (self.cpu_reg.a << 1) | (self.cpu_reg.a >> 7);
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_c(self.cpu_reg.a & 0x01 != 0);
            }
            0x08 => {
                let l = self.gb_read_pc();
                let h = self.gb_read_pc();
                let temp = { (l as u16) | ((h as u16) << 8) } as usize;
                self._write(temp, self.cpu_reg.sp.get_lo());
                let temp = temp + 1;
                self._write(temp, self.cpu_reg.sp.get_hi());
            }
            0x09 => {
                let temp = self.cpu_reg.hl.bytes as u32 + self.cpu_reg.bc.bytes as u32;
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(
                    (temp ^ self.cpu_reg.hl.bytes as u32 ^ self.cpu_reg.bc.bytes as u32) & 0x1000
                        != 0,
                );
                self.cpu_reg.f.set_c((temp & 0xFFFF0000) != 0);
                self.cpu_reg.hl.bytes = temp as u16 & 0x0000FFFF;
            }
            0x0A => {
                self.cpu_reg.a = self._read(self.cpu_reg.bc.bytes as usize);
            }
            0x0B => {
                self.cpu_reg.bc.bytes -= 1;
            }
            0x0C => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.bc.get_lo() + 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.bc.get_lo() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.bc.get_lo() == 0x00)
            }
            0x0D => {
                self.cpu_reg
                    .bc
                    .set_lo(self.cpu_reg.bc.get_lo().wrapping_sub(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.bc.get_lo() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.bc.get_lo() == 0x00)
            }
            0x0E => {
                let b = self.gb_read_pc();
                self.cpu_reg.bc.set_lo(b);
            }
            0x0F => {
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_c(self.cpu_reg.a & 0x01 != 0);
                self.cpu_reg.a = (self.cpu_reg.a >> 1) | (self.cpu_reg.a << 7);
            }
            0x10 => {
                // self.gb_halt = true;
                #[cfg(feature = "gbc")]
                {
                    if (self.cgb.mode & self.cgb.double_speed_prep) != 0 {
                        self.cgb.double_speed_prep = 0;
                        self.cgb.double_speed ^= 1;
                    }
                }
            }
            0x11 => {
                let b = self.gb_read_pc();
                self.cpu_reg.de.set_lo(b);
                let b = self.gb_read_pc();
                self.cpu_reg.de.set_hi(b);
            }
            0x12 => {
                self._write(self.cpu_reg.de.bytes as usize, self.cpu_reg.a);
            }
            0x13 => {
                self.cpu_reg.de.bytes += 1;
            }
            0x14 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.de.get_hi() + 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_hi() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_hi() == 0x00)
            }
            0x15 => {
                self.cpu_reg
                    .de
                    .set_hi(self.cpu_reg.de.get_hi().wrapping_sub(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_hi() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_hi() == 0x00)
            }
            0x16 => {
                let b = self.gb_read_pc();
                self.cpu_reg.de.set_hi(b)
            }
            0x17 => {
                let temp = self.cpu_reg.a;
                self.cpu_reg.a = (self.cpu_reg.a << 1) | self.cpu_reg.f.get_c();
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_c((temp >> 7) & 0x01 != 0);
            }
            0x18 => {
                let temp = (self.gb_read_pc() as i8) as i16; //peformance?
                self.cpu_reg.pc.bytes = (self.cpu_reg.pc.bytes as i16 + temp) as u16
            }
            0x19 => {
                let temp = self.cpu_reg.hl.bytes as u32 + self.cpu_reg.de.bytes as u32;
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(
                    (temp as u16 ^ self.cpu_reg.hl.bytes ^ self.cpu_reg.de.bytes) & 0x1000 != 0,
                );
                self.cpu_reg.f.set_c((temp & 0xFFFF0000) != 0);
                self.cpu_reg.hl.bytes = temp as u16 & 0x0000FFFF;
            }
            0x1A => {
                self.cpu_reg.a = self._read(self.cpu_reg.de.bytes as usize);
            }
            0x1B => {
                self.cpu_reg.de.bytes -= 1;
            }
            0x1C => {
                self.cpu_reg.de.set_lo(self.cpu_reg.de.get_lo() + 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_lo() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_lo() == 0x00)
            }
            0x1D => {
                self.cpu_reg.de.set_lo(self.cpu_reg.de.get_lo() - 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_lo() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_lo() == 0x00)
            }
            0x1E => {
                let b = self.gb_read_pc();
                self.cpu_reg.de.set_lo(b);
            }
            0x1F => {
                let temp = self.cpu_reg.a;
                self.cpu_reg.a = self.cpu_reg.a >> 1 | (self.cpu_reg.f.get_c() << 7);
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_c(temp & 0x1 != 0);
            }
            0x20 => {
                if self.cpu_reg.f.get_z() == 0 {
                    let temp = (self.gb_read_pc() as i8) as i16;
                    self.cpu_reg.pc.bytes = (self.cpu_reg.pc.bytes as i16 + temp) as u16;
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 1;
                }
            }
            0x21 => {
                let b = self.gb_read_pc();
                self.cpu_reg.hl.set_lo(b);
                let b = self.gb_read_pc();
                self.cpu_reg.hl.set_hi(b);
            }
            0x22 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.a);
                self.cpu_reg.hl.bytes += 1;
            }
            0x23 => {
                self.cpu_reg.hl.bytes += 1;
            }
            0x24 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.de.get_hi() + 1);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_hi() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_hi() == 0x00)
            }
            0x25 => {
                self.cpu_reg
                    .de
                    .set_hi(self.cpu_reg.de.get_hi().wrapping_sub(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.de.get_hi() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.de.get_hi() == 0x00)
            }
            0x26 => {
                let b = self.gb_read_pc();
                self.cpu_reg.hl.set_hi(b);
            }
            0x27 => {
                let mut a = self.cpu_reg.a as i16;

                if self.cpu_reg.f.get_n() != 0 {
                    if self.cpu_reg.f.get_h() != 0 {
                        a = (a - 0x06) & 0xFF;
                    }

                    if self.cpu_reg.f.get_c() != 0 {
                        a -= 0x60;
                    }
                } else {
                    if self.cpu_reg.f.get_h() != 0 || (a & 0x0F) > 9 {
                        a += 0x06;
                    }

                    if self.cpu_reg.f.get_c() != 0 || a > 0x9F {
                        a += 0x60;
                    }
                }

                if (a & 0x100) == 0x100 {
                    self.cpu_reg.f.set_c(true);
                };

                self.cpu_reg.a = a as u8;
                self.cpu_reg.f.set_z(self.cpu_reg.a == 0);
                self.cpu_reg.f.set_h(false);
            }
            0x28 => {
                if self.cpu_reg.f.get_z() != 0 {
                    let temp = (self.gb_read_pc() as i8) as i16; //performance?
                    self.cpu_reg.pc.bytes = (self.cpu_reg.pc.bytes as i16 + temp) as u16;
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 1;
                }
            }
            0x29 => {
                self.cpu_reg.f.set_c((self.cpu_reg.hl.bytes & 0x8000) > 0);
                self.cpu_reg.hl.bytes <<= 1;
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h((self.cpu_reg.hl.bytes & 0x1000) > 0);
            }
            0x2A => {
                let hl = self.cpu_reg.hl.bytes as usize;
                self.cpu_reg.hl.bytes += 1;
                self.cpu_reg.a = self._read(hl);
            }
            0x2B => {
                self.cpu_reg.hl.bytes -= 1;
            }
            0x2C => {
                self.cpu_reg
                    .hl
                    .set_lo(self.cpu_reg.hl.get_lo().wrapping_add(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.hl.get_lo() & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.hl.get_lo() == 0x00)
            }
            0x2D => {
                self.cpu_reg
                    .hl
                    .set_lo(self.cpu_reg.hl.get_lo().wrapping_sub(1));
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.hl.get_lo() & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.hl.get_lo() == 0x00)
            }
            0x2E => {
                let b = self.gb_read_pc();
                self.cpu_reg.hl.set_lo(b);
            }
            0x2F => {
                self.cpu_reg.a = !self.cpu_reg.a;
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_h(true);
            }
            0x30 => {
                if self.cpu_reg.f.get_c() == 0 {
                    let temp = self.gb_read_pc() as i16;
                    self.cpu_reg.pc.bytes = (self.cpu_reg.pc.bytes as i16 + temp) as u16;
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 1
                };
            }
            0x31 => {
                let b = self.gb_read_pc();
                self.cpu_reg.sp.set_lo(b);
                let b = self.gb_read_pc();
                self.cpu_reg.sp.set_hi(b);
            }
            0x32 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.a);
                self.cpu_reg.hl.bytes -= 1;
            }
            0x33 => {
                self.cpu_reg.sp.bytes += 1;
            }
            0x34 => {
                let temp = self._read(self.cpu_reg.hl.bytes as usize).wrapping_add(1);
                self.cpu_reg.f.set_h((temp & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(temp == 0x00);
                self._write(self.cpu_reg.hl.bytes as usize, temp);
            }
            0x35 => {
                let temp = self._read(self.cpu_reg.hl.bytes as usize).wrapping_sub(1);
                self.cpu_reg.f.set_h((temp & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(temp == 0x00);
                self._write(self.cpu_reg.hl.bytes as usize, temp);
            }
            0x36 => {
                let b = self.gb_read_pc();
                self._write(self.cpu_reg.hl.bytes as usize, b);
            }
            0x37 => {
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(false);
                self.cpu_reg.f.set_c(false);
            }
            0x38 => {
                if self.cpu_reg.f.get_c() != 0 {
                    let temp = (self.gb_read_pc() as i8) as i16;
                    self.cpu_reg.pc.bytes = (self.cpu_reg.pc.bytes as i16 + temp) as u16;
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 1;
                }
            }
            0x39 => {
                let temp = self.cpu_reg.hl.bytes as u32 + self.cpu_reg.sp.bytes as u32;
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(
                    ((self.cpu_reg.hl.bytes & 0xFFF) + (self.cpu_reg.sp.bytes & 0xFFF)) & 0x1000
                        != 0,
                );
                self.cpu_reg.f.set_c(temp & 0x10000 != 0);
                self.cpu_reg.hl.bytes = temp as u16;
            }
            0x3A => {
                let hl = self.cpu_reg.hl.bytes as usize;
                self.cpu_reg.hl.bytes -= 1;
                self.cpu_reg.a = self._read(hl);
            }
            0x3B => {
                self.cpu_reg.sp.bytes -= 1;
            }
            0x3C => {
                self.cpu_reg.a = self.cpu_reg.a.wrapping_add(1);
                self.cpu_reg.f.set_h((self.cpu_reg.a & 0x0F) == 0x00);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_z(self.cpu_reg.a == 0x00)
            }
            0x3D => {
                self.cpu_reg.a = self.cpu_reg.a.wrapping_sub(1);
                self.cpu_reg.f.set_h((self.cpu_reg.a & 0x0F) == 0x0F);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg.f.set_z(self.cpu_reg.a == 0x00)
            }
            0x3E => {
                self.cpu_reg.a = self.gb_read_pc();
            }
            0x3F => {
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(false);
                self.cpu_reg.f.set_c(self.cpu_reg.f.get_c() == 0);
            }
            0x40 => {}
            0x41 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.bc.get_lo());
            }
            0x42 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.de.get_hi());
            }
            0x43 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.de.get_lo());
            }
            0x44 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.hl.get_hi());
            }
            0x45 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.hl.get_lo());
            }
            0x46 => {
                self.cpu_reg
                    .bc
                    .set_hi(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x47 => {
                self.cpu_reg.bc.set_hi(self.cpu_reg.a);
            }
            0x48 => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.bc.get_hi());
            }
            0x49 => {}
            0x4A => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.de.get_hi());
            }
            0x4B => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.de.get_lo());
            }
            0x4C => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.hl.get_hi());
            }
            0x4D => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.hl.get_lo());
            }
            0x4E => {
                self.cpu_reg
                    .bc
                    .set_lo(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x4F => {
                self.cpu_reg.bc.set_lo(self.cpu_reg.a);
            }
            0x50 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.bc.get_hi());
            }
            0x51 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.bc.get_lo());
            }
            0x52 => {}
            0x53 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.de.get_lo());
            }
            0x54 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.hl.get_hi());
            }
            0x55 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.hl.get_lo());
            }
            0x56 => {
                self.cpu_reg
                    .de
                    .set_hi(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x57 => {
                self.cpu_reg.de.set_hi(self.cpu_reg.a);
            }
            0x58 => {
                self.cpu_reg.de.set_lo(self.cpu_reg.bc.get_hi());
            }
            0x59 => {
                self.cpu_reg.de.set_lo(self.cpu_reg.bc.get_lo());
            }
            0x5A => {
                self.cpu_reg.de.set_lo(self.cpu_reg.de.get_hi());
            }
            0x5B => {}
            0x5C => {
                self.cpu_reg.de.set_lo(self.cpu_reg.hl.get_hi());
            }
            0x5D => {
                self.cpu_reg.de.set_lo(self.cpu_reg.hl.get_lo());
            }
            0x5E => {
                self.cpu_reg
                    .de
                    .set_lo(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x5F => {
                self.cpu_reg.de.set_lo(self.cpu_reg.a);
            }
            0x60 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.bc.get_hi());
            }
            0x61 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.bc.get_lo());
            }
            0x62 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.de.get_hi());
            }
            0x63 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.de.get_lo());
            }
            0x64 => {}
            0x65 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.hl.get_lo());
            }
            0x66 => {
                self.cpu_reg
                    .hl
                    .set_hi(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x67 => {
                self.cpu_reg.hl.set_hi(self.cpu_reg.a);
            }
            0x68 => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.bc.get_hi());
            }
            0x69 => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.bc.get_lo());
            }
            0x6A => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.de.get_hi());
            }
            0x6B => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.de.get_lo());
            }
            0x6C => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.hl.get_hi());
            }
            0x6D => {}
            0x6E => {
                self.cpu_reg
                    .hl
                    .set_lo(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0x6F => {
                self.cpu_reg.hl.set_lo(self.cpu_reg.a);
            }
            0x70 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.bc.get_hi());
            }
            0x71 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.bc.get_lo());
            }
            0x72 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.de.get_hi());
            }
            0x73 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.de.get_lo());
            }
            0x74 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.hl.get_hi());
            }
            0x75 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.hl.get_lo());
            }
            0x76 => {
                let mut halt_cycles = i16::MAX;

                self.gb_halt = true;

                if self.hram_io[IO_IE] == 0 {
                    self.gb_error.unwrap()(self, GbError::GbHaltForever, self.cpu_reg.pc.bytes - 1);
                    panic!();
                }

                if (self.hram_io[IO_SC] & SERIAL_SC_TX_START) != 0 {
                    let serial_cycles = SERIAL_CYCLES - self.counter.serial_count;

                    if (serial_cycles as i16) < halt_cycles {
                        halt_cycles = serial_cycles as i16;
                    }
                }

                if (self.hram_io[IO_TAC] & IO_TAC_ENABLE_MASK) != 0 {
                    let tac_cycles = TAC_CYCLES[(self.hram_io[IO_TAC] & IO_TAC_RATE_MASK) as usize]
                        - self.counter.tima_count;

                    if (tac_cycles as i16) < halt_cycles {
                        halt_cycles = tac_cycles as i16;
                    }
                }

                if (self.hram_io[IO_LCDC] & LCDC_ENABLE) != 0 {
                    let lcd_cycles;
                    if (self.hram_io[IO_STAT] & STAT_MODE) == IO_STAT_MODE_HBLANK {
                        lcd_cycles = LCD_MODE_2_CYCLES - self.counter.lcd_count;
                    } else if (self.hram_io[IO_STAT] & STAT_MODE) == IO_STAT_MODE_SEARCH_OAM {
                        lcd_cycles = LCD_MODE_3_CYCLES - self.counter.lcd_count;
                    } else if (self.hram_io[IO_STAT] & STAT_MODE) == IO_STAT_MODE_SEARCH_TRANSFER {
                        lcd_cycles = LCD_MODE_0_CYCLES.wrapping_sub(self.counter.lcd_count);
                    } else {
                        lcd_cycles = LCD_LINE_CYCLES - self.counter.lcd_count;
                    }

                    if (lcd_cycles as i16) < halt_cycles {
                        halt_cycles = lcd_cycles as i16;
                    }
                }

                if halt_cycles <= 0 {
                    halt_cycles = 4;
                }

                inst_cycles = halt_cycles as u8;
            }
            0x77 => {
                self._write(self.cpu_reg.hl.bytes as usize, self.cpu_reg.a);
            }
            0x78 => {
                self.cpu_reg.a = self.cpu_reg.bc.get_hi();
            }
            0x79 => {
                self.cpu_reg.a = self.cpu_reg.bc.get_lo();
            }
            0x7A => {
                self.cpu_reg.a = self.cpu_reg.de.get_hi();
            }
            0x7B => {
                self.cpu_reg.a = self.cpu_reg.de.get_lo();
            }

            0x7C => {
                self.cpu_reg.a = self.cpu_reg.hl.get_hi();
            }
            0x7D => {
                self.cpu_reg.a = self.cpu_reg.hl.get_lo();
            }
            0x7E => {
                self.cpu_reg.a = self._read(self.cpu_reg.hl.bytes as usize);
            }
            0x7F => {}
            0x80 => {
                self._adc(self.cpu_reg.bc.get_hi(), 0);
            }
            0x81 => {
                self._adc(self.cpu_reg.bc.get_lo(), 0);
            }
            0x82 => {
                self._adc(self.cpu_reg.de.get_hi(), 0);
            }
            0x83 => {
                self._adc(self.cpu_reg.de.get_lo(), 0);
            }
            0x84 => {
                self._adc(self.cpu_reg.hl.get_hi(), 0);
            }
            0x85 => {
                self._adc(self.cpu_reg.hl.get_lo(), 0);
            }
            0x86 => {
                self._adc_hl(0);
            }
            0x87 => {
                self._adc(self.cpu_reg.a, 0);
            }
            0x88 => {
                self._adc(self.cpu_reg.bc.get_hi(), self.cpu_reg.f.get_c());
            }
            0x89 => {
                self._adc(self.cpu_reg.bc.get_lo(), self.cpu_reg.f.get_c());
            }
            0x8A => {
                self._adc(self.cpu_reg.de.get_hi(), self.cpu_reg.f.get_c());
            }
            0x8B => {
                self._adc(self.cpu_reg.de.get_lo(), self.cpu_reg.f.get_c());
            }
            0x8C => {
                self._adc(self.cpu_reg.hl.get_hi(), self.cpu_reg.f.get_c());
            }
            0x8D => {
                self._adc(self.cpu_reg.hl.get_lo(), self.cpu_reg.f.get_c());
            }
            0x8E => {
                self._adc_hl(self.cpu_reg.f.get_c());
            }
            0x8F => {
                self._adc(self.cpu_reg.a, self.cpu_reg.f.get_c());
            }
            0x90 => {
                self._sbc(self.cpu_reg.bc.get_hi(), 0);
            }
            0x91 => {
                self._sbc(self.cpu_reg.bc.get_lo(), 0);
            }
            0x92 => {
                self._sbc(self.cpu_reg.de.get_hi(), 0);
            }
            0x93 => {
                self._sbc(self.cpu_reg.de.get_lo(), 0);
            }
            0x94 => {
                self._sbc(self.cpu_reg.hl.get_hi(), 0);
            }
            0x95 => {
                self._sbc(self.cpu_reg.hl.get_lo(), 0);
            }
            0x96 => {
                self._sbc_hl(0);
            }
            0x97 => {
                self.cpu_reg.a = 0;
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_z(false);
                self.cpu_reg.f.set_n(false);
            }
            0x98 => {
                self._sbc(self.cpu_reg.bc.get_hi(), self.cpu_reg.f.get_c());
            }
            0x99 => {
                self._sbc(self.cpu_reg.bc.get_lo(), self.cpu_reg.f.get_c());
            }
            0x9A => {
                self._sbc(self.cpu_reg.de.get_hi(), self.cpu_reg.f.get_c());
            }
            0x9B => {
                self._sbc(self.cpu_reg.de.get_lo(), self.cpu_reg.f.get_c());
            }
            0x9C => {
                self._sbc(self.cpu_reg.hl.get_hi(), self.cpu_reg.f.get_c());
            }
            0x9D => {
                self._sbc(self.cpu_reg.hl.get_lo(), self.cpu_reg.f.get_c());
            }
            0x9E => {
                self._sbc_hl(self.cpu_reg.f.get_c());
            }
            0x9F => {
                self.cpu_reg.a = {
                    if self.cpu_reg.f.get_c() != 0 {
                        0xFF
                    } else {
                        0x00
                    }
                };
                self.cpu_reg.f.set_z(self.cpu_reg.f.get_c() == 0);
                self.cpu_reg.f.set_n(false);
                self.cpu_reg.f.set_h(self.cpu_reg.f.get_c() != 0);
            }
            0xA0 => {
                self._and(self.cpu_reg.bc.get_hi());
            }
            0xA1 => {
                self._and(self.cpu_reg.bc.get_lo());
            }
            0xA2 => {
                self._and(self.cpu_reg.de.get_hi());
            }
            0xA3 => {
                self._and(self.cpu_reg.de.get_lo());
            }
            0xA4 => {
                self._and(self.cpu_reg.hl.get_hi());
            }
            0xA5 => {
                self._and(self.cpu_reg.hl.get_lo());
            }
            0xA6 => {
                self._and(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0xA7 => {
                self._and(self.cpu_reg.a);
            }
            0xA8 => {
                self._xor(self.cpu_reg.bc.get_hi());
            }
            0xA9 => {
                self._xor(self.cpu_reg.bc.get_lo());
            }
            0xAA => {
                self._xor(self.cpu_reg.de.get_hi());
            }
            0xAB => {
                self._xor(self.cpu_reg.de.get_lo());
            }
            0xAC => {
                self._xor(self.cpu_reg.hl.get_hi());
            }
            0xAD => {
                self._xor(self.cpu_reg.hl.get_lo());
            }
            0xAE => {
                self._xor(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0xAF => {
                self._xor(self.cpu_reg.a);
            }
            0xB0 => {
                self._or(self.cpu_reg.bc.get_hi());
            }
            0xB1 => {
                self._or(self.cpu_reg.bc.get_lo());
            }
            0xB2 => {
                self._or(self.cpu_reg.de.get_hi());
            }
            0xB3 => {
                self._or(self.cpu_reg.de.get_lo());
            }
            0xB4 => {
                self._or(self.cpu_reg.hl.get_hi());
            }
            0xB5 => {
                self._or(self.cpu_reg.hl.get_lo());
            }
            0xB6 => {
                self._or(self._read(self.cpu_reg.hl.bytes as usize));
            }
            0xB7 => {
                self._or(self.cpu_reg.a);
            }
            0xB8 => {
                self._cp(self.cpu_reg.bc.get_hi());
            }
            0xB9 => {
                self._cp(self.cpu_reg.bc.get_lo());
            }
            0xBA => {
                self._cp(self.cpu_reg.de.get_hi());
            }
            0xBB => {
                self._cp(self.cpu_reg.de.get_lo());
            }
            0xBC => {
                self._cp(self.cpu_reg.hl.get_hi());
            }
            0xBD => {
                self._cp(self.cpu_reg.hl.get_lo());
            }
            0xBE => {
                self._cp_hl();
            }
            0xBF => {
                self.cpu_reg.f.byte = 0;
                self.cpu_reg.f.set_z(false);
                self.cpu_reg.f.set_n(false);
            }
            0xC0 => {
                if self.cpu_reg.f.get_z() == 0 {
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_lo(b);
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_hi(b);
                    inst_cycles += 12;
                }
            }
            0xC1 => {
                let b = self.gb_read_sp();
                self.cpu_reg.bc.set_lo(b);
                let b = self.gb_read_sp();
                self.cpu_reg.bc.set_hi(b);
            }
            0xC2 => {
                if self.cpu_reg.f.get_z() == 0 {
                    let c = self.gb_read_pc();
                    let p = self._read(self.cpu_reg.pc.bytes as usize);
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xC3 => {
                let c = self.gb_read_pc();
                let p = self._read(self.cpu_reg.pc.bytes as usize);
                self.cpu_reg.pc.set_lo(c);
                self.cpu_reg.pc.set_hi(p);
            }
            0xC4 => {
                if self.cpu_reg.f.get_z() == 0 {
                    let c = self.gb_read_pc();
                    let p = self.gb_read_pc();
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 12;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xC5 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.bc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.bc.get_lo());
            }
            0xC6 => {
                let val = self.gb_read_pc();
                self._adc(val, 0);
            }
            0xC7 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0000;
            }
            0xC8 => {
                if self.cpu_reg.f.get_z() != 0 {
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_lo(b);
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_hi(b);
                    inst_cycles += 12;
                }
            }
            0xC9 => {
                let b = self.gb_read_sp();
                self.cpu_reg.pc.set_lo(b);
                let b = self.gb_read_sp();
                self.cpu_reg.pc.set_hi(b);
            }
            0xCA => {
                if self.cpu_reg.f.get_z() != 0 {
                    let c = self.gb_read_pc();
                    let p = self._read(self.cpu_reg.pc.bytes as usize);
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xCB => {
                inst_cycles = self._execute_cb();
            }
            0xCC => {
                if self.cpu_reg.f.get_z() != 0 {
                    let c = self.gb_read_pc();
                    let p = self.gb_read_pc();
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 12;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xCD => {
                let c = self.gb_read_pc();
                let p = self.gb_read_pc();
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.set_lo(c);
                self.cpu_reg.pc.set_hi(p);
            }
            0xCE => {
                let b = self.gb_read_pc();
                self._adc(b, self.cpu_reg.f.get_c());
            }
            0xCF => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0008;
            }
            0xD0 => {
                if self.cpu_reg.f.get_c() == 0 {
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_lo(b);
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_hi(b);
                    inst_cycles += 12;
                }
            }
            0xD1 => {
                let b = self.gb_read_sp();
                self.cpu_reg.de.set_lo(b);
                let b = self.gb_read_sp();
                self.cpu_reg.de.set_hi(b);
            }
            0xD2 => {
                if self.cpu_reg.f.get_c() == 0 {
                    let c = self.gb_read_pc();
                    let p = self._read(self.cpu_reg.pc.bytes as usize);
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xD4 => {
                if self.cpu_reg.f.get_c() == 0 {
                    let c = self.gb_read_pc();
                    let p = self.gb_read_pc();
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 12;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xD5 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.de.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.de.get_lo());
            }
            0xD6 => {
                let val = self.gb_read_pc();
                let temp = (self.cpu_reg.a as i16 - val as i16) as u16;
                self.cpu_reg.f.set_z((temp & 0xFF) == 0x00);
                self.cpu_reg.f.set_n(true);
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.a as u16 ^ val as u16 ^ temp) & 0x10 != 0);
                self.cpu_reg.f.set_c((temp & 0xFF00) != 0);
                self.cpu_reg.a = (temp & 0xFF) as u8;
            }
            0xD7 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0010;
            }
            0xD8 => {
                if self.cpu_reg.f.get_c() != 0 {
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_lo(b);
                    let b = self.gb_read_sp();
                    self.cpu_reg.pc.set_hi(b);
                    inst_cycles += 12;
                }
            }
            0xD9 => {
                let b = self.gb_read_sp();
                self.cpu_reg.pc.set_lo(b);
                let b = self.gb_read_sp();
                self.cpu_reg.pc.set_hi(b);
                self.gb_ime = true;
            }
            0xDA => {
                if self.cpu_reg.f.get_c() != 0 {
                    let c = self.gb_read_pc();
                    let p = self._read(self.cpu_reg.pc.bytes as usize);
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 4;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xDC => {
                if self.cpu_reg.f.get_c() != 0 {
                    let c = self.gb_read_pc();
                    let p = self.gb_read_pc();
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                    self.cpu_reg.sp.bytes -= 1;
                    self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                    self.cpu_reg.pc.set_lo(c);
                    self.cpu_reg.pc.set_hi(p);
                    inst_cycles += 12;
                } else {
                    self.cpu_reg.pc.bytes += 2;
                }
            }
            0xDE => {
                let val = self.gb_read_pc();
                self._sbc(val, self.cpu_reg.f.get_c());
            }
            0xDF => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0018;
            }
            0xE0 => {
                let b = self.gb_read_pc();
                self._write(0xFF00 | b as usize, self.cpu_reg.a);
            }
            0xE1 => {
                let b = self.gb_read_sp();
                self.cpu_reg.hl.set_lo(b);
                let b = self.gb_read_sp();
                self.cpu_reg.hl.set_hi(b);
            }
            0xE2 => {
                self._write(0xFF00 | self.cpu_reg.bc.get_lo() as usize, self.cpu_reg.a);
            }
            0xE5 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.hl.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.hl.get_lo());
            }
            0xE6 => {
                let temp = self.gb_read_pc();
                self._and(temp);
            }
            0xE7 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0020;
            }
            0xE8 => {
                let offset = self.gb_read_pc() as i8;
                self.cpu_reg.f.byte = 0;
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.sp.bytes as i16 & 0xF) + (offset as i16 & 0xF) > 0xF);
                self.cpu_reg
                    .f
                    .set_c((self.cpu_reg.sp.bytes as i16 & 0xFF) + (offset as i16 & 0xFF) > 0xFF);
                self.cpu_reg.sp.bytes += offset as u16;
            }
            0xE9 => {
                self.cpu_reg.pc.bytes = self.cpu_reg.hl.bytes;
            }
            0xEA => {
                let l = self.gb_read_pc();
                let h = self.gb_read_pc();
                let addr = { l as usize | ((h as usize) << 8) };
                self._write(addr, self.cpu_reg.a);
            }
            0xEE => {
                let b = self.gb_read_pc();
                self._xor(b);
            }
            0xEF => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0028;
            }
            0xF0 => {
                let b = self.gb_read_pc();
                self.cpu_reg.a = self._read(0xFF00 | b as usize);
            }
            0xF1 => {
                let temp_8 = self.gb_read_sp();
                self.cpu_reg.f.set_z((temp_8 >> 7) & 1 != 0);
                self.cpu_reg.f.set_n((temp_8 >> 6) & 1 != 0);
                self.cpu_reg.f.set_h((temp_8 >> 5) & 1 != 0);
                self.cpu_reg.f.set_c((temp_8 >> 4) & 1 != 0);
                self.cpu_reg.a = self.gb_read_sp();
            }
            0xF2 => {
                self.cpu_reg.a = self._read(0xFF00 | self.cpu_reg.bc.get_lo() as usize);
            }
            0xF3 => {
                self.gb_ime = false;
            }
            0xF5 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.a);
                self.cpu_reg.sp.bytes -= 1;
                self._write(
                    self.cpu_reg.sp.bytes as usize,
                    self.cpu_reg.f.get_z() << 7
                        | self.cpu_reg.f.get_n() << 6
                        | self.cpu_reg.f.get_h() << 5
                        | self.cpu_reg.f.get_c() << 4,
                );
            }
            0xF6 => {
                let b = self.gb_read_pc();
                self._or(b);
            }
            0xF7 => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0030;
            }
            0xF8 => {
                let offset = self.gb_read_pc() as i16;
                self.cpu_reg.hl.bytes = (self.cpu_reg.sp.bytes as i16 + offset) as u16;
                self.cpu_reg.f.byte = 0;
                self.cpu_reg
                    .f
                    .set_h((self.cpu_reg.sp.bytes as i16 & 0xF) + (offset & 0xF) > 0xF);
                self.cpu_reg
                    .f
                    .set_c((self.cpu_reg.sp.bytes as i16 & 0xFF) + (offset & 0xFF) > 0xFF);
            }
            0xF9 => {
                self.cpu_reg.sp.bytes = self.cpu_reg.hl.bytes;
            }
            0xFA => {
                let l = self.gb_read_pc();
                let h = self.gb_read_pc();
                let addr = { l as usize | ((h as usize) << 8) };
                self.cpu_reg.a = self._read(addr);
            }
            0xFB => {
                self.gb_ime = true;
            }
            0xFE => {
                let b = self.gb_read_pc();
                self._cp(b);
            }
            0xFF => {
                self.cpu_reg.sp.bytes -= 1;
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_hi());
                self.cpu_reg.sp.bytes = self.cpu_reg.sp.bytes.wrapping_sub(1);
                self._write(self.cpu_reg.sp.bytes as usize, self.cpu_reg.pc.get_lo());
                self.cpu_reg.pc.bytes = 0x0038;
            }
            _ => {
                self.gb_error.unwrap()(self, GbError::GbInvalidOpcode, self.cpu_reg.pc.bytes - 1);
            }
        }

        let mut do_while_condition = true;
        while do_while_condition {
            self.counter.div_count += inst_cycles as u16;
            while self.counter.div_count >= DIV_CYCLES {
                self.hram_io[IO_DIV] = self.hram_io[IO_DIV].wrapping_add(1);
                self.counter.div_count -= DIV_CYCLES;
            }

            if self.mbc == 3 && (self.rtc_real.get_high() & 0x40) == 0 {
                self.counter.rtc_count += inst_cycles as u32;
                while self.counter.rtc_count >= RTC_CYCLES {
                    self.counter.rtc_count -= RTC_CYCLES;

                    if self.rtc_real.get_sec() == 63 {
                        self.rtc_real.set_sec(0);
                        continue;
                    }

                    if ({
                        self.rtc_real.set_sec(self.rtc_real.get_sec() + 1);
                        self.rtc_real.get_sec()
                    } != 60)
                    {
                        continue;
                    }

                    self.rtc_real.set_sec(0);
                    if self.rtc_real.get_min() == 63 {
                        self.rtc_real.set_min(0);
                        continue;
                    }
                    if ({
                        self.rtc_real.set_min(self.rtc_real.get_min() + 1);
                        self.rtc_real.get_min()
                    } != 60)
                    {
                        continue;
                    }

                    self.rtc_real.set_min(0);
                    if self.rtc_real.get_hour() == 31 {
                        self.rtc_real.set_hour(0);
                        continue;
                    }
                    if ({
                        self.rtc_real.set_hour(self.rtc_real.get_hour() + 1);
                        self.rtc_real.get_hour()
                    } != 24)
                    {
                        continue;
                    }

                    self.rtc_real.set_hour(0);
                    if ({
                        self.rtc_real.set_yday(self.rtc_real.get_yday() + 1);
                        self.rtc_real.get_yday()
                    } != 0)
                    {
                        continue;
                    }

                    if (self.rtc_real.get_high() & 1) != 0 {
                        self.rtc_real.set_high(self.rtc_real.get_high() | 0x80);
                    }

                    self.rtc_real.set_high(self.rtc_real.get_high() ^ 1);
                }
            }

            if (self.hram_io[IO_SC] & SERIAL_SC_TX_START) != 0 {
                let mut serial_cycles = SERIAL_CYCLES_1KB;

                if self.counter.serial_count == 0 && self.gb_serial_tx.is_some() {
                    self.gb_serial_tx.unwrap()(self, self.hram_io[IO_SB])
                };

                #[cfg(feature = "gbc")]
                {
                    if self.hram_io[IO_SC] & 0x3 != 0 {
                        serial_cycles = SERIAL_CYCLES_32KB;
                    }
                }

                self.counter.serial_count += inst_cycles as u16;

                if self.counter.serial_count >= serial_cycles {
                    let mut rx: u8 = 0;
                    if self.gb_serial_rx.is_some_and(|frx| {
                        matches!(frx(self, &mut rx), GbSerialRxRet::GbSerialRxSuccess)
                    }) {
                        self.hram_io[IO_SB] = rx;

                        self.hram_io[IO_SC] &= 0x01;
                        self.hram_io[IO_IF] |= SERIAL_INTR;
                    } else if (self.hram_io[IO_SC] & SERIAL_SC_CLOCK_SRC) != 0 {
                        self.hram_io[IO_SB] = 0xFF;

                        self.hram_io[IO_SC] &= 0x01;
                        self.hram_io[IO_IF] |= SERIAL_INTR;
                    } else {
                        /* If using external clock, and console is not
                         * attached to any external peripheral, bits are
                         * not shifted, so SB is not modified. */
                    }

                    self.counter.serial_count = 0;
                }
            }

            if (self.hram_io[IO_TAC] & IO_TAC_ENABLE_MASK) != 0 {
                self.counter.tima_count += inst_cycles as u16;

                while self.counter.tima_count
                    >= TAC_CYCLES[(self.hram_io[IO_TAC] & IO_TAC_RATE_MASK) as usize]
                {
                    self.counter.tima_count -=
                        TAC_CYCLES[(self.hram_io[IO_TAC] & IO_TAC_RATE_MASK) as usize];

                    if ({
                        self.hram_io[IO_TIMA] = self.hram_io[IO_TIMA].checked_add(1).unwrap_or(0);
                        self.hram_io[IO_TIMA]
                    } == 0)
                    {
                        self.hram_io[IO_IF] |= TIMER_INTR;
                        self.hram_io[IO_TIMA] = self.hram_io[IO_TMA];
                    }
                }
            }

            if (self.hram_io[IO_LCDC] & LCDC_ENABLE) == 0 {
                do_while_condition =
                    self.gb_halt && (self.hram_io[IO_IF] & self.hram_io[IO_IE]) == 0;
                continue;
            }

            #[cfg(feature = "gbc")]
            {
                if inst_cycles > 1 {
                    self.counter.lcd_count += (inst_cycles as u16) >> self.cgb.double_speed;
                } else {
                    self.counter.lcd_count += inst_cycles as u16;
                }
            }
            #[cfg(not(feature = "gbc"))]
            {
                self.counter.lcd_count += inst_cycles as u16;
            }

            if self.counter.lcd_count >= LCD_LINE_CYCLES {
                self.counter.lcd_count -= LCD_LINE_CYCLES;

                self.hram_io[IO_LY] = (self.hram_io[IO_LY] + 1) % LCD_VERT_LINES;

                if self.hram_io[IO_LY] == self.hram_io[IO_LYC] {
                    self.hram_io[IO_STAT] |= STAT_LYC_COINC;

                    if (self.hram_io[IO_STAT] & STAT_LYC_INTR) != 0 {
                        self.hram_io[IO_IF] |= LCDC_INTR;
                    }
                } else {
                    self.hram_io[IO_STAT] &= 0xFB;
                }

                if self.hram_io[IO_LY] == LCD_HEIGHT {
                    self.hram_io[IO_STAT] =
                        (self.hram_io[IO_STAT] & !STAT_MODE) | IO_STAT_MODE_VBLANK;
                    self.gb_frame = true;
                    self.hram_io[IO_IF] |= VBLANK_INTR;
                    self.lcd_blank = false;

                    if (self.hram_io[IO_STAT] & STAT_MODE_1_INTR) != 0 {
                        self.hram_io[IO_IF] |= LCDC_INTR;
                    }

                    #[cfg(feature = "lcd")]
                    {
                        if self.direct.frame_skip {
                            self.display.frame_skip_count = !self.display.frame_skip_count;
                        }

                        if self.direct.interlace
                            && (!self.direct.frame_skip || self.display.frame_skip_count)
                        {
                            self.display.interlace_count = !self.display.interlace_count;
                        }
                    }
                } else if self.hram_io[IO_LY] < LCD_HEIGHT {
                    if self.hram_io[IO_LY] == 0 {
                        self.display.wy = self.hram_io[IO_WY];
                        self.display.window_clear = 0;
                    }

                    self.hram_io[IO_STAT] =
                        (self.hram_io[IO_STAT] & !STAT_MODE) | IO_STAT_MODE_HBLANK;

                    #[cfg(feature = "gbc")]
                    {
                        if self.cgb.mode != 0 && self.cgb.dma_active == 0 && self.cgb.dma_mode != 0
                        {
                            for i in 0..0x10_usize {
                                self._write(
                                    ((self.cgb.dma_dest as usize & 0x1FF0) | 0x8000) + i,
                                    self._read((self.cgb.dma_source as usize & 0xFFF0) + i),
                                );
                            }
                            self.cgb.dma_source += 0x10;
                            self.cgb.dma_dest += 0x10;
                            self.cgb.dma_size -= 1;
                            if self.cgb.dma_size == 0 {
                                self.cgb.dma_active = 1;
                            }
                        }
                    }

                    if (self.hram_io[IO_STAT] & STAT_MODE_0_INTR) != 0 {
                        self.hram_io[IO_IF] |= LCDC_INTR;
                    }

                    if self.counter.lcd_count < LCD_MODE_2_CYCLES {
                        inst_cycles = (LCD_MODE_2_CYCLES - self.counter.lcd_count) as u8;
                    }
                }
            } else if (self.hram_io[IO_STAT] & STAT_MODE) == IO_STAT_MODE_HBLANK
                && self.counter.lcd_count >= LCD_MODE_2_CYCLES
            {
                self.hram_io[IO_STAT] =
                    (self.hram_io[IO_STAT] & !STAT_MODE) | IO_STAT_MODE_SEARCH_OAM;

                if (self.hram_io[IO_STAT] & STAT_MODE_2_INTR) != 0 {
                    self.hram_io[IO_IF] |= LCDC_INTR
                };

                if self.counter.lcd_count < LCD_MODE_3_CYCLES {
                    inst_cycles = (LCD_MODE_3_CYCLES - self.counter.lcd_count) as u8;
                }
            } else if (self.hram_io[IO_STAT] & STAT_MODE) == IO_STAT_MODE_SEARCH_OAM
                && self.counter.lcd_count >= LCD_MODE_3_CYCLES
            {
                self.hram_io[IO_STAT] =
                    (self.hram_io[IO_STAT] & !STAT_MODE) | IO_STAT_MODE_SEARCH_TRANSFER;
                #[cfg(feature = "lcd")]
                {
                    if !self.lcd_blank {
                        self._draw_line();
                    }
                }
                if self.counter.lcd_count < LCD_MODE_0_CYCLES {
                    inst_cycles = (LCD_MODE_0_CYCLES - self.counter.lcd_count) as u8;
                }
            }

            do_while_condition = self.gb_halt && (self.hram_io[IO_IF] & self.hram_io[IO_IE]) == 0;
        }
    }

    pub fn run_frame(&mut self) -> () {
        self.gb_frame = false;
        while !self.gb_frame {
            self._step_cpu();
        }
    }

    pub fn get_save_size(&mut self) -> usize {
        const RAM_SIZE_LOCATION: usize = 0x0149;
        const RAM_SIZES: [usize; 5] = [0x00, 0x800, 0x2000, 0x8000, 0x20000];
        let ram_size = (self.gb_rom_read)(self, RAM_SIZE_LOCATION);

        if self.mbc == 2 {
            return 0x200;
        }

        return RAM_SIZES[ram_size as usize];
    }

    fn gb_init_serial(
        &mut self,
        gb_serial_tx: fn(&Gb<T>, u8) -> (),
        gb_serial_rx: fn(&Gb<T>, &mut u8) -> GbSerialRxRet,
    ) {
        self.gb_serial_tx = Some(gb_serial_tx);
        self.gb_serial_rx = Some(gb_serial_rx)
    }

    fn gb_colour_hash(&self) -> u8 {
        let mut x: u8 = 0;

        const ROM_TITLE_START_ADDR: usize = 0x0134;
        const ROM_TITLE_END_ADDR: usize = 0x0143;

        for i in ROM_TITLE_START_ADDR..(ROM_TITLE_END_ADDR + 1) {
            x += (self.gb_rom_read)(self, i)
        }
        return x;
    }

    pub fn gb_reset(&mut self) -> () {
        self.gb_halt = false;
        self.gb_ime = true;

        self.selected_rom_bank = 1;
        self.cart_ram_bank = 0;
        self.enable_cart_ram = false;
        self.cart_mode_select = 0;

        self.cycle = 0;

        if self.gb_bootrom_read.is_none() {
            let hdr_chk = (self.gb_rom_read)(self, ROM_HEADER_CHECKSUM_LOC as usize) != 0;

            self.cpu_reg.a = 0x01;
            self.cpu_reg.f.set_z(true);
            self.cpu_reg.f.set_n(false);
            self.cpu_reg.f.set_h(hdr_chk);
            self.cpu_reg.f.set_c(hdr_chk);
            self.cpu_reg.bc.bytes = 0x0013;
            self.cpu_reg.de.bytes = 0x00D8;
            self.cpu_reg.hl.bytes = 0x014D;
            self.cpu_reg.sp.bytes = 0xFFFE;
            self.cpu_reg.pc.bytes = 0x0100;

            self.hram_io[IO_DIV] = 0xAB;
            self.hram_io[IO_LCDC] = 0x91;
            self.hram_io[IO_STAT] = 0x85;
            self.hram_io[IO_BANK] = 0x01;
            #[cfg(feature = "gbc")]
            {
                if self.cgb.mode != 0 {
                    self.cpu_reg.a = 0x11;
                    self.cpu_reg.f.set_z(true);
                    self.cpu_reg.f.set_n(false);
                    self.cpu_reg.f.set_h(hdr_chk);
                    self.cpu_reg.f.set_c(hdr_chk);
                    self.cpu_reg.bc.bytes = 0x0000;
                    self.cpu_reg.de.bytes = 0x0008;
                    self.cpu_reg.hl.bytes = 0x007C;
                    self.hram_io[IO_DIV] = 0xFF;
                }
            }

            self.vram.fill(0x00);
        } else {
            self.cpu_reg.pc.bytes = 0x0000;
            self.hram_io[IO_DIV] = 0x00;
            self.hram_io[IO_LCDC] = 0x00;
            self.hram_io[IO_STAT] = 0x84;
            self.hram_io[IO_BANK] = 0x00;
        }

        self.counter.lcd_count = 0;
        self.counter.div_count = 0;
        self.counter.tima_count = 0;
        self.counter.serial_count = 0;
        self.counter.rtc_count = 0;

        self.direct.joypad = 0xFF;
        self.hram_io[IO_JOYP] = 0xCF;
        self.hram_io[IO_SB] = 0x00;
        self.hram_io[IO_SC] = 0x7E;

        #[cfg(feature = "gbc")]
        {
            if self.cgb.mode != 0 {
                self.hram_io[IO_SC] = 0x7F;
            }
        }

        self.hram_io[IO_TIMA] = 0x00;
        self.hram_io[IO_TMA] = 0x00;
        self.hram_io[IO_TAC] = 0xF8;
        self.hram_io[IO_IF] = 0xE1;

        self.hram_io[IO_SCY] = 0x00;
        self.hram_io[IO_SCX] = 0x00;
        self.hram_io[IO_LY] = 0x00;
        self.hram_io[IO_LYC] = 0x00;
        self._write(0xFF47, 0xFC);
        self._write(0xFF48, 0xFF);
        self._write(0xFF49, 0xFF);
        self.hram_io[IO_WY] = 0x00;
        self.hram_io[IO_WX] = 0x00;
        self.hram_io[IO_IE] = 0x00;
        self.hram_io[IO_IF] = 0xE1;

        #[cfg(feature = "gbc")]
        {
            /* Initialize some CGB registers */
            self.cgb.double_speed = 0;
            self.cgb.double_speed_prep = 0;
            self.cgb.wram_bank = 1;
            self.cgb.wram_bank_offset = WRAM_0_ADDR;
            self.cgb.vram_bank = 0;
            self.cgb.vram_bank_offset = VRAM_ADDR;
            for i in 0..0x20_usize {
                self.cgb.oam_palette[i << 1] = 0x7F;
                self.cgb.bg_palette[i << 1] = 0x7F;
                self.cgb.oam_palette[(i << 1) + 1] = 0xFF;
                self.cgb.bg_palette[(i << 1) + 1] = 0xFF;
            }
            self.cgb.oam_palette_id = 0;
            self.cgb.bg_palette_id = 0;
            self.cgb.oam_palette_inc = 0;
            self.cgb.bg_palette_inc = 0;
            self.cgb.dma_active = 1; // Not active
            self.cgb.dma_mode = 0;
            self.cgb.dma_size = 0;
            self.cgb.dma_source = 0;
            self.cgb.dma_dest = 0;
        }
    }

    pub fn set_joypad(&mut self, joypad: u8) -> () {
        self.direct.joypad = joypad;
    }

    pub fn get_palette(&self) -> &[u16; 0x40] {
        &self.cgb.fix_palette
    }

    pub fn new(
        context: &T,
        gb_rom_read: fn(&Gb<T>, usize) -> u8,
        gb_cart_ram_read: fn(&Gb<T>, usize) -> u8,
        gb_cart_ram_write: fn(&Gb<T>, usize, u8) -> (),
        gb_error: Option<fn(&Gb<T>, GbError, u16) -> ()>,
    ) -> GbInitError<T> {
        #[cfg(feature = "gbc")]
        const CGB_FLAG: u16 = 0x0143;
        const MBC_LOCATION: u16 = 0x0147;
        const BANK_COUNT_LOCATION: u16 = 0x0148;
        const RAM_SIZE_LOCATION: u16 = 0x0149;

        const CART_MBC: [i8; 32] = [
            0, 1, 1, 1, -1, 2, 2, -1, 0, 0, -1, 0, 0, 0, -1, 3, 3, 3, 3, 3, -1, -1, -1, -1, -1, 5,
            5, 5, 5, 5, 5, -1,
        ];
        const CART_RAM: [u8; 32] = [
            0, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0,
            0, 0, 0,
        ];
        const NUM_ROM_BANKS_MASK: [u16; 9] = [2, 4, 8, 16, 32, 64, 128, 256, 512];
        const NUM_RAM_BANKS: [u8; 6] = [0, 1, 1, 4, 16, 8];

        let mut gb = Gb {
            gb_halt: true,
            gb_ime: true,
            gb_frame: true,
            lcd_blank: false,
            mbc: 0,
            cart_ram: 0,
            num_rom_banks_mask: 0,
            num_ram_banks: 0,
            selected_rom_bank: 0,
            cart_ram_bank: 0,
            enable_cart_ram: false,
            cart_mode_select: 0,
            rtc_latched: CartRtc::new(),
            rtc_real: CartRtc::new(),
            cpu_reg: CpuRegisters::new(),
            counter: Count::new(),
            wram: vec![0; WRAM_SIZE],
            vram: vec![0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            hram_io: [0; HRAM_IO_SIZE],
            display: Display::new(),
            direct: Direct::new(),
            #[cfg(feature = "gbc")]
            cgb: Cgb::new(0),
            gb_rom_read,
            gb_cart_ram_read,
            gb_cart_ram_write,
            gb_error,
            gb_serial_tx: None,
            gb_serial_rx: None,
            gb_bootrom_read: None,
            quit: false,
            cycle: 0,
            context,
        };

        let mut x: u8 = 0;
        for i in 0x0134..(0x014C + 1) {
            x = x.wrapping_sub(gb_rom_read(&gb, i)).wrapping_sub(1);
        }

        if x != gb_rom_read(&gb, ROM_HEADER_CHECKSUM_LOC as usize) {
            return GbInitError::GbInitInvalidChecksum;
        }

        gb.mbc = {
            let mbc_value = gb_rom_read(&gb, MBC_LOCATION as usize);
            if mbc_value as usize > std::mem::size_of::<[i8; 32]>() - 1
                || CART_MBC[mbc_value as usize] == -1
            {
                return GbInitError::GbInitCartridgeUnsupported;
            } else {
                CART_MBC[mbc_value as usize]
            }
        };

        gb.cart_ram = CART_RAM[gb_rom_read(&gb, MBC_LOCATION as usize) as usize];

        gb.num_rom_banks_mask =
            NUM_ROM_BANKS_MASK[gb_rom_read(&gb, BANK_COUNT_LOCATION as usize) as usize] - 1;

        gb.num_ram_banks = NUM_RAM_BANKS[gb_rom_read(&gb, RAM_SIZE_LOCATION as usize) as usize];

        gb.cgb = Cgb::new((gb_rom_read(&gb, CGB_FLAG as usize) & 0x80) >> 7);

        gb.gb_reset();
        return GbInitError::GbInitNoError(gb);
    }

    fn gb_get_rom_name(&self) -> String {
        let mut title_loc: u16 = 0x134;
        let title_end: u16 = 0x143;
        let mut title_str = String::new();

        while title_loc <= title_end {
            let title_char = (self.gb_rom_read)(self, title_loc as usize);

            if title_char >= b' ' && title_char <= b'_' {
                title_str.push(title_char as char);
            } else {
                break;
            }
            title_loc += 1;
        }

        title_str
    }

    #[cfg(feature = "lcd")]
    pub fn gb_init_lcd(&mut self, lcd_draw_line: fn(&Gb<T>, [u8; 160], u8) -> ()) -> () {
        self.display.lcd_draw_line = Some(lcd_draw_line);

        self.direct.interlace = false;
        self.display.interlace_count = false;
        self.direct.frame_skip = false;
        self.display.frame_skip_count = false;

        self.display.window_clear = 0;
        self.display.wy = 0;

        return;
    }

    fn gb_set_bootrom(&mut self, gb_bootrom_read: fn(&Gb<T>, usize) -> u8) {
        self.gb_bootrom_read = Some(gb_bootrom_read);
    }

    fn gb_set_rtc(&mut self, sec: u8, min: u8, hour: u8, yday: u16) {
        self.rtc_real.set_sec(sec);
        self.rtc_real.set_min(min);
        self.rtc_real.set_hour(hour);
        self.rtc_real.set_yday((yday & 0xFF) as u8);
        self.rtc_real.set_yday((yday >> 8) as u8);
    }

    pub fn get_context(&self) -> &T {
        &self.context
    }
}

#[cfg(feature = "lcd")]
#[derive(Clone, Copy, Debug)]
struct SpriteData {
    sprite_number: u8,
    x: u8,
}

#[cfg(feature = "lcd")]
#[cfg(feature = "high-lcd-accuracy")]
fn compare_sprites(sd1: &Option<SpriteData>, sd2: &Option<SpriteData>) -> std::cmp::Ordering {
    use std::cmp::Ordering;

    if let (Some(sd1), Some(sd2)) = (sd1, sd2) {
        let x_res = sd1.x as i16 - sd2.x as i16;
        if x_res > 0 {
            return Ordering::Greater;
        } else if x_res < 0 {
            return Ordering::Less;
        }
        if sd1.sprite_number as i16 - sd2.sprite_number as i16 > 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    } else {
        if sd1.is_some() {
            return Ordering::Less;
        }
        if sd2.is_some() {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}
