use crate::cpu::*;

pub fn execute(cpu: &mut Cpu) -> u8 {
    let op_index = cpu.fetch();
    OPCODES[op_index as usize](cpu)
}

#[rustfmt::skip]
const OPCODES: [fn(&mut Cpu) -> u8; 256] = [
    //  0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
    nop_00, ld_01_, ld_02_, inc_03, inc_04, dec_05, ld_06_, todo_0, ld_08_, todo_0, ld_0a_, dec_0b, inc_0c, dec_0d, ld_0e_, todo_0, // 0x00
    todo_0, ld_11_, ld_12_, inc_13, inc_14, dec_15, ld_16_, todo_0, todo_0, todo_0, ld_1a_, dec_1b, inc_1c, dec_1d, ld_1e_, todo_0, // 0x10
    todo_0, ld_21_, ld_22_, inc_23, inc_24, dec_25, ld_26_, todo_0, todo_0, todo_0, ld_2a_, dec_2b, inc_2c, dec_2d, ld_2e_, todo_0, // 0x20
    todo_0, ld_31_, ld_32_, inc_33, inc_34, dec_35, ld_36_, todo_0, todo_0, todo_0, ld_3a_, dec_3b, inc_3c, dec_3d, ld_3e_, todo_0, // 0x30
    ld_40_, ld_41_, ld_42_, ld_43_, ld_44_, ld_45_, ld_46_, ld_47_, ld_48_, ld_49_, ld_4a_, ld_4b_, ld_4c_, ld_4d_, ld_4e_, ld_4f_, // 0x40
    ld_50_, ld_51_, ld_52_, ld_53_, ld_54_, ld_55_, ld_56_, ld_57_, ld_58_, ld_59_, ld_5a_, ld_5b_, ld_5c_, ld_5d_, ld_5e_, ld_5f_, // 0x50
    ld_60_, ld_61_, ld_62_, ld_63_, ld_64_, ld_65_, ld_66_, ld_67_, ld_68_, ld_69_, ld_6a_, ld_6b_, ld_6c_, ld_6d_, ld_6e_, ld_6f_, // 0x60
    ld_70_, ld_71_, ld_72_, ld_73_, ld_74_, ld_75_, todo_0, ld_77_, ld_78_, ld_79_, ld_7a_, ld_7b_, ld_7c_, ld_7d_, ld_7e_, ld_7f_, // 0x70
    add_80, add_81, add_82, add_83, add_84, add_85, add_86, add_87, adc_88, adc_89, adc_8a, adc_8b, adc_8c, adc_8d, adc_8e, adc_8f, // 0x80
    sub_90, sub_91, sub_92, sub_93, sub_94, sub_95, sub_96, sub_97, sbc_98, sbc_99, sbc_9a, sbc_9b, sbc_9c, sbc_9d, sbc_9e, sbc_9f, // 0x90
    and_a0, and_a1, and_a2, and_a3, and_a4, and_a5, and_a6, and_a7, xor_a8, xor_a9, xor_aa, xor_ab, xor_ac, xor_ad, xor_ae, xor_af, // 0xA0
    or_b0_, or_b1_, or_b2_, or_b3_, or_b4_, or_b5_, or_b6_, or_b7_, cp_b8_, cp_b9_, cp_ba_, cp_bb_, cp_bc_, cp_bd_, cp_be_, cp_bf_, // 0xB0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, add_c6, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, adc_ce, todo_0, // 0xC0
    todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, sub_d6, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, todo_0, sbc_de, todo_0, // 0xD0
    ldh_e0, todo_0, ldh_e2, todo_0, todo_0, todo_0, and_e6, todo_0, todo_0, todo_0, ld_ea_, todo_0, todo_0, todo_0, xor_ee, todo_0, // 0xE0
    ldh_f0, todo_0, ldh_f2, todo_0, todo_0, todo_0, or_f6_, todo_0, ld_f8_, ld_f9_, ld_fa_, todo_0, todo_0, todo_0, cp_fe_, todo_0, // 0xF0
];

#[allow(unused)]
fn todo_0(cpu: &mut Cpu) -> u8 {
    todo!();
}

#[allow(unused)]
fn nop_00(cpu: &mut Cpu) -> u8 {
    1
}

fn ld_01_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

fn ld_11_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::DE, val);
    3
}

fn ld_21_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

fn ld_31_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch_u16();
    cpu.set_r16(Regs16::BC, val);
    3
}

fn ld_02_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::BC, val);
    2
}

