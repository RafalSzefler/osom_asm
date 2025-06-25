#![cfg(target_arch = "x86_64")]
use osom_asm_x86_64::{
    assembler::X86_64Assembler,
    models::{GPR, Immediate, Instruction, Label},
};

mod region_stream;
use region_stream::{RegionStream, as_sysv_fn};

use rstest::rstest;

#[rstest]
#[case(false)]
#[case(true)]
fn test_simple_execution(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    assembler.assemble(&mut stream).unwrap();
    let fn_ptr = as_sysv_fn!(stream);
    assert_eq!(fn_ptr(), 0);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_with_jumps(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::Jump_Label { dst: label }).unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    assembler.assemble(&mut stream).unwrap();
    let fn_ptr = as_sysv_fn!(stream);
    assert_eq!(fn_ptr(), 1);
}
