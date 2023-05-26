use bus::Bus;
use clap::Parser;
use cpu::Cpu;

mod bus;
mod cpu;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    bios: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args =  Args::parse();

    let bios = std::fs::read(args.bios)?;
    
    let mut cpu = Cpu::new();
    let mut bus = Bus::new(bios);

    loop {
        cpu.cpu_cycle(&mut bus);
    }
}
