
const REGS: [&str; 8] = ["B", "C", "D", "E", "H", "L", "M", "A"];
const REG_PAIRS: [&str; 4] = ["B", "D", "H", "SP"];

pub fn disassemble(rom: &[u8]) {
    let mut pc = 0x0;
    while pc < rom.len() {
        pc += print_opcode_return_increment(&rom, pc);
    }
}

fn print_opcode_return_increment(rom: &[u8], pc: usize) -> usize {
    let opcode = rom[pc];
    let (op_size, mnemonic, operand) = match opcode {
        // NOP
        0x00 | 0x08 | 0x10 | 0x18 | 0x20 | 0x28 | 0x30 | 0x38 => (1, "NOP", "".to_string()),

        // LXI rp, D16
        0x01 | 0x11 | 0x21 | 0x31 => {
            let rp = opcode >> 4;
            (3, "LXI", format!("{},${:04x}", REG_PAIRS[rp as usize], get_addr(rom, pc)))
        }
        // STAX rp
        0x02 | 0x12 => {
            let rp = opcode >> 4;
            (1, "STAX", format!("{}", REG_PAIRS[rp as usize]))
        }
        // LDAX rp
        0x0A | 0x1A => {
            let rp = opcode >> 4;
            (1, "LDAX", format!("{}", REG_PAIRS[rp as usize]))
        }
        // SHLD addr
        0x22 => (3, "SHLD", format!("${:04x}", get_addr(rom, pc))),
        // LHLD addr
        0x2A => (3, "LXLD", format!("${:04x}", get_addr(rom, pc))),
        // STA addr

        0x01 => (3, "LXI", format!("B,${:04x}", get_addr(rom, pc))),
        0x02 => (1, "STAX", "B".to_string()),
        0x03 => (1, "INX", "B".to_string()),
        0x04 => (1, "INR", "B".to_string()),
        0x05 => (1, "DCR", "B".to_string()),
        0x06 => (2, "MVI", format!("B,${:02x}", rom[pc + 1])),
        0x07 => (1, "RLC", "".to_string()),

        0x09 => (1, "DAD", "B".to_string()),
        0x0A => (1, "LDAX", "B".to_string()),
        0x0B => (1, "DCX", "B".to_string()),
        0x0C => (1, "INR", "C".to_string()),
        0x0D => (1, "DCR", "C".to_string()),
        0x0E => (2, "MVI", format!("C,${:02x}", rom[pc + 1])),
        0x0F => (1, "RRC", "".to_string()),

        0x11 => (3, "LXI", format!("D,${:04x}", get_addr(rom, pc))),
        0x12 => (1, "STAX", "D".to_string()),
        0x13 => (1, "INX", "D".to_string()),
        0x14 => (1, "INR", "D".to_string()),
        0x15 => (1, "DCR", "D".to_string()),
        0x16 => (2, "MVI", format!("D,${:02x}", rom[pc + 1])),
        0x17 => (1, "RAL", "".to_string()),

        0x19 => (1, "DAD", "D".to_string()),
        0x1A => (1, "LDAX", "D".to_string()),
        0x1B => (1, "DCX", "D".to_string()),
        0x1C => (1, "INR", "E".to_string()),
        0x1D => (1, "DCR", "E".to_string()),
        0x1E => (2, "MVI", format!("E,${:02x}", rom[pc + 1])),
        0x1F => (1, "RAR", "".to_string()),

        0x20 => (1, "NOP", "".to_string()),
        0x21 => (3, "LXI", format!("H,${:04x}", get_addr(rom, pc))),
        0x22 => (3, "SHLD", format!("${:04x}", get_addr(rom, pc))),
        0x23 => (1, "INX", "H".to_string()),
        0x24 => (1, "INR", "H".to_string()),
        0x25 => (1, "DCR", "H".to_string()),
        0x26 => (2, "MVI", format!("H,${:02x}", rom[pc + 1])),
        0x27 => (1, "DAA", "".to_string()),

        0x28 => (1, "NOP", "".to_string()),
        0x29 => (1, "DAD", "H".to_string()),
        0x2A => (3, "LXLD", format!("${:04x}", get_addr(rom, pc))),
        0x2B => (1, "DCX", "H".to_string()),
        0x2C => (1, "INR", "L".to_string()),
        0x2D => (1, "DCR", "L".to_string()),
        0x2E => (2, "MVI", format!("L,${:02x}", rom[pc + 1])),
        0x2F => (1, "CMA", "".to_string()),


        0xc3 => (3, "JMP", format!("${:04x}", get_addr(rom, pc))),
        0xc5 => (1, "PUSH", "B".to_string()),
        0xd5 => (1, "PUSH", "D".to_string()),

        0xf5 => (1, "PUSH", "PSW".to_string()),
        _ => panic!("Unknown opcode {:02x}", rom[pc]),
    };
    let mut bytes_str = format!("{:02x}", rom[pc]);
    if op_size > 1 {
        bytes_str.push_str(&format!(" {:02x}", rom[pc + 1]));
    }
    if op_size > 2 {
        bytes_str.push_str(&format!(" {:02x}", rom[pc + 2]));
    }

    println!("{:04x} {:<8} {:<6} {}", pc, bytes_str, mnemonic, operand);
    op_size
}

fn get_addr(rom: &[u8], pc: usize) -> u16 {
    let lo = rom[pc + 1] as u16;
    let hi = rom[pc + 2] as u16;
    (hi << 8) | lo
}
