const MEMORY: usize = 0xFFFF;
const HALT: u8 = 0x0;
const LOAD_IMMEDIATE: u8 = 0x1;
// const LOAD_FROM_MEM: u8 = 0x2;
const DUMP: u8 = 0x3;
const ADD: u8 = 0x4;
const SUB: u8 = 0x5;
const MULT: u8 = 0x6;
const DIV: u8 = 0x7;
const AND: u8 = 0x8;
const OR: u8 = 0x9;
const NOT: u8 = 0xA;
const XOR: u8 = 0xB;
const LSHIFT: u8 = 0xC;
const RSHIFT: u8 = 0xD;
const PUSH: u8 = 0xE;
const POP: u8 = 0xF;
const DUMP_ACCUM: u8 = 0x10;
const PUSH_ACCUM: u8 = 0x11;

pub struct VM {
    mem: [u8; MEMORY + 1],
    // like 6502 where last register is for setting flags
    regs: [u16; 16],
    accum: u16,
    sp: usize,
    instruc: usize,
    halted: bool,
}

impl VM {
    fn step(&mut self) {
        let opcode = self.get_opcode_nibbles();

        // println!(
        //     "{:X} {:X} {:X} {:X}",
        //     opcode[0], opcode[1], opcode[2], opcode[3]
        // );

        if !self.halted {
            match opcode[0] {
                LOAD_IMMEDIATE => {
                    self.regs[opcode[1] as usize] =
                        (((opcode[2]) as u16) << 8) | ((opcode[3]) as u16);
                    println!("LOAD_IMMEDIATE")
                }
                // LOAD_FROM_MEM => {
                //     let index = (((self.regs[2] as u16) << 8) | (self.regs[3] as u16));
                //     self.regs[opcode[1] as usize] =
                //         ((self.mem[index] as u16) << 8) | (self.mem[index + 1] as u16);
                // }
                DUMP => {
                    let reg = self.regs[opcode[1] as usize];
                    let index = (((opcode[2] as u16) << 8) | (opcode[3] as u16)) as usize;
                    self.mem[index] = (reg >> 8) as u8;
                    self.mem[index + 1] = reg as u8;
                    println!("DUMP");
                }
                ADD => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 + reg2;
                    println!("ADD");
                }
                SUB => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 - reg2;
                    println!("SUB");
                }
                MULT => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 * reg2;
                    println!("MULT");
                }
                DIV => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 / reg2;
                    println!("DIV");
                }
                AND => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 & reg2;
                    println!("AND");
                }
                OR => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 | reg2;
                    println!("OR");
                }
                NOT => {
                    let reg1 = self.regs[opcode[1] as usize];
                    self.accum = !reg1;
                    println!("NOT");
                }
                XOR => {
                    let reg1 = self.regs[opcode[1] as usize];
                    let reg2 = self.regs[opcode[2] as usize];
                    self.accum = reg1 ^ reg2;
                    println!("XOR");
                }
                LSHIFT => {
                    let reg1 = self.regs[opcode[1] as usize];
                    self.accum = reg1 << opcode[2];
                    println!("LSHIFT");
                }
                RSHIFT => {
                    let reg1 = self.regs[opcode[1] as usize];
                    self.accum = reg1 >> opcode[2];
                    println!("RSHIFT");
                }
                PUSH => {
                    let reg = self.regs[opcode[1] as usize];
                    self.mem[self.sp] = reg as u8;
                    self.mem[self.sp - 1] = (reg >> 8) as u8;
                    self.sp -= 2;
                    println!("PUSH");
                }
                POP => {
                    self.regs[opcode[1] as usize] =
                        ((self.mem[self.sp + 2] as u16) << 8) | (self.mem[self.sp + 1] as u16);
                    self.sp += 2;
                    println!("POP");
                }
                DUMP_ACCUM => {
                    let index = (((opcode[2] as u16) << 8) | (opcode[3] as u16)) as usize;
                    self.mem[index] = (self.accum >> 8) as u8;
                    self.mem[index + 1] = self.accum as u8;
                    println!("DUMP_ACCUM");
                }
                PUSH_ACCUM => {
                    self.mem[self.sp] = self.accum as u8;
                    self.mem[self.sp - 1] = (self.accum >> 8) as u8;
                    self.sp -= 2;
                    println!("PUSH_ACCUM");
                }
                HALT => {
                    self.halted = true;
                    println!("HALT");
                }
                _ => println!("{:#?}", opcode),
            }
        }

        self.instruc += 2;
    }
    fn get_opcode_nibbles(&self) -> [u8; 4] {
        let pt1 = self.mem[self.instruc];
        let pt2 = self.mem[self.instruc + 1];

        let n1 = self.byte_to_nibbles(pt1);
        let n2 = self.byte_to_nibbles(pt2);

        // println!("{:X} {:X}", pt1, pt2);
        // println!("{:X} {:X} {:X} {:X}", n1[0], n1[1], n2[0], n2[1]);

        [n1[0], n1[1], n2[0], n2[1]]
    }
    fn byte_to_nibbles(&self, byte: u8) -> [u8; 2] {
        let n1 = byte >> 4;
        let n2 = byte & 0xF;

        [n1, n2]
    }
    fn insert_opcode(&mut self, opcode: u16) {
        self.mem[self.instruc] = (opcode >> 8) as u8;
        self.mem[self.instruc + 1] = opcode as u8;
    }
    pub fn run(&mut self, opcode: u16) {
        self.insert_opcode(opcode);
        self.step();
    }
}

impl Default for VM {
    fn default() -> Self {
        Self {
            mem: [0; MEMORY + 1],
            regs: [0; 16],
            instruc: 0,
            sp: MEMORY,
            accum: 0,
            halted: false,
        }
    }
}
