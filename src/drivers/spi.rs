use std::sync::{Arc, LazyLock, Mutex};

use svc::hal::{
    gpio::{AnyIOPin, AnyInputPin, AnyOutputPin, InputPin, OutputPin, Pin},
    peripheral::Peripheral,
    spi::{
        self,
        config::{self as SpiConfig, DriverConfig},
        Dma, Spi, SpiAnyPins, SpiDeviceDriver, SpiDriver, SpiDriverConfig,
    },
    units::FromValueType,
};

pub struct SpiPins<SCK: OutputPin, MOSI: OutputPin, MISO: InputPin>
where
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
{
    sck: SCK,
    mosi: MOSI,
    miso: MISO,
}
impl<SCK, MOSI, MISO> SpiPins<SCK, MOSI, MISO>
where
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
{
    pub fn new(sck: SCK, mosi: MOSI, miso: MISO) -> Self {
        SpiPins { sck, mosi, miso }
    }
}

pub fn create_spi_driver<'p, SCK, MOSI, MISO, SPI>(
    pins: SpiPins<SCK, MOSI, MISO>,
    spi: impl Peripheral<P = SPI> + 'static,
) -> SpiDriver<'static>
where
    SCK: OutputPin,
    MOSI: OutputPin,
    MISO: InputPin,
    SPI: SpiAnyPins,
{
    SpiDriver::new(
        spi,
        pins.sck,
        pins.mosi,
        Some(pins.miso),
        &DriverConfig::default().dma(Dma::Auto(4096)),
    )
    .unwrap()
}
