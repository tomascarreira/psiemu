use crate::cpu::Exception;

struct SimpleRam(Vec<u8>);

impl SimpleRam {
    fn new(size: usize) -> Self {
        SimpleRam(vec![0; size])
    }

    fn read_byte(&self, address: u32) -> u8 {
        self.0[address as usize]
    }

    fn read_halfword(&self, address: u32) -> u16 {
        u16::from_le_bytes(
            <[u8; 2]>::try_from(&self.0[address as usize..address as usize + 2]).unwrap(),
        )
    }

    fn read_word(&self, address: u32) -> u32 {
        u32::from_le_bytes(
            <[u8; 4]>::try_from(&self.0[address as usize..address as usize + 4]).unwrap(),
        )
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        self.0[address as usize] = value;
    }

    fn write_halfword(&mut self, address: u32, value: u16) {
        self.0[address as usize..address as usize + 2].copy_from_slice(&value.to_le_bytes());
    }

    fn write_word(&mut self, address: u32, value: u32) {
        self.0[address as usize..address as usize + 4].copy_from_slice(&value.to_le_bytes());
    }
}

struct SimpleRom(Vec<u8>);

impl SimpleRom {
    fn new(data: Vec<u8>) -> Self {
        SimpleRom(data)
    }

    fn read_byte(&self, address: u32) -> u8 {
        self.0[address as usize]
    }

    fn read_halfword(&self, address: u32) -> u16 {
        u16::from_le_bytes(
            <[u8; 2]>::try_from(&self.0[address as usize..address as usize + 2]).unwrap(),
        )
    }

    fn read_word(&self, address: u32) -> u32 {
        u32::from_le_bytes(
            <[u8; 4]>::try_from(&self.0[address as usize..address as usize + 4]).unwrap(),
        )
    }
}

// I dont like this name very much but i can think of a better one
enum AddressBusDevice {
    Ram(u32),
    Scratchpad(u32),
    Bios(u32),
    Unknown(u32),
}

pub struct Bus {
    ram: SimpleRam,
    scratchpad: SimpleRam,
    bios: SimpleRom,
}

impl Bus {
    pub fn new(bios: Vec<u8>) -> Self {
        Bus {
            ram: SimpleRam::new(0x200000),
            scratchpad: SimpleRam::new(0x400),
            bios: SimpleRom::new(bios),
        }
    }

    pub fn read_byte(&self, address: u32) -> Result<u8, Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => Ok(self.ram.read_byte(address)),
            AddressBusDevice::Scratchpad(address) => Ok(self.scratchpad.read_byte(address)),
            AddressBusDevice::Bios(address) => Ok(self.bios.read_byte(address)),
            AddressBusDevice::Unknown(address) => {
                println!("Bus read on unknown device on address {:x}", address);
                Ok(0)
            }
        }
    }

    pub fn read_halfword(&self, address: u32) -> Result<u16, Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => Ok(self.ram.read_halfword(address)),
            AddressBusDevice::Scratchpad(address) => Ok(self.scratchpad.read_halfword(address)),
            AddressBusDevice::Bios(address) => Ok(self.bios.read_halfword(address)),
            AddressBusDevice::Unknown(address) => {
                println!("Bus read on unknown device on address {:x}", address);
                Ok(0)
            }
        }
    }

    pub fn read_word(&self, address: u32) -> Result<u32, Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => Ok(self.ram.read_word(address)),
            AddressBusDevice::Scratchpad(address) => Ok(self.scratchpad.read_word(address)),
            AddressBusDevice::Bios(address) => Ok(self.bios.read_word(address)),
            AddressBusDevice::Unknown(address) => {
                println!("Bus read on unknown device on address {:x}", address);
                Ok(0)
            }
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) -> Result<(), Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => {
                self.ram.write_byte(address, value);
                Ok(())
            }
            AddressBusDevice::Scratchpad(address) => {
                self.ram.write_byte(address, value);
                Ok(())
            }
            AddressBusDevice::Bios(_) => {
                println!("Trying to write to bios that I think is read-only");
                Ok(())
            }
            AddressBusDevice::Unknown(address) => {
                println!("Bus write on unknown device of {:x} on address {:x}", value, address);
                Ok(())
            }
        }
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) -> Result<(), Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => {
                self.ram.write_halfword(address, value);
                Ok(())
            }
            AddressBusDevice::Scratchpad(address) => {
                self.ram.write_halfword(address, value);
                Ok(())
            }
            AddressBusDevice::Bios(_) => {
                println!("Trying to write to bios that I think is read-only");
                Ok(())
            }
            AddressBusDevice::Unknown(address) => {
                println!("Bus write on unknown device of {:x} on address {:x}", value, address);
                Ok(())
            }
        }
    }

    pub fn write_word(&mut self, address: u32, value: u32) -> Result<(), Exception> {
        match bus_device_address(address) {
            AddressBusDevice::Ram(address) => {
                self.ram.write_word(address, value);
                Ok(())
            }
            AddressBusDevice::Scratchpad(address) => {
                self.ram.write_word(address, value);
                Ok(())
            }
            AddressBusDevice::Bios(_) => {
                println!("Trying to write to bios that I think is read-only");
                Ok(())
            }
            AddressBusDevice::Unknown(address) => {
                println!("Bus write on unknown device of {:x} on address {:x}", value, address);
                Ok(())
            }
        }
    }
}

// Think of a better name for this
fn bus_device_address(address: u32) -> AddressBusDevice {
    match address {
        0x00000000..=0x001fffff => AddressBusDevice::Ram(address),
        0x1f800000..=0x1f8003ff => AddressBusDevice::Scratchpad(address - 0x1f800000),
        0x1fc00000..=0x1fc7ffff => AddressBusDevice::Bios(address - 0x1fc00000),
        _ => AddressBusDevice::Unknown(address),
    }
}
