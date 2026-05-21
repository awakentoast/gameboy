use crate::cpu::*;

#[rustfmt::skip]
const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    //  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    nop_00, todo_0, todo_0, inc_03, inc_04, dec_05, todo_0, todo_0, todo_0, todo_0, todo_0, dec_0b, inc_0c, dec_0d, todo_0, todo_0, // 0x00
    todo_0, todo_0, todo_0, inc_13, inc_14, dec_15, todo_0, todo_0, todo_0, todo_0, todo_0, dec_1b, inc_1c, dec_1d, todo_0, todo_0, // 0x10
    todo_0, todo_0, todo_0, inc_23, inc_24, dec_25, todo_0, todo_0, todo_0, todo_0, todo_0, dec_2b, inc_2c, dec_2d, todo_0, todo_0, // 0x20
    todo_0, todo_0, todo_0, inc_33, inc_34, dec_35, todo_0, todo_0, todo_0, todo_0, todo_0, dec_3b, inc_3c, dec_3d, todo_0, todo_0, // 0x30
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x40
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x50
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x60
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x70
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x80
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0x90
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xA0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xB0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xC0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xD0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xE0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, // 0xF0
];

#[allow(unused)]
fn todo_0(cpu: &mut Cpu) -> u8 {
    todo!();
}

#[allow(unused)]
fn nop_00(cpu: &mut Cpu) -> u8 {
    1
}

fn inc_03(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::BC);
    2
}

fn inc_13(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::DE);
    2
}

fn inc_23(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::HL);
    2
}

fn inc_33(cpu: &mut Cpu) -> u8 {
    cpu.inc_r16(Regs16::SP);
    2
}

fn inc_04(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::B);
    1
}

fn inc_14(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::D);
    1
}

fn inc_24(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::H);
    1
}

fn inc_34(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::HL);
    3
}

fn inc_0c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::C);
    1
}

fn inc_1c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::E);
    1
}

fn inc_2c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::L);
    1
}

fn inc_3c(cpu: &mut Cpu) -> u8 {
    cpu.inc_r8(Regs8::A);
    1
}

fn dec_05(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::B);
    1
}

fn dec_15(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::D);
    1
}

fn dec_25(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::H);
    1
}

fn dec_35(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::HL);
    3
}


fn dec_0d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::C);
    1
}

fn dec_1d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::E);
    1
}

fn dec_2d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::L);
    1
}

fn dec_3d(cpu: &mut Cpu) -> u8 {
    cpu.dec_r8(Regs8::A);
    1
}

fn dec_0b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::BC);
    2
}

fn dec_1b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::DE);
    2
}

fn dec_2b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::HL);
    2
}

fn dec_3b(cpu: &mut Cpu) -> u8 {
    cpu.dec_r16(Regs16::SP);
    2
}

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}
