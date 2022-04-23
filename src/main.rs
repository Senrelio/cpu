#![allow(unused)]

fn main() {
    let opcode = Opcode { code: 0x2100 };
    // dbg!(opcode.optype());
    let mut cpu = CPU {
        registers: [0; 16],
        pos_in_mem: 0,
        memory: [0; 4096],
        stack: [0; 128],
        ptr_stack: 0,
    };
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    let mem = &mut cpu.memory;
    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[0x002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0x00;
    mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xee;
    cpu.run();
    assert_eq!(cpu.registers[0], 45);
}

pub struct CPU {
    registers: [u8; 16],
    pos_in_mem: usize,
    memory: [u8; 4096],
    stack: [u16; 128],
    ptr_stack: usize,
}

impl CPU {
    fn read_opcode(&self) -> Opcode {
        let pos = self.pos_in_mem;
        let l = self.memory[pos] as u16;
        let r = self.memory[pos + 1] as u16;
        Opcode { code: l << 8 | r }
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.pos_in_mem += 2; //assume the current opcode is executed successfully
                                  // println!("{:04x}: {:?}", &opcode.code, &opcode.optype());
            match opcode.optype() {
                OpType::Add { x, y } => {
                    self.add(x, y);
                }
                OpType::Call { addr } => {
                    self.call(addr);
                }
                OpType::Ret => self.ret(),
                OpType::Finish => {
                    return;
                }
                OpType::Unknown => todo!(),
            }
        }
    }
    fn add(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (result, overflowed) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = result;
        if overflowed {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }
    fn call(&mut self, addr: u16) {
        if self.ptr_stack >= self.stack.len() {
            panic!("stack overflow");
        }
        self.stack[self.ptr_stack] = self.pos_in_mem as u16;
        self.ptr_stack += 1;
        self.pos_in_mem = addr as usize;
    }
    fn ret(&mut self) {
        if self.ptr_stack == 0 {
            panic!("return to where?");
        }
        self.ptr_stack -= 1;
        let return_to = self.stack[self.ptr_stack];
        self.stack[self.ptr_stack] = 0;
        self.pos_in_mem = return_to as usize;
    }
}

#[derive(Debug)]
struct Opcode {
    code: u16,
}

impl Opcode {
    fn c(&self) -> u8 {
        ((self.code & 0xf000) >> 12) as u8
    }
    fn x(&self) -> u8 {
        ((self.code & 0x0f00) >> 8) as u8
    }
    fn y(&self) -> u8 {
        ((self.code & 0x00f0) >> 4) as u8
    }
    fn n(&self) -> u8 {
        (self.code & 0x000f) as u8
    }
    fn optype(&self) -> OpType {
        // println!("{}", self.code);
        let c = self.c();
        let x = self.x();
        let y = self.y();
        let n = self.n();
        match (c, x, y, n) {
            (8, _, _, 4) => OpType::Add { x, y },
            (2, _, _, _) => OpType::Call {
                addr: { self.code & 0x0fff },
            },
            (0, 0, 0xe, 0xe) => OpType::Ret,
            (0, 0, 0, 0) => OpType::Finish,
            _ => {
                println!("{:02x}, {:02x}, {:02x}, {:02x}", c, x, y, n);
                OpType::Unknown
            }
        }
    }
}

#[derive(Debug)]
enum OpType {
    Add { x: u8, y: u8 },
    Call { addr: u16 },
    Ret,
    Finish,
    Unknown,
}
