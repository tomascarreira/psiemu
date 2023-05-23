struct Ram(Vec<u8>);

impl Ram {
    fn new() -> Self {
        Ram(Vec::with_capacity(0x200000))
    }
}

struct Scratchpad(Vec<u8>);

impl Scratchpad {
    fn new() -> Self {
        Scratchpad(Vec::with_capacity(0x400))
    }
}

pub struct Bus {
    ram: Ram,
    scratchpad: Scratchpad,
}

impl Bus {
    pub fn new() -> Self {

        Bus { ram: Ram::new(), scratchpad: Scratchpad::new() }
    }
}
