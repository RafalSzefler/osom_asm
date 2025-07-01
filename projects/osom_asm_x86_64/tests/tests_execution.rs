#![cfg(target_arch = "x86_64")]
use osom_asm_x86_64::{
    assembler::X86_64Assembler,
    models::{Condition, GPR, Immediate32, Immediate64, Instruction, Label, Memory},
};

use osom_tools_dev::macros::{convert_to_fn, convert_to_fn_with_offset};

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
            src: Immediate32::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream);
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
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::Jump_Label { dst: label }).unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(0),
        })
        .unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream);
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
    let _ = assembler.assemble(&mut stream).unwrap();

    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i32);
    assert_eq!(unsafe { fn_ptr() }, value);
}

#[rstest]
#[case(5)]
#[case(-300)]
#[case(0)]
#[case(123456)]
fn test_pass_and_return(#[case] value: i32) {
    let mut assembler = X86_64Assembler::new(true);
    assembler
        .emit(Instruction::Mov_RegReg {
            dst: GPR::RAX,
            src: GPR::RDI,
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();

    let fn_ptr = convert_to_fn!("sysv64", stream, fn(i32) -> i32);
    assert_eq!(unsafe { fn_ptr(value) }, value);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_cmp_reg_imm(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);
    let label = Label::new();
    assembler
        .emit(Instruction::Xor_RegReg {
            dst: GPR::RAX,
            src: GPR::RAX,
        })
        .unwrap();
    assembler
        .emit(Instruction::Cmp_RegImm {
            dst: GPR::RDI,
            src: Immediate32::new(0),
        })
        .unwrap();
    assembler
        .emit(Instruction::CondJump_Label {
            condition: Condition::GreaterOrEqual,
            dst: label,
        })
        .unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();

    let fn_ptr = convert_to_fn!("sysv64", stream, fn(i64) -> i64);

    assert_eq!(unsafe { fn_ptr(i64::MIN) }, 1);
    assert_eq!(unsafe { fn_ptr(i64::MAX) }, 0);

    for i in -100..0 {
        assert_eq!(unsafe { fn_ptr(i) }, 1);
    }

    for i in 0..100 {
        assert_eq!(unsafe { fn_ptr(i) }, 0);
    }
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_jmp_reg(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    const VALUE: i64 = -100;

    let ptr: fn() -> i64 = || VALUE;
    let ptr_int = ptr as i64;

    assembler
        .emit(Instruction::Mov_RegImm64 {
            dst: GPR::R10,
            src: Immediate64::new(ptr_int),
        })
        .unwrap();
    assembler.emit(Instruction::Jump_Reg { dst: GPR::R10 }).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i64);
    assert_eq!(unsafe { fn_ptr() }, VALUE);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_jmp_mem(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    const VALUE: i64 = 1731;

    let ptr: fn() -> i64 = || VALUE;
    let ptr_int = ptr as i64;
    let ptr_int_address = &ptr_int as *const i64;
    let ptr_int_address_int = ptr_int_address as i64;

    assembler
        .emit(Instruction::Mov_RegImm64 {
            dst: GPR::R10,
            src: Immediate64::new(ptr_int_address_int),
        })
        .unwrap();
    assembler
        .emit(Instruction::Jump_Mem {
            dst: Memory::based(GPR::R10, Immediate32::ZERO).unwrap(),
        })
        .unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i64);
    assert_eq!(unsafe { fn_ptr() }, VALUE);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_call_reg(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    const VALUE: i64 = 555;

    let ptr: fn() -> i64 = || VALUE;
    let ptr_int = ptr as i64;

    assembler
        .emit(Instruction::Mov_RegImm64 {
            dst: GPR::R10,
            src: Immediate64::new(ptr_int),
        })
        .unwrap();
    assembler.emit(Instruction::Call_Reg { dst: GPR::R10 }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i64);
    assert_eq!(unsafe { fn_ptr() }, VALUE);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_call_mem(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    const VALUE: i64 = 666;

    let ptr: fn() -> i64 = || VALUE;
    let ptr_int = ptr as i64;
    let ptr_int_address = &ptr_int as *const i64;
    let ptr_int_address_int = ptr_int_address as i64;

    assembler
        .emit(Instruction::Mov_RegImm64 {
            dst: GPR::R10,
            src: Immediate64::new(ptr_int_address_int),
        })
        .unwrap();
    assembler
        .emit(Instruction::Call_Mem {
            dst: Memory::based(GPR::R10, Immediate32::ZERO).unwrap(),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();
    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i64);
    assert_eq!(unsafe { fn_ptr() }, VALUE);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_call_label(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    let label = Label::new();

    assembler.emit(Instruction::Call_Label { dst: label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(-3),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let _ = assembler.assemble(&mut stream).unwrap();

    let fn_ptr = convert_to_fn!("sysv64", stream, fn() -> i32);
    assert_eq!(unsafe { fn_ptr() }, -3);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_call_with_public_label_and_backwards(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    const RESULT: i32 = 17;

    let label = Label::new();
    let entry = Label::new();

    assembler.emit(Instruction::SetPrivate_Label { label }).unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(RESULT),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    assembler.emit(Instruction::SetPublic_Label { label: entry }).unwrap();
    assembler.emit(Instruction::Call_Label { dst: label }).unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let result = assembler.assemble(&mut stream).unwrap();

    assert_eq!(result.public_labels_positions().len(), 1);

    let offset = *result.public_labels_positions().get(&entry).unwrap();
    let fn_ptr = convert_to_fn_with_offset!("sysv64", stream, offset, fn() -> i32);
    assert_eq!(unsafe { fn_ptr() }, RESULT);
}

#[rstest]
#[case(false)]
#[case(true)]
fn test_fibonacci(#[case] with_relaxation: bool) {
    let mut assembler = X86_64Assembler::new(with_relaxation);

    let entry = Label::new();
    let recursion_entry = Label::new();
    let initial_skip = Label::new();

    assembler.emit(Instruction::SetPublic_Label { label: entry }).unwrap();
    assembler
        .emit(Instruction::Cmp_RegImm {
            dst: GPR::RDI,
            src: Immediate32::ZERO,
        })
        .unwrap();
    assembler
        .emit(Instruction::CondJump_Label {
            condition: Condition::GreaterOrEqual,
            dst: recursion_entry,
        })
        .unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(-1),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    assembler
        .emit(Instruction::SetPrivate_Label { label: recursion_entry })
        .unwrap();
    assembler
        .emit(Instruction::Cmp_RegImm {
            dst: GPR::RDI,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler
        .emit(Instruction::CondJump_Label {
            condition: Condition::Greater,
            dst: initial_skip,
        })
        .unwrap();
    assembler
        .emit(Instruction::Mov_RegImm {
            dst: GPR::RAX,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();
    assembler
        .emit(Instruction::SetPrivate_Label { label: initial_skip })
        .unwrap();
    assembler
        .emit(Instruction::Sub_RegImm {
            dst: GPR::RDI,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::Push_Reg { src: GPR::RDI }).unwrap();
    assembler
        .emit(Instruction::Call_Label { dst: recursion_entry })
        .unwrap();
    assembler.emit(Instruction::Pop_Reg { src: GPR::RDI }).unwrap();
    assembler
        .emit(Instruction::Sub_RegImm {
            dst: GPR::RDI,
            src: Immediate32::new(1),
        })
        .unwrap();
    assembler.emit(Instruction::Push_Reg { src: GPR::RAX }).unwrap();
    assembler
        .emit(Instruction::Call_Label { dst: recursion_entry })
        .unwrap();
    assembler.emit(Instruction::Pop_Reg { src: GPR::RDI }).unwrap();
    assembler
        .emit(Instruction::Add_RegReg {
            dst: GPR::RAX,
            src: GPR::RDI,
        })
        .unwrap();
    assembler.emit(Instruction::Ret).unwrap();

    let mut stream = RegionStream::new();
    let result = assembler.assemble(&mut stream).unwrap();

    assert_eq!(result.public_labels_positions().len(), 1);

    let offset = *result.public_labels_positions().get(&entry).unwrap();
    let fn_ptr = convert_to_fn_with_offset!("sysv64", stream, offset, fn(i64) -> i64);

    fn fibonacci(n: i64) -> i64 {
        if n < 0 {
            return -1;
        }

        if n <= 1 {
            return 1;
        }

        fibonacci(n - 1) + fibonacci(n - 2)
    }

    for i in -100..20 {
        assert_eq!(unsafe { fn_ptr(i) }, fibonacci(i));
    }
}
