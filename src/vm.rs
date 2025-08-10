const MEMORY: usize = 2 ^ 16;
const HALT: u8 = 0x0;
const LOAD_IMMEDIATE: u8 = 0x1;
const LOAD_FROM_MEM: u8 = 0x2;
const DUMP: u8 = 0x3;

pub struct VM {
    mem: Vec<u8>,
    // like 6502 where last register is for setting flags
    regs: [u16; 16],
    instruc: usize,
    halted: bool,
}

impl VM {
    fn step(&mut self) {
        let opcode = self.get_opcode_nibbles();

        if !self.halted {
            match opcode[0] {
                LOAD_IMMEDIATE => {
                    self.regs[opcode[1] as usize] =
                        ((self.regs[2] as u16) << 8) | (self.regs[3] as u16);
                }
                LOAD_FROM_MEM => {
                    let index = (((self.regs[2] as u16) << 8) | (self.regs[3] as u16)) as usize;
                    self.regs[opcode[1] as usize] =
                        ((self.mem[index] as u16) << 8) | (self.mem[index + 1] as u16);
                }
                DUMP => {
                    let reg = self.regs[opcode[1] as usize];
                    let index = (((opcode[2] as u16) << 8) | (opcode[3] as u16)) as usize;
                    self.mem[index] = (reg >> 8) as u8;
                    self.mem[index + 1] = reg as u8;
                }
                HALT | _ => {
                    self.halted = true;
                }
            }
        }
    }
    fn get_opcode_nibbles(&self) -> [u8; 4] {
        let pt1 = self.mem[self.instruc];
        let pt2 = self.mem[self.instruc + 1];

        let n1 = self.byte_to_nibbles(pt1);
        let n2 = self.byte_to_nibbles(pt2);

        [n1[0], n1[1], n2[0], n2[1]]
    }
    fn byte_to_nibbles(&self, byte: u8) -> [u8; 2] {
        let n1 = byte >> 4;
        let n2 = byte & (0xF << 4);

        [n1, n2]
    }
}

impl Default for VM {
    fn default() -> Self {
        Self {
            mem: Vec::with_capacity(MEMORY),
            regs: [0; 16],
            instruc: 0,
        }
    }
}
