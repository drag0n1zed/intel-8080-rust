use std::fs;
use std::path::Path;

mod disassembler;

fn main() {
    let rom_path = Path::new("roms/invaders/invaders.h");
    match fs::read(rom_path) {
        Ok(rom_bytes) => disassembler::disassemble(&rom_bytes),
        Err(e) => eprintln!("Error: {}", e),
    };
}