fn ld_12_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::DE, val);
    2
}

fn ld_22_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::HL, val);
    cpu.inc_r16(Regs16::HL);
    2
}

fn ld_32_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::HL, val);
    cpu.dec_r16(Regs16::HL);
    2
}

fn ld_06_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::B, val);
    2
}

fn ld_16_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::D, val);
    2
}

fn ld_26_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::H, val);
    2
}

fn ld_36_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::HL, val);
    3
}

fn ld_0a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::BC);
    cpu.set_r8(Regs8::A, val);
    2
}

fn ld_1a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::DE);
    cpu.set_r8(Regs8::A, val);
    2
}

fn ld_2a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::A, val);

    cpu.inc_r16(Regs16::HL);
    2
}

fn ld_3a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::A, val);

    cpu.dec_r16(Regs16::HL);
    2
}

fn ld_0e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::C, val);
    2
}

fn ld_1e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::E, val);
    2
}

fn ld_2e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::L, val);
    2
}

fn ld_3e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.set_r8(Regs8::A, val);
    2
}

fn ld_40_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_41_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_42_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_43_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_44_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_45_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_46_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::B, val);
    2
}

fn ld_47_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::B, val);
    1
}

fn ld_48_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_49_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_4a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_4b_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_4c_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_4d_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_4e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::C, val);
    2
}

fn ld_4f_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::C, val);
    1
}

fn ld_50_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_51_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_52_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_53_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_54_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_55_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_56_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::D, val);
    2
}

fn ld_57_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::D, val);
    1
}

fn ld_58_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_59_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_5a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_5b_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_5c_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_5d_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_5e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::E, val);
    2
}

fn ld_5f_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::E, val);
    1
}

fn ld_60_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_61_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_62_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_63_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_64_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_65_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_66_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::H, val);
    2
}

fn ld_67_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::H, val);
    1
}

fn ld_68_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_69_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_6a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_6b_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_6c_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_6d_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_6e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::L, val);
    2
}

fn ld_6f_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::L, val);
    1
}

fn ld_70_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_71_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_72_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_73_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_74_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_75_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::HL, val);
    2
}

fn ld_77_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::HL, val);
    1
}

fn ld_78_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::B);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_79_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::C);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_7a_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::D);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_7b_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::E);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_7c_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::H);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_7d_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::L);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_7e_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.set_r8(Regs8::A, val);
    2
}

fn ld_7f_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::A);
    cpu.set_r8(Regs8::A, val);
    1
}

fn ld_08_(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.get_r16(Regs16::SP);

    cpu.write_ram(addr, val.low_byte());
    cpu.write_ram(addr + 1, val.high_byte());
    5
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
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);

    cpu.write_ram(addr, val.wrapping_add(1));
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
    let addr = cpu.get_r16(Regs16::HL);
    let val = cpu.read_ram(addr);

    cpu.write_ram(addr, val.wrapping_sub(1));
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

fn ldh_e0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.get_r8(Regs8::A);
    cpu.write_ram(addr, val);
    3
}

fn ldh_f0(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs8::A, val);
    3
}

fn ldh_e2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_r8(Regs8::C) as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.get_r8(Regs8::A);
    cpu.write_ram(addr, val);
    2
}

fn ldh_f2(cpu: &mut Cpu) -> u8 {
    let offset = cpu.get_r8(Regs8::C) as u16;
    let addr = 0xFF00 + offset;
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs8::A, val);
    2
}

fn ld_ea_(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.get_r8(Regs8::A);
    cpu.write_ram(addr, val);
    4
}

fn ld_fa_(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch_u16();
    let val = cpu.read_ram(addr);
    cpu.set_r8(Regs8::A, val);
    4
}

fn ld_f8_(cpu: &mut Cpu) -> u8 {
    let offset = cpu.fetch() as i8 as i16 as u16;
    let sp = cpu.get_r16(Regs16::SP);
    let set_c = check_c_carry_u8(sp.low_byte(), offset.low_byte());
    let set_h = check_h_carry_u8(sp.low_byte(), offset.low_byte());

    cpu.set_r16(Regs16::HL, offset.wrapping_add(sp));
    cpu.set_flag(Flags::Z, false);
    cpu.set_flag(Flags::N, false);
    cpu.set_flag(Flags::C, set_c);
    cpu.set_flag(Flags::H, set_h);
    3
}

fn ld_f9_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r16(Regs16::HL);
    cpu.set_r16(Regs16::SP, val);
    2
}

