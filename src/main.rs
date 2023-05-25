use bus::Bus;
use cpu::Cpu;

mod bus;
mod cpu;

fn main() {
    let mut cpu = Cpu::new();
    let mut bus = Bus::new();

    loop {
        cpu.cpu_cycle(&mut bus);
    }
}
