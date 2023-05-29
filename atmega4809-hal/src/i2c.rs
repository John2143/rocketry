use embedded_hal::blocking::i2c::{self, Read, Write};

pub struct I2C;
/*
 *
Before enabling the master or the slave unit, ensure that the correct settings for SDASETUP, SDAHOLD, and, if
used, Fast-mode plus (FMPEN) are stored in TWI.CTRLA. If alternate pins are to be used for the slave, this must be
specified in the TWIn.DUALCTRL register as well. Note that for dual mode the master enables the primary SCL/SDA
pins, while the ENABLE bit in TWIn.DUALCTRL enables the secondary pins

Master Operation
It is recommended to write the Master Baud Rate register (TWIn.BAUD) before enabling the TWI master since
TIMEOUT is dependent on the baud rate setting. To start the TWI master, write a ‘1’ to the ENABLE bit and configure
an appropriate TIMEOUT if using the TWI in an SMBus environment. The ENABLE and TIMEOUT bits are all located
in the Master Control A register (TWIn.MCTRLA). If no TIMEOUT value is set, which is the case for I²C operation, the
bus state must be manually set to IDLE by writing 0x1 to BUSSTATE in TWIn.MSTATUS at a “safe” point in time.
Note that unlike the SMBus specification, the I²C specification does not specify when it is safe to assume that the bus
is IDLE in a multi-master system. The application can solve this by ensuring that after all masters connected to the
bus are enabled, one supervising master performs a transfer before any of the other masters. The stop condition of
this initial transfer will indicate to the Bus State Monitor logic that the bus is IDLE and ready

25.3.4.2.1: The TWIn.MBAUD register must be set to a value that results in a TWI bus clock frequency (fSCL) equal or less than
100 kHz/400 kHz/1 MHz, dependent on the mode used by the application (Standard mode Sm/Fast mode Fm/Fast
mode plus Fm+).
The low (TLOW) and high (THIGH) times are determined by the Baud Rate register (TWIn.MBAUD), while the rise
(TRISE) and fall (TFALL) times are determined by the bus topology. Because of the wired-AND logic of the bus, TFALL
will be considered as part of TLOW. Likewise, TRISE will be in a state between TLOW and THIGH until a high state has
been detected


Offset Name Bit Pos.
0x00 CTRLA 7:0 SDASETUP SDAHOLD[1:0] FMPEN
0x01 DUALCTRL 7:0 SDAHOLD[1:0] FMPEN ENABLE
0x02 DBGCTRL 7:0 DBGRUN
0x03 MCTRLA 7:0 RIEN WIEN QCEN TIMEOUT[1:0] SMEN ENABLE
0x04 MCTRLB 7:0 FLUSH ACKACT MCMD[1:0]
0x05 MSTATUS 7:0 RIF WIF CLKHOLD RXACK ARBLOST BUSERR BUSSTATE[1:0]
0x06 MBAUD 7:0 BAUD[7:0]
0x07 MADDR 7:0 ADDR[7:0]
0x08 MDATA 7:0 DATA[7:0]
0x09 SCTRLA 7:0 DIEN APIEN PIEN PMEN SMEN ENABLE
0x0A SCTRLB 7:0 ACKACT SCMD[1:0]
0x0B SSTATUS 7:0 DIF APIF CLKHOLD RXACK COLL BUSERR DIR AP
0x0C SADDR 7:0 ADDR[7:0]
0x0D SDATA 7:0 DATA[7:0]
0x0E SADDRMASK 7:0 ADDRMASK[6:0] ADDREN
https://2143.me/f/xmwH.png


*/

pub enum BusState {
    Unknown,
    Idle,
    Owner,
    Busy,
}

pub enum RW {
    DirRead,
    DirWrite,
}

pub struct BusStatus(u8);

#[derive(Debug)]
pub enum I2CError {
    NACK,
    PartialTransmit(u8),
    ArbLost,
}

