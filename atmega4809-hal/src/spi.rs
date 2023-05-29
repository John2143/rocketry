use embedded_hal::spi::{blocking::Transfer, ErrorType, ErrorKind};
use ufmt::derive::uDebug;

pub struct SPI;

/*
 *
24.4 Register Summary - SPIn
Offset Name Bit Pos.
0x00 CTRLA 7:0 DORD MASTER CLK2X PRESC[1:0] ENABLE
0x01 CTRLB 7:0 BUFEN BUFWR SSD MODE[1:0]
0x02 INTCTRL 7:0 RXCIE TXCIE DREIE SSIE IE
0x03 INTFLAGS 7:0 RXCIF TXCIF DREIF SSIF BUFOVF
0x04 DATA 7:0 DATA[7:0]

*/
#[derive(Debug, uDebug)]
pub enum SPIError {
    ReadOverflow,
    Other,
}

pub const PORTMUX: *mut u8 = 0x05E0 as *mut _;
pub const SPI0: *mut u8 = 0x08C0 as *mut _;

pub enum Polarity {
    ///Leading edge: Rising, sample
    ///Trailing edge: Falling, setup
    P0 = 0x0,
    ///Leading edge: Rising, setup
    ///Trailing edge: Falling, sample
    P1 = 0x1,
    ///Leading edge: Falling, sample
    ///Trailing edge: Rising, setup
    P2 = 0x2,
    ///Leading edge: Falling, setup
    ///Trailing edge: Rising, sample
    P3 = 0x3,
}

impl SPI {
    pub fn setup(high_speed: bool, wait_for_receive: bool, mode: Polarity) {
        // Initialization
        // Initialize the SPI to a basic functional state by following these steps:
        // 1. Configure the SS pin in the port peripheral.
        // 2. Select SPI Master/Slave operation by writing the Master/Slave Select bit (MASTER) in the Control A register
        // (SPIn.CTRLA).

        // 3. In Master mode, select the clock speed by writing the Prescaler bits (PRESC) and the Clock Double bit
        // (CLK2X) in SPIn.CTRLA.
        // 4. Optional: Select the Data Transfer mode by writing to the MODE bits in the Control B register (SPIn.CTRLB).
        // 5. Optional: Write the Data Order bit (DORD) in SPIn.CTRLA.
        // 6. Optional: Setup Buffer mode by writing BUFEN and BUFWR bits in the Control B register (SPIn.CTRLB).
        // (1.5) disable SS by writing 1 to SSD in SPIn.CTRLB: we are always the master
        // 7. Optional: To disable the multi-master support in Master mode, write ‘1’ to the Slave Select Disable bit (SSD) in
        // SPIn.CTRLB.
        let ctrl_b = {
            0b1000_0100 | //Buffer enable, disable SS
            if wait_for_receive { 0b0100_0000 } else { 0 } |
            (mode as u8)
        };
        unsafe { SPI0.offset(0x01).write_volatile(ctrl_b) };
        // 8. Enable the SPI by writing a ‘1’ to the ENABLE bit in SPIn.CTRLA
        let ctrl_a = {
            0b0110_0000 | // LSB first, master mode,
            if !high_speed { 0b0000_0110 } else { 0 } | // low speed, write 0x3 to PRESC
            1 //enable
        };
        unsafe { SPI0.offset(0x00).write_volatile(ctrl_a) };
    }

    fn raw_read_byte() -> u8 {
        unsafe { SPI0.offset(0x04).read_volatile() }
    }

    fn raw_write_byte(byte: u8) {
        unsafe { SPI0.offset(0x04).write_volatile(byte) }
    }

    pub fn get_bus_status() -> BusStatus {
        BusStatus(unsafe { SPI0.offset(0x03).read_volatile() })
    }
}

impl Transfer<u8> for SPI {
    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        let mut wptr = 0;
        let mut rptr = 0;

        loop {
            let status = Self::get_bus_status();
            if wptr < write.len() && status.dreif() {
                Self::raw_write_byte(write[wptr]);
                wptr += 1;
            }

            if status.rxcif() {
                if rptr < read.len() {
                    read[rptr] = Self::raw_read_byte();
                    rptr += 1;
                } else {
                    return Err(ErrorKind::Overrun);
                }
            }

            if status.bufovf() {
                // TODO ??
                //return Err(SPIError::ReadOverflow);
            }

            if status.ssif() {
                unreachable!("SPI0.SSIF somehow triggered?");
            }

            if status.txcif() && write.len() == wptr {
                // && wptr == rptr ?? not sure if SPI always
                // sends the same amount of data both dirs
                //
                // it probably does cause shared clock, huh

                break;
            }
        }

        Ok(())
    }
}

impl ErrorType for SPI {
    type Error = ErrorKind;
}



pub struct BusStatus(u8);

impl BusStatus {
    pub fn rxcif(&self) -> bool {
        self.0 & 0b1000_0000 > 0
    }

    pub fn txcif(&self) -> bool {
        self.0 & 0b0100_0000 > 0
    }

    pub fn dreif(&self) -> bool {
        self.0 & 0b0010_0000 > 0
    }

    pub fn ssif(&self) -> bool {
        self.0 & 0b0001_0000 > 0
    }

    pub fn bufovf(&self) -> bool {
        self.0 & 0b1000_0001 > 0
    }
}
