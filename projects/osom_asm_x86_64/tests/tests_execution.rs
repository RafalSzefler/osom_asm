#![cfg(target_arch = "x86_64")]
use osom_asm_x86_64::{
    assembler::X86_64Assembler,
    models::{GPR, Immediate, Instruction, Label, Memory},
};

use osom_asm_test_helpers::as_abi_fn;

mod utils;
use utils::region_stream::RegionStream;

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
    let fn_ptr = as_abi_fn!("sysv64", stream);
    assert_eq!(unsafe { fn_ptr() }, 0);
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
    let fn_ptr = as_abi_fn!("sysv64", stream);
    assert_eq!(unsafe { fn_ptr() }, 1);
}

#[rstest]
#[case(5)]
#[case(-300)]
#[case(0)]
#[case(123456)]
fn test_patchable_load(#[case] value: i32) {
    let mut assembler = X86_64Assembler::new(true);
    let label = Label::new();
    assembler
        .emit(Instruction::Mov_RegMem {
            dst: GPR::EAX,
            src: Memory::label(label),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();

    let value_bytes = value.to_le_bytes();
    assembler.emit(value_bytes).unwrap();

    let mut stream = RegionStream::new();
    assembler.assemble(&mut stream).unwrap();

    let fn_ptr = as_abi_fn!("sysv64", stream, fn() -> i32);
    assert_eq!(unsafe { fn_ptr() }, value);
}

#[rstest]
#[case(5)]
#[case(-300)]
#[case(0)]
#[case(123456)]
fn test_pass_and_return(#[case] value: i32) {
    let mut assembler = X86_64Assembler::new(true);
    let label = Label::new();
    assembler
        .emit(Instruction::Mov_RegReg {
            dst: GPR::RAX,
            src: GPR::RDI,
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    assembler.assemble(&mut stream).unwrap();

    let fn_ptr = as_abi_fn!("sysv64", stream, fn(i32) -> i32);
    assert_eq!(unsafe { fn_ptr(value) }, value);
}
