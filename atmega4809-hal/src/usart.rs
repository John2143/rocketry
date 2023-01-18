use crate::set16;

pub struct USART;

/*
0x00 RXDATAL 7:0 DATA[7:0]
0x01 RXDATAH 7:0 RXCIF BUFOVF FERR PERR DATA[8]
0x02 TXDATAL 7:0 DATA[7:0]
0x03 TXDATAH 7:0 DATA[8]
0x04 STATUS 7:0 RXCIF TXCIF DREIF RXSIF ISFIF BDF WFB
0x05 CTRLA 7:0 RXCIE TXCIE DREIE RXSIE LBME ABEIE RS485[1:0]
0x06 CTRLB 7:0 RXEN TXEN SFDEN ODME RXMODE[1:0] MPCM
0x07 CTRLC 7:0 CMODE[1:0] PMODE[1:0] SBMODE CHSIZE[2:0]
0x07 CTRLC 7:0 CMODE[1:0] UDORD UCPHA
0x08 BAUDL 7:0 BAUD[7:0]
0x09 BAUDH 7:0 BAUD[15:8]
0x0A CTRLD 7:0 ABW[1:0]
0x0B DBGCTRL 7:0 DBGRUN
0x0C EVCTRL 7:0 IREI
0x0D TXPLCTRL 7:0 TXPL[7:0]
0x0E RXPLCTRL 7:0 RXPL[6:0]
*/

pub const USART0: *mut u8 = 0x0800 as *mut _;
pub const USART1: *mut u8 = 0x0820 as *mut _;
pub const USART2: *mut u8 = 0x0840 as *mut _;
pub const USART3: *mut u8 = 0x0860 as *mut _;

pub enum CommunicationMode {
    ///Asynchronous USART
    Asynchronous = 0x00,
    ///Synchronous USART
    Synchronous = 0x01,
    ///Infrared Communication
    IRCOM = 0x02,
    ///Master SPI
    MSPI = 0x03,
}

pub enum ParityMode {
    ///Disabled
    Disabled = 0x0,
    ///Enabled, even parity
    Even = 0x2,
    ///Enabled, odd parity
    Odd = 0x3,
}

pub enum StopBitMode {
    One = 0,
    Two = 1,
}

pub enum CharacterSize {
    ///5-bit
    B5 = 0x00,
    ///6-bit
    B6 = 0x01,
    ///7-bit
    B7 = 0x02,
    ///8-bit
    B8 = 0x03,
    ///9-bit (Low byte first)
    B9L = 0x06,
    ///9-bit (High byte first)
    B9H = 0x07,
}

#[derive(Debug)]
pub enum USARTError {
    ReadOverflow,
    Other,
}

impl USART {
    pub fn setup(
        baud: u16,
        m: CommunicationMode,
        p: ParityMode,
        s: StopBitMode,
        wsize: CharacterSize,
    ) {
        // 1. Set the baud rate (USARTn.BAUD).
        set16(unsafe { USART0.offset(0x08) }, baud);
        // 2. Set the frame format and mode of operation (USARTn.CTRLC).
        let ctrl_c = {
            (m as u8) << 6 | // (fmt)
            (p as u8) << 4 |
            (s as u8) << 3 |
            wsize as u8
        };
        unsafe { USART0.offset(0x07).write_volatile(ctrl_c) };
        // 3. Configure the TXD pin as an output.
        crate::gpio::GPIO::PORTC(4).output_enable();
        crate::gpio::GPIO::PORTC(4).pin_ctrl_pullup(true);
        crate::gpio::GPIO::PORTC(4).pin_ctrl_isc(&crate::gpio::ISC::InputDisable);
        // 4. Enable the transmitter and the receiver (USARTn.CTRLB)
        unsafe { USART0.offset(0x06).write_volatile(0b1100_0000) };
    }

    pub fn transact(mut write: &[u8], mut read: &mut [u8]) -> Result<(), USARTError> {
        loop {
            let status = Self::get_bus_status();
            if status.rxcif() {
                //we can read a new byte
                read = match read.split_first_mut() {
                    Some((first, rest)) => {
                        *first = unsafe { USART0.offset(0x00).read_volatile() };
                        rest
                    }
                    None => {
                        Self::stop();
                        return Err(USARTError::ReadOverflow);
                    }
                }
            } else if !write.is_empty() && status.dreif() {
                match write.split_first() {
                    Some((to_write, rest)) => {
                        unsafe { USART0.offset(0x02).write_volatile(*to_write) };
                        write = rest;
                    }
                    None => {}
                }
            }
            if write.is_empty() && read.is_empty() {
                break;
            }
        }
        Self::stop();
        Ok(())
    }

    pub fn stop() {
        unsafe { USART0.offset(0x06).write_volatile(0b0000_0000) };
    }

    pub fn get_bus_status() -> BusStatus {
        BusStatus(unsafe { USART0.offset(0x04).read_volatile() })
    }
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

    pub fn rxsif(&self) -> bool {
        self.0 & 0b0001_0000 > 0
    }

    pub fn isfif(&self) -> bool {
        self.0 & 0b0000_1000 > 0
    }

    pub fn bdf(&self) -> bool {
        self.0 & 0b0000_0010 > 0
    }

    pub fn wfb(&self) -> bool {
        self.0 & 0b1000_0001 > 0
    }
}
