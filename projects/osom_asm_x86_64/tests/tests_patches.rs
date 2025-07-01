use osom_tools_dev::macros::assert_eq_hex;

use osom_asm_x86_64::assembler::X86_64Assembler;
use osom_asm_x86_64::models::{GPR, Immediate32, Instruction, Label, Memory};

#[test]
fn test_patchable_mov() {
    let expected = &[0x48, 0x31, 0xC0, 0x48, 0x8B, 0x15, 0xF6, 0xFF, 0xFF, 0xFF, 0xC3];
    let mut assembler = X86_64Assembler::new(true);
    let label = Label::new();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler
        .emit(Instruction::Mov_RegMem {
            dst: GPR::RDX,
            src: Memory::label(label),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len());
}

#[test]
fn test_patchable_mov_forward() {
    let expected = &[
        0x48, 0x8B, 0x15, 0x01, 0x00, 0x00, 0x00, 0xC3, 0x48, 0xC7, 0xC0, 0x01, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03,
    ];
    let mut assembler = X86_64Assembler::new(true);
    let label = Label::new();
    assembler
        .emit(Instruction::Mov_RegMem {
            dst: GPR::RDX,
            src: Memory::label(label),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit([1, 2, 3]).unwrap();

    let mut final_code = Vec::new();
    let result = assembler.assemble(&mut final_code).unwrap();
    assert_eq_hex!(final_code, expected);
    assert_eq!(result.emitted_bytes(), expected.len());
}
