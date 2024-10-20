use std::{
    sync::Mutex,
    thread::{self, JoinHandle},
};

use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb555, Rgb565},
    prelude::*,
    primitives::Rectangle,
};
use mipidsi::options::{ColorInversion, ColorOrder, Orientation, Rotation};
use svc::{
    hal::{
        self,
        gpio::{Output, OutputPin, PinDriver},
        spi::{config::Duplex, SpiConfig, SpiDeviceDriver, SpiDriver},
        units::MegaHertz,
    },
    sys,
};

use crate::peanut_gb::{LCD_HEIGHT, LCD_PALETTE_ALL, LCD_WIDTH};
use crate::*;

const B_MASK: u16 = 0b11111 << 10;
const G_MASK: u16 = 0b11111 << 5;
const R_MASK: u16 = 0b11111;

pub enum DisplayMessage {
    Buffer(([u8; 160], u8)),
    Draw([u16; 0x40]),
}

pub struct DisplayPins<CS, DC, RST>
where
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
{
    cs: CS,
    dc: DC,
    rst: RST,
}
impl<CS, DC, RST> DisplayPins<CS, DC, RST>
where
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn new(cs: CS, dc: DC, rst: RST) -> DisplayPins<CS, DC, RST> {
        DisplayPins { cs, dc, rst }
    }
}

pub struct Display<'p, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    driver: mipidsi::Display<
        display_interface_spi::SPIInterface<
            SpiDeviceDriver<'p, SpiDriver<'p>>,
            PinDriver<'p, DC, Output>,
        >,
        mipidsi::models::ILI9341Rgb565,
        PinDriver<'p, RST, Output>,
    >,
    palette: [[u16; 4]; 3],
    area: Rectangle,
}
impl<'p, DC, RST> Display<'p, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn new<CS>(
        spi_driver: SpiDriver<'static>,
        pins: DisplayPins<CS, DC, RST>,
    ) -> Display<'p, DC, RST>
    where
        CS: OutputPin,
    {
        let dc = PinDriver::output(pins.dc).unwrap();
        let rst = PinDriver::output(pins.rst).unwrap();

        let spi_device_driver = SpiDeviceDriver::new(
            spi_driver,
            Some(pins.cs),
            &SpiConfig::new()
                .baudrate(MegaHertz(60).into())
                .duplex(Duplex::Full)
                .write_only(true),
        )
        .unwrap();

        let spi_interface = display_interface_spi::SPIInterface::new(spi_device_driver, dc);

        let mut driver = mipidsi::Builder::new(mipidsi::models::ILI9341Rgb565, spi_interface)
            .reset_pin(rst)
            .orientation(Orientation {
                rotation: Rotation::Deg90,
                mirrored: true,
            })
            .color_order(ColorOrder::Bgr)
            .init(&mut hal::delay::Ets)
            .unwrap();
        driver.clear(Rgb565::BLACK).unwrap();

        Display {
            driver,
            palette: [
                [0x7FFF, 0x03E0, 0x1A00, 0x0120], /* OBJ0 */
                [0x7FFF, 0x329F, 0x001F, 0x001F], /* OBJ1 */
                [0x7FFF, 0x7E10, 0x48E7, 0x0000], /* BG */
            ],
            area: Rectangle::new(Point::new(0, 0), Size::new(LCD_WIDTH as u32, 144)),
        }
    }
}

impl<'p, DC, RST> Display<'p, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    // pub fn buffer_line_gbc(&mut self, pixels: [u8; 160], line: u8, palette: [u16; 0x40]) -> () {
    //     for x in 0..LCD_WIDTH as usize {
    //         self.buffer[x + (LCD_WIDTH as usize * line as usize)] =
    //             rgb565_from_u16(palette[pixels[x as usize] as usize]);
    //     }
    // }
    // pub fn buffer_line_gbc(&mut self, pixels: [u8; 160], line: u8) -> () {
    //     self.buffer[(line as usize * LCD_WIDTH as usize)
    //         ..(LCD_WIDTH as usize + line as usize * LCD_WIDTH as usize)]
    //         .copy_from_slice(&pixels);
    // }
    // pub fn buffer_line_gbc(&mut self, pixels: [u8; 160], line: u8) -> () {
    //     self.buffer.insert_line(pixels, line);
    // }
    pub fn draw(&mut self, palette: &[u16; 0x40]) -> () {
        let frame = unsafe { FRAME_BUFFER.map(|p| rgb565_from_u16(palette[p as usize])) }.to_vec();
        self.driver.fill_contiguous(&self.area, frame).unwrap();
    }
    // pub fn draw(&mut self) -> () {
    //     self.driver
    //         .fill_contiguous(&self.area, self.buffer.clone())
    //         .unwrap();
    // }
    // pub fn draw_line_gbc(&mut self, pixels: [u8; 160], line: u8, palette: &[u16; 0x40]) -> () {
    //     let color_iter = pixels.map(|p| rgb565_from_u16(palette[p as usize]));
    //     self.driver
    //         .set_pixels(0, line as u16, 160, line as u16, color_iter)
    //         .unwrap()
    // }
}

fn rgb565_from_u16(color: u16) -> Rgb565 {
    let r = ((color & B_MASK) >> 10) as u8;
    let g = ((color & G_MASK) >> 5) as u8;
    let b = (color & R_MASK) as u8;
    Rgb565::new(r, g, b)
}
