use crate::{gpio::GPIO, set16};

pub struct USART<const ADDR: u16, const ALT: bool>;

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
0x0A CTRLD 8:0 ABW[1:0]
0x0B DBGCTRL 7:0 DBGRUN
0x0C EVCTRL 7:0 IREI
0x0D TXPLCTRL 7:0 TXPL[7:0]
0x0E RXPLCTRL 7:0 RXPL[6:0]
*/

pub const PORTMUX: *mut u8 = 0x05E0 as *mut _;
pub const USART0: u16 = 0x0800;
pub const USART1: u16 = 0x0820;
pub const USART2: u16 = 0x0840;
pub const USART3: u16 = 0x0860;

pub const BAUD9600: u16 = (17 << 6) | 0b0001_1000;

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

//testing func
pub fn t() {
    GPIO::PORTA(1).output_enable();
    GPIO::PORTA(1).output_high();
}

impl<const UADDR: u16, const ALT: bool> USART<UADDR, ALT> {
    fn addr() -> *mut u8 {
        UADDR as *mut _
    }

    fn get_out_pin() -> GPIO {
        let pin = if ALT { 4 } else { 0 };
        match UADDR {
            USART0 => GPIO::PORTA(pin),
            USART1 => GPIO::PORTC(pin),
            USART2 => GPIO::PORTF(pin),
            USART3 => GPIO::PORTB(pin),
            _ => unreachable!(),
        }
    }

    pub fn change_baud(baud: u16) {
        set16(unsafe { Self::addr().offset(0x08) }, baud);
    }

    pub fn setup(
        baud: u16,
        m: CommunicationMode,
        p: ParityMode,
        s: StopBitMode,
        wsize: CharacterSize,
    ) {
        // 1. Set the baud rate (USARTn.BAUD).
        Self::change_baud(baud);
        //let baud = 0b00010001 | 0b00011010 << 8;
        //set16(unsafe { Self::addr().offset(0x08) }, baud);

        // 2. Set the frame format and mode of operation (USARTn.CTRLC).
        let ctrl_c = {
            (m as u8) << 6 | // (fmt)
            (p as u8) << 4 |
            (s as u8) << 3 |
            wsize as u8
        };
        unsafe { Self::addr().offset(0x07).write_volatile(ctrl_c) };

        // 3. Configure the TXD pin as an output.
        // see 15.3.3 PORTMUX Control for USART
        //unsafe { PORTMUX.offset(0x02).write(0b0100_0000) };
        let usart_pmux = unsafe { PORTMUX.offset(0x02) };
        let cur = unsafe { usart_pmux.read_volatile() };
        let bit_target = match UADDR {
            USART0 => 0b0000_0001,
            USART1 => 0b0000_0100,
            USART2 => 0b0001_0000,
            USART3 => 0b0100_0000,
            _ => unreachable!(),
        };

        unsafe {
            if ALT {
                usart_pmux.write_volatile(cur | bit_target)
            } else {
                //turn 0x1 to 0x3 and clear the bits
                usart_pmux.write_volatile(cur & !(bit_target | (bit_target << 1)))
            }
        };

        let out_pin = Self::get_out_pin();
        out_pin.output_enable();
        //out_pin.pin_ctrl_pullup(true);
        out_pin.pin_ctrl_isc(&crate::gpio::ISC::InputDisable);
        //crate::gpio::GPIO::PORTC(4).output_high();

        // (3.5, enable ints)
        //unsafe { Self::addr().offset(0x05).write_volatile(0b1010_0000) };

        for _ in 0..0xff {
            unsafe { core::arch::asm!("nop") };
        }
        // 4. Enable the transmitter and the receiver (USARTn.CTRLB)
        //unsafe { Self::addr().offset(0x06).write_volatile(0b1100_0000) };
        unsafe { Self::addr().offset(0x06).write_volatile(0b1100_0000) };

        //9600
        //8 0b00010001
        //9 0b00011010
        //9601
        //8 0b00010000
        //9 0b00011010

        //9602
        //8 0b00001111
        //9 0b00011010
        //
        //9600*2
        //8 0b00001000
        //9 0b00001101
        //9600*8
        //8 0b00110111
        //9 0b00010110
        //57600
        //8 0b01011000
        //9 0b00000100

        //Arduino memory dump:
        // 0x00000000  0x0
        // 0x00000000  0x1
        // 0x01110100  0x2
        // 0x00000000  0x3
        // 0x00000000  0x4
        // 0x10100000  0x5
        // 0x11000000  0x6
        // 0x00000011  0x7
        // 0x10000100  0x8
        // 0x00000110  0x9
        // 0x00000000  0xa
        // 0x00000000  0xb
        // 0x00000000  0xc
        // 0x00000000  0xd
        // 0x00000000  0xe
    }

    pub fn transact<'k>(mut write: &[u8], read: &'k mut [u8]) -> Result<&'k [u8], USARTError> {
        let mut read_is_done = false;
        let mut empty_reads = 0u16;
        let mut char_count = 0;
        loop {
            let status = Self::get_bus_status();
            if !write.is_empty() && status.dreif() {
                match write.split_first() {
                    Some((to_write, rest)) => {
                        unsafe { Self::addr().offset(0x02).write_volatile(*to_write) };
                        write = rest;
                    }
                    None => {}
                }
            }

            if status.rxcif() && char_count < read.len() {
                let new_byte = unsafe { Self::addr().offset(0x00).read_volatile() };
                read[char_count] = new_byte;
                char_count += 1;

                if new_byte == b'\n' {
                    read_is_done = true;
                }
                empty_reads = 0;
            } else {
                empty_reads += 1;
            }

            if write.is_empty()
                && status.txcif()
                && (read.is_empty() || read_is_done || empty_reads >= 0x2000)
            {
                break;
            }
        }

        Ok(&read[0..char_count])
    }

    pub fn stop() {
        //unsafe { Self::addr().offset(0x06).write_volatile(0b0000_0000) };
    }

    pub fn off() {
        unsafe { Self::addr().offset(0x06).write_volatile(0b0000_0000) };
    }

    pub fn get_bus_status() -> BusStatus {
        BusStatus(unsafe { Self::addr().offset(0x04).read_volatile() })
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

impl<const UADDR: u16, const ALT: bool> ufmt::uWrite for USART<UADDR, ALT> {
    type Error = USARTError;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        Self::transact(s.as_bytes(), &mut []).map(drop)
    }
}