impl I2C {
    const TWI0: *mut u8 = 0x08A0 as *mut _;
    pub fn setup() {
        let bus_timeout = 0x01;
        //sdahold 500ns, fmpen no
        unsafe { I2C::TWI0.offset(0x00).write_volatile(0b1100) };
        //set bus timeout to 50us, turn off master
        unsafe { I2C::TWI0.offset(0x03).write_volatile(bus_timeout << 2) };
        //baud 0x0a = Same that arduino uses when at max clock
        unsafe { I2C::TWI0.offset(0x06).write_volatile(0x0b) };

        unsafe {
            //~~interrupts~~ + timeout
            I2C::TWI0
                .offset(0x03)
                .write_volatile(bus_timeout << 2 /* | 0b11000000 */);

            //set bus state to idle
            I2C::TWI0.offset(0x05).write_volatile(0x01);
        }
    }

    pub fn wait_for_bus() {
        loop {
            match I2C::get_bus_status().get_bus_state() {
                Ok(BusState::Idle) => break,
                Ok(BusState::Busy) => {}
                Ok(BusState::Owner) => {}
                Ok(BusState::Unknown) => {}
                Err(_) => {}
            }
        }
    }

    pub fn wait_wif() -> BusStatus {
        loop {
            let status = Self::get_bus_status();
            if status.wif() {
                return status;
            }
        }
    }

    pub fn write(address: u8, data: &[u8]) -> Result<(), I2CError> {
        let pre_enable = unsafe { I2C::TWI0.offset(0x03).read_volatile() };
        //turn on chip
        unsafe { I2C::TWI0.offset(0x03).write_volatile(pre_enable | 1) };

        Self::wait_for_bus();
        unsafe { I2C::TWI0.offset(0x07).write_volatile(address << 1 | 0) };

        let status = Self::wait_wif();

        if status.rxack() == CK::NACK {
            //Case M3: Address Packet " ", Not ACK by client
            return Err(I2CError::NACK);
        } else if status.arblost() {
            //Case M4: Error
            return Err(I2CError::ArbLost);
        } else if status.rxack() == CK::ACK && status.clkhld() {
            //Case M1: Address Packet Transmit Complete, Dir bit set 0
            //
            //prepare to transmit data
        } else {
            //unreachable!()
            return Err(I2CError::ArbLost);
        }

        if data.len() >= 1 {
            let (last, rest) = data.split_last().unwrap();
            for (i, c) in rest.iter().enumerate() {
                match Self::write_byte(*c) {
                    Ok(_) => {}
                    Err(I2CError::NACK) => return Err(I2CError::PartialTransmit(i as u8)),
                    Err(e) => return Err(e),
                }
            }
            Self::write_last_byte(*last)?;
        } else {
        }

        //turn off chip
        unsafe { I2C::TWI0.offset(0x03).write_volatile(pre_enable | 0) };
        return Ok(());
    }

    pub fn write_byte(data: u8) -> Result<(), I2CError> {
        unsafe { I2C::TWI0.offset(0x08).write_volatile(data) };
        let status = Self::wait_wif();

        if status.arblost() {
            return Err(I2CError::ArbLost);
        } else if status.rxack() == CK::NACK {
            return Err(I2CError::NACK);
        }

        Ok(())
    }

    pub fn write_last_byte(data: u8) -> Result<(), I2CError> {
        unsafe { I2C::TWI0.offset(0x08).write_volatile(data) };
        Self::stop();
        let status = Self::wait_wif();
        if status.arblost() {
            return Err(I2CError::ArbLost);
        }

        Ok(())
    }

    pub fn stop() {
        unsafe { I2C::TWI0.offset(0x04).write_volatile(0x03) };
    }

    pub fn recv_trans() {
        unsafe { I2C::TWI0.offset(0x04).write_volatile(0x02) };
    }

