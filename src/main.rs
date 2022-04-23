#![allow(unused)]

fn main() {
    let mut memory: [u8; 4096] = [0; 4096];
    let mem = &mut memory;
    let add_twice = [0x80, 0x14, 0x80, 0x14, 0x00, 0xee];
    mem[0x100..0x106].copy_from_slice(&add_twice);
    println!("{:?}", &mem[0x100..0x106]);
}

struct MemoryAddr(u16);

impl MemoryAddr {
    fn new(addr: u16) -> MemoryAddr {
        assert_eq!(0, addr >> 12);
        MemoryAddr(addr)
    }
}

pub struct CPU {
    registers: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_ptr: usize,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            registers: [0; 16],
            position_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_ptr: 0,
        }
    }
    fn read_opcode(&self) -> u16 {
        let idx_op = self.position_in_memory;
        let op_l = self.memory[idx_op] as u16;
        let op_r = self.memory[idx_op + 1] as u16;
        op_l << 8 | op_r
    }
    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            let c = ((opcode & 0xf000) >> 12) as u8;
            let x = ((opcode & 0x0f00) >> 8) as u8;
            let y = ((opcode & 0x00f0) >> 4) as u8;
            let d = (opcode & 0x000f) as u8;
            match (c, x, y, d) {
                (0, 0, 0, 0) => return,
                (2, _, _, _) => {
                    let addr =
                        MemoryAddr::new(((x as u16) << 8) | ((y as u16) << 4) | ((d as u16) << 0));
                    self.call(addr);
                }
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }
    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];
        let (val, overflow) = arg1.overflowing_add(arg2);
        self.registers[x as usize] = val;
        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }
    fn call(&mut self, addr: MemoryAddr) {
        self.stack_ptr += 1;
        self.stack[self.stack_ptr] = addr.0;
    }
    fn op_return(&mut self) {}
}