fn add_80(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::B, false);
    1
}

fn add_81(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::C, false);
    1
}

fn add_82(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::D, false);
    1
}

fn add_83(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::E, false);
    1
}

fn add_84(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::H, false);
    1
}

fn add_85(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::L, false);
    1
}

fn add_86(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.add_a_u8(val, false);
    2
}

fn add_87(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::A, false);
    1
}

fn adc_88(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::B, true);
    1
}

fn adc_89(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::C, true);
    1
}

fn adc_8a(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::D, true);
    1
}

fn adc_8b(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::E, true);
    1
}

fn adc_8c(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::H, true);
    1
}

fn adc_8d(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::L, true);
    1
}

fn adc_8e(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.add_a_u8(val, true);
    2
}

fn adc_8f(cpu: &mut Cpu) -> u8 {
    cpu.add_a_u8_from_register(Regs8::A, true);
    1
}

fn sub_90(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::B, false);
    1
}

fn sub_91(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::C, false);
    1
}

fn sub_92(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::D, false);
    1
}

fn sub_93(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::E, false);
    1
}

fn sub_94(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::H, false);
    1
}

fn sub_95(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::L, false);
    1
}

fn sub_96(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.sub_a_u8(val, false);
    2
}

fn sub_97(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::A, false);
    1
}

fn sbc_98(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::B, true);
    1
}

fn sbc_99(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::C, true);
    1
}

fn sbc_9a(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::D, true);
    1
}

fn sbc_9b(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::E, true);
    1
}

fn sbc_9c(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::H, true);
    1
}

fn sbc_9d(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::L, true);
    1
}

fn sbc_9e(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.sub_a_u8(val, true);
    2
}

fn sbc_9f(cpu: &mut Cpu) -> u8 {
    cpu.sub_a_u8_from_register(Regs8::A, true);
    1
}

fn and_a0(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::B);
    1
}

fn and_a1(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::C);
    1
}

fn and_a2(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::D);
    1
}

fn and_a3(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::E);
    1
}

fn and_a4(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::H);
    1
}

fn and_a5(cpu: &mut Cpu) -> u8 {
    cpu.and_a_u8_from_register(Regs8::L);
    1
}

fn and_a6(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.and_a_u8(val);
    2
}

fn and_a7(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::A);
    1
}

fn xor_a8(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::B);
    1
}

fn xor_a9(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::C);
    1
}

fn xor_aa(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::D);
    1
}

fn xor_ab(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::E);
    1
}

fn xor_ac(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::H);
    1
}

fn xor_ad(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::L);
    1
}

fn xor_ae(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.xor_a_u8(val);
    2
}

fn xor_af(cpu: &mut Cpu) -> u8 {
    cpu.xor_a_u8_from_register(Regs8::A);
    1
}

fn or_b0_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::B);
    1
}

fn or_b1_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::C);
    1
}

fn or_b2_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::D);
    1
}

fn or_b3_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::E);
    1
}

fn or_b4_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::H);
    1
}

fn or_b5_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::L);
    1
}

fn or_b6_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.or_a_u8(val);
    2
}

fn or_b7_(cpu: &mut Cpu) -> u8 {
    cpu.or_a_u8_from_register(Regs8::A);
    1
}

fn cp_b8_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::B);
    1
}

fn cp_b9_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::C);
    1
}

fn cp_ba_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::D);
    1
}

fn cp_bb_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::E);
    1
}

fn cp_bc_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::H);
    1
}

fn cp_bd_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::L);
    1
}

fn cp_be_(cpu: &mut Cpu) -> u8 {
    let val = cpu.get_r8(Regs8::HL);
    cpu.cp_a_u8(val);
    2
}

fn cp_bf_(cpu: &mut Cpu) -> u8 {
    cpu.cp_a_u8_from_register(Regs8::A);
    1
}

fn add_c6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.add_a_u8(val, false);
    2
}

fn sub_d6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.sub_a_u8(val, false);
    2
}

fn and_e6(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.and_a_u8(val);
    2
}

fn or_f6_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.or_a_u8(val);
    2
}

fn adc_ce(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.add_a_u8(val, true);
    2
}

fn sbc_de(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.sub_a_u8(val, true);
    2
}

fn xor_ee(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.xor_a_u8(val);
    2
}

fn cp_fe_(cpu: &mut Cpu) -> u8 {
    let val = cpu.fetch();
    cpu.cp_a_u8(val);
    2
}
