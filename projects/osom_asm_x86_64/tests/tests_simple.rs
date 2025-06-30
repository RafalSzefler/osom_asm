use osom_asm_x86_64::{
    assembler::X86_64Assembler,
    models::{Condition, GPR, Immediate, Instruction, Label},
};
use osom_tools_dev::macros::assert_eq_hex;
use rstest::rstest;

#[test]
fn test_simple_mov() {
    let mut assembler = X86_64Assembler::new(false);
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, [0x48, 0x33, 0xC0, 0xC3]);
}

#[rstest]
#[case(true, &[0x48, 0x33, 0xC0, 0xEB, 0xFB, 0xC3])]
#[case(false, &[0x48, 0x33, 0xC0, 0xE9, 0xF8, 0xFF, 0xFF, 0xFF, 0xC3])]
fn test_jmp(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler.emit(Instruction::Jump_Label { dst: label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
}

#[rstest]
#[case(true, &[0xEB, 0x03, 0x48, 0x33, 0xC0, 0xC3])]
#[case(false, &[0xE9, 0x03, 0x00, 0x00, 0x00, 0x48, 0x33, 0xC0, 0xC3])]
fn test_jmp_forward(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler.emit(Instruction::Jump_Label { dst: label }).unwrap();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
}

#[rstest]
#[case(true, &[0x48, 0x33, 0xC0, 0x77, 0xFB, 0xC3])]
#[case(false, &[0x48, 0x33, 0xC0, 0x0F, 0x87, 0xF7, 0xFF, 0xFF, 0xFF, 0xC3])]
fn test_jmp_cond(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler
        .emit(Instruction::CondJump_Label {
            condition: Condition::Above,
            dst: label,
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
}

#[rstest]
#[case(true, &[0x77, 0x03, 0x48, 0x33, 0xC0, 0xC3])]
#[case(false, &[0x0F, 0x87, 0x03, 0x00, 0x00, 0x00, 0x48, 0x33, 0xC0, 0xC3])]
fn test_jmp_cond_forward(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler
        .emit(Instruction::CondJump_Label {
            condition: Condition::Above,
            dst: label,
        })
        .unwrap();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
}
