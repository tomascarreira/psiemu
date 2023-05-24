use bus::Bus;
use cpu::Cpu;

mod cpu;
mod bus;

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();
}