    pub fn rep_start() {
        unsafe { I2C::TWI0.offset(0x04).write_volatile(0x01) };
    }

    pub fn respond(c: CK) {
        unsafe {
            I2C::TWI0.offset(0x04).write_volatile(match c {
                CK::NACK => 0x04,
                CK::ACK => 0x00,
            })
        };
    }

    //for &c in data {
    //Self::write_byte(c);
    //}

    pub fn wait_rif() -> BusStatus {
        loop {
            let status = Self::get_bus_status();
            if status.rif() {
                return status;
            }
        }
    }

    pub fn read<const N: usize>(address: u8) -> Result<[u8; N], I2CError> {
        let mut buf = [0u8; N];
        Self::read_to_buf(address, &mut buf)?;
        Ok(buf)
    }

    pub fn read_to_buf(address: u8, buf: &mut [u8]) -> Result<(), I2CError> {
        let pre_enable = unsafe { I2C::TWI0.offset(0x03).read_volatile() };
        //turn on chip
        unsafe { I2C::TWI0.offset(0x03).write_volatile(pre_enable | 1) };

        Self::wait_for_bus();

        unsafe { I2C::TWI0.offset(0x07).write_volatile(address << 1 | 1) };

        for b in buf {
            *b = Self::read_byte()?;
        }

        Self::stop();

        for _ in 0..100 {
            unsafe { I2C::TWI0.offset(0x00).read_volatile() };
        }
        //turn off chip
        unsafe { I2C::TWI0.offset(0x03).write_volatile(pre_enable | 0) };

        Ok(())
    }

    pub fn read_byte() -> Result<u8, I2CError> {
        let mut status = Self::get_bus_status();
        loop {
            if status.arblost() {
                return Err(I2CError::ArbLost);
            //} else if status.rxack() == CK::NACK {
            //return Err(I2CError::NACK);
            } else if status.rif() {
                break;
            }
            status = Self::wait_rif();
        }

        let v = unsafe { I2C::TWI0.offset(0x08).read_volatile() };
        Self::respond(CK::ACK);
        Self::recv_trans();
        //Self::stop();

        Ok(v)
    }

    pub fn get_bus_status() -> BusStatus {
        let bs = unsafe { I2C::TWI0.offset(0x05).read_volatile() };
        BusStatus(bs)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CK {
    ACK,
    NACK,
}

impl BusStatus {
    pub fn get_bus_state(&self) -> Result<BusState, ()> {
        if self.0 & 0b100 > 0 {
            return Err(());
        } else {
            match self.0 & 0b011 {
                0 => Ok(BusState::Unknown),
                1 => Ok(BusState::Idle),
                2 => Ok(BusState::Owner),
                3 => Ok(BusState::Busy),
                _ => unreachable!(),
            }
        }
    }

    pub fn rif(&self) -> bool {
        self.0 & 0b1000_0000 > 0
    }

    pub fn wif(&self) -> bool {
        self.0 & 0b0100_0000 > 0
    }

    pub fn rxack(&self) -> CK {
        match self.0 & 0b0000_1000 > 0 {
            true => CK::NACK,
            false => CK::ACK,
        }
    }

    pub fn clkhld(&self) -> bool {
        self.0 & 0b0010_0000 > 0
    }

    pub fn arblost(&self) -> bool {
        self.0 & 0b0000_1000 > 0
    }
}

impl i2c::Read for I2C {
    type Error = I2CError;

    fn read(&mut self, address: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
        Self::read_to_buf(address, buffer)
    }
}

impl i2c::Write for I2C {
    type Error = I2CError;

    fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Self::Error> {
        Self::write(address, bytes)
    }
}

impl i2c::WriteRead for I2C {
    type Error = I2CError;

    fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.write(address, bytes)?;
        self.read(address, buffer)?;
        Ok(())
    }
}


impl embedded_hal::i2c::blocking::I2c for I2C {
}
