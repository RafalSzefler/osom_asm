#![allow(non_camel_case_types)]

use core::num::NonZero;

use super::{Condition, GPR, Immediate32, Label, Memory};

/// Represents custom assembly language instructions.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[must_use]
#[repr(u16)]
pub enum Instruction {
    /// `nop` extended to specified non-zero `length`
    Nop { length: NonZero<u32> } = 1, // We start from 1 because we want Option<Instruction> to be optimized.

    /// Pseudoinstruction: sets label at current position.
    /// "Private" means that the label won't be visible
    /// outside the compiled code.
    SetPrivate_Label { label: Label },

    /// Pseudoinstruction: sets label at current position.
    /// "Public" means that the label will be visible
    /// and reachable from outside the compiled code.
    SetPublic_Label { label: Label },

    /// `ret`
    Ret,

    /// `cpuid`
    Cpuid,

    /// `mov reg, imm64`
    ///
    /// # Notes
    /// The only instruction that uses 64-bit immediate.
    Mov_RegImm64 { dst: GPR, src: super::Immediate64 },

    /// `mov reg, imm`
    Mov_RegImm { dst: GPR, src: Immediate32 },

    /// `mov [mem], imm`
    Mov_MemImm { dst: Memory, src: Immediate32 },

    /// `mov reg, reg`
    Mov_RegReg { dst: GPR, src: GPR },

    /// `mov [mem], reg`
    Mov_MemReg { dst: Memory, src: GPR },

    /// `mov reg, [mem]`
    Mov_RegMem { dst: GPR, src: Memory },

    /// `cmp reg, imm`
    Cmp_RegImm { dst: GPR, src: Immediate32 },

    /// `cmp reg, reg`
    Cmp_RegReg { dst: GPR, src: GPR },

    /// `cmp [mem], imm`
    Cmp_MemImm { dst: Memory, src: Immediate32 },

    /// `cmp reg, [mem]`
    Cmp_RegMem { dst: GPR, src: Memory },

    /// `cmp [mem], reg`
    Cmp_MemReg { dst: Memory, src: GPR },

    /// `add reg, imm`
    Add_RegImm { dst: GPR, src: Immediate32 },

    /// `add [mem], imm`
    Add_MemImm { dst: Memory, src: Immediate32 },

    /// `add reg, reg`
    Add_RegReg { dst: GPR, src: GPR },

    /// `add [mem], reg`
    Add_MemReg { dst: Memory, src: GPR },

    /// `add reg, [mem]`
    Add_RegMem { dst: GPR, src: Memory },

    /// `sub reg, imm`
    Sub_RegImm { dst: GPR, src: Immediate32 },

    /// `sub [mem], imm`
    Sub_MemImm { dst: Memory, src: Immediate32 },

    /// `sub reg, reg`
    Sub_RegReg { dst: GPR, src: GPR },

    /// `sub [mem], reg`
    Sub_MemReg { dst: Memory, src: GPR },

    /// `sub reg, [mem]`
    Sub_RegMem { dst: GPR, src: Memory },

    /// `xor reg, imm`
    Xor_RegImm { dst: GPR, src: Immediate32 },

    /// `xor [mem], imm`
    Xor_MemImm { dst: Memory, src: Immediate32 },

    /// `xor reg, reg`
    Xor_RegReg { dst: GPR, src: GPR },

    /// `xor [mem], reg`
    Xor_MemReg { dst: Memory, src: GPR },

    /// `xor reg, [mem]`
    Xor_RegMem { dst: GPR, src: Memory },

    /// Jumps to label.
    ///
    /// # Notes
    ///
    /// Pseudoinstruction: it is compiled into RIP-relative jump
    /// at the machine level.
    Jump_Label { dst: Label },

    /// `jmp reg`
    Jump_Reg { dst: GPR },

    /// `jmp [mem]`
    Jump_Mem { dst: Memory },

    /// Calls label.
    ///
    /// # Notes
    ///
    /// Pseudoinstruction: it is compiled into RIP-relative call
    /// at the machine level.
    Call_Label { dst: Label },

    /// `call reg`
    Call_Reg { dst: GPR },

    /// `call [mem]`
    Call_Mem { dst: Memory },

    /// Conditional jump to label.
    ///
    /// # Notes
    ///
    /// Pseudoinstruction: it is compiled into RIP-relative jump
    /// at the machine level.
    CondJump_Label { condition: Condition, dst: Label },

    /// `push imm`
    Push_Imm { src: Immediate32 },

    /// `push reg`: note that [`GPR`] has to be 64-bit.
    Push_Reg { src: GPR },

    /// `push [mem]`
    Push_Mem { src: Memory },

    /// `pop reg`
    ///
    /// # Notes
    ///
    /// The `src` value has to be a 64-bit [`GPR`].
    Pop_Reg { src: GPR },

    /// `pop [mem]`
    Pop_Mem { src: Memory },
}
