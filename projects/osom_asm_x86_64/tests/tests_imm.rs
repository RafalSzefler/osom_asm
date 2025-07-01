use rstest::rstest;

use osom_asm_x86_64::models::{Immediate32, Immediate64, Size};

#[rstest]
#[case(0, Size::Bit8)]
#[case(-1, Size::Bit8)]
#[case(1, Size::Bit8)]
#[case(i8::MIN as i32, Size::Bit8)]
#[case(i8::MAX as i32, Size::Bit8)]
#[case(i16::MIN as i32, Size::Bit16)]
#[case(-30000, Size::Bit16)]
#[case(25000, Size::Bit16)]
#[case(i16::MAX as i32, Size::Bit16)]
#[case(i32::MIN as i32, Size::Bit32)]
#[case(-300000000, Size::Bit32)]
#[case(250000000, Size::Bit32)]
#[case(i32::MAX as i32, Size::Bit32)]
fn test_imm_size(#[case] value: i32, #[case] expected: Size) {
    let imm = Immediate32::new(value);
    assert_eq!(imm.real_size(), expected);
}

#[rstest]
#[case(0, Size::Bit8)]
#[case(-1, Size::Bit8)]
#[case(1, Size::Bit8)]
#[case(i8::MIN as i64, Size::Bit8)]
#[case(i8::MAX as i64, Size::Bit8)]
#[case(i16::MIN as i64, Size::Bit16)]
#[case(-30000, Size::Bit16)]
#[case(25000, Size::Bit16)]
#[case(i16::MAX as i64, Size::Bit16)]
#[case(i32::MIN as i64, Size::Bit32)]
#[case(-300000000, Size::Bit32)]
#[case(250000000, Size::Bit32)]
#[case(i32::MAX as i64, Size::Bit32)]
#[case(i64::MIN as i64, Size::Bit64)]
#[case(-3000000000000000000, Size::Bit64)]
#[case(2500000000000000000, Size::Bit64)]
#[case(i64::MAX as i64, Size::Bit64)]
fn test_imm64_size(#[case] value: i64, #[case] expected: Size) {
    let imm = Immediate64::new(value);
    assert_eq!(imm.real_size(), expected);
}
