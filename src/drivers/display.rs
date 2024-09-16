use embedded_graphics::{
    pixelcolor::{raw::RawU16, Rgb555, Rgb565},
    prelude::*,
    primitives::Rectangle,
};
use mipidsi::options::{ColorInversion, ColorOrder, Orientation, Rotation};
use svc::hal::{
    self,
    gpio::{Output, OutputPin, PinDriver},
    spi::{config::Duplex, SpiConfig, SpiDeviceDriver, SpiDriver},
    units::MegaHertz,
};

use crate::cashew_gb::{LCD_HEIGHT, LCD_PALETTE_ALL, LCD_WIDTH};

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
            SpiDeviceDriver<'p, &'p SpiDriver<'p>>,
            PinDriver<'p, DC, Output>,
        >,
        mipidsi::models::ST7735s,
        PinDriver<'p, RST, Output>,
    >,
    buffer: Vec<Rgb565>,
    palette: [[u16; 4]; 3],
    area: Rectangle,
}
impl<'p, DC, RST> Display<'p, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn new<CS>(
        spi_driver: &'p SpiDriver<'static>,
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

        let mut driver = mipidsi::Builder::new(mipidsi::models::ST7735s, spi_interface)
            .reset_pin(rst)
            .orientation(Orientation {
                rotation: Rotation::Deg270,
                mirrored: false,
            })
            .color_order(ColorOrder::Rgb)
            .init(&mut hal::delay::Ets)
            .unwrap();
        driver.clear(Rgb565::BLACK).unwrap();

        Display {
            driver,
            buffer: vec![Rgb565::WHITE; LCD_WIDTH as usize * LCD_HEIGHT as usize],
            palette: [
                [0x7FFF, 0x03E0, 0x1A00, 0x0120], /* OBJ0 */
                [0x7FFF, 0x329F, 0x001F, 0x001F], /* OBJ1 */
                [0x7FFF, 0x7E10, 0x48E7, 0x0000], /* BG */
            ],
            area: Rectangle::new(Point::new(0, 0), Size::new(LCD_WIDTH as u32, 128)),
        }
    }
}

impl<'p, DC, RST> Display<'p, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    pub fn buffer_line_gb(&mut self, pixels: [u8; 160], line: u8) -> () {
        for x in 0..LCD_WIDTH as usize {
            self.buffer[x + (LCD_WIDTH as usize * line as usize)] = Rgb565::from(RawU16::new(
                self.palette[((pixels[x] & LCD_PALETTE_ALL) as usize) >> 4][pixels[x] as usize & 3],
            ));
        }
    }
    pub fn buffer_line_gbc(&mut self, pixels: [u8; 160], line: u8, palette: [u16; 0x40]) -> () {
        for x in 0..LCD_WIDTH as usize {
            self.buffer[x + (LCD_WIDTH as usize * line as usize)] =
                Rgb565::from(Rgb555::from(RawU16::new(palette[pixels[x] as usize])));
        }
    }
    pub fn draw(&mut self) -> () {
        self.driver
            .fill_contiguous(&self.area, self.buffer.clone())
            .unwrap();
    }
}
