pub struct SPI;
#[derive(Debug)]
pub enum SPIError {}

impl SPI {
    pub fn setup() {
        todo!()
    }
}

impl embedded_hal::blocking::spi::Transfer<u8> for SPI {
    type Error = SPIError;

    fn transfer<'w>(&mut self, _words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        todo!()
    }
}
