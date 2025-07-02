use std::collections::HashMap;

use osom_asm_x86_64::{
    assembler::X86_64AssemblerBuilder,
    models::{Condition, GPR, Immediate32, Instruction, Label, Memory, Scale},
};
use osom_tools_dev::macros::assert_eq_hex;
use rstest::rstest;

#[test]
fn test_simple_mov() {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(false).build();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    let expected = &[0x48, 0xC7, 0xC0, 0x00, 0x00, 0x00, 0x00, 0xC3];
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0x48, 0x31, 0xC0, 0xEB, 0xFB, 0xC3])]
#[case(false, &[0x48, 0x31, 0xC0, 0xE9, 0xF8, 0xFF, 0xFF, 0xFF, 0xC3])]
fn test_jmp(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
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
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0xEB, 0x03, 0x48, 0x31, 0xC0, 0xC3])]
#[case(false, &[0xE9, 0x03, 0x00, 0x00, 0x00, 0x48, 0x31, 0xC0, 0xC3])]
fn test_jmp_forward(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
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
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0x48, 0x31, 0xC0, 0x77, 0xFB, 0xC3])]
#[case(false, &[0x48, 0x31, 0xC0, 0x0F, 0x87, 0xF7, 0xFF, 0xFF, 0xFF, 0xC3])]
fn test_jmp_cond(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
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
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0x77, 0x03, 0x48, 0x31, 0xC0, 0xC3])]
#[case(false, &[0x0F, 0x87, 0x03, 0x00, 0x00, 0x00, 0x48, 0x31, 0xC0, 0xC3])]
fn test_jmp_cond_forward(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
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
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0x50, 0x5F, 0xC3])]
#[case(false, &[0x50, 0x5F, 0xc3])]
fn test_push_pop(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
    assembler.emit(Instruction::Push_Reg { src: GPR::RAX }).unwrap();
    assembler.emit(Instruction::Pop_Reg { src: GPR::RDI }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0x6A, 0x11, 0x5F, 0xC3])]
#[case(false, &[0x6A, 0x11, 0x5F, 0xC3])]
fn test_push_pop_imm(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
    assembler
        .emit(Instruction::Push_Imm {
            src: Immediate32::new(17),
        })
        .unwrap();
    assembler.emit(Instruction::Pop_Reg { src: GPR::RDI }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0xFF, 0x35, 0x08, 0x00, 0x00, 0x00, 0x42, 0x8F, 0x04, 0x55, 0x11, 0x00, 0x00, 0x00, 0xC3])]
#[case(false, &[0xFF, 0x35, 0x08, 0x00, 0x00, 0x00, 0x42, 0x8F, 0x04, 0x55, 0x11, 0x00, 0x00, 0x00, 0xC3])]
fn test_push_pop_mem(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
    let label = Label::new();
    let mem = Memory::scaled(GPR::R10, Scale::Scale2, Immediate32::new(17)).unwrap();
    assembler
        .emit(Instruction::Push_Mem {
            src: Memory::label(label),
        })
        .unwrap();
    assembler.emit(Instruction::Pop_Mem { src: mem }).unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[rstest]
#[case(true, &[0xCD, 0x00, 0xF1, 0xCD, 0x02, 0xCC, 0xCD, 0x04, 0xCD, 0x05, 0xCD, 0x06, 0xCD, 0xFE, 0xCD, 0xFF, 0xC3])]
#[case(false, &[0xCD, 0x00, 0xF1, 0xCD, 0x02, 0xCC, 0xCD, 0x04, 0xCD, 0x05, 0xCD, 0x06, 0xCD, 0xFE, 0xCD, 0xFF, 0xC3])]
fn test_int_imm(#[case] with_relaxation: bool, #[case] expected: &[u8]) {
    let mut assembler = X86_64AssemblerBuilder::new().with_relaxation(with_relaxation).build();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(0),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(2),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(3),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(4),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(5),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(6),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(254),
        })
        .unwrap();
    assembler
        .emit(Instruction::Int_Imm {
            src: Immediate32::new(255),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}

#[test]
fn test_predefined_labels() {
    let predefined_label = Label::new();
    let predefined_label_position = HashMap::from([(predefined_label, -15)]);
    let mut assembler = X86_64AssemblerBuilder::new()
        .with_predefined_labels(&predefined_label_position)
        .build();

    assembler
        .emit(Instruction::Jump_Label { dst: predefined_label })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert!(result.public_labels_positions().is_empty());
    let expected = &[0xEB, 0xEF, 0xC3];
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len() as i32);
}
