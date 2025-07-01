use core::mem::size_of;

use super::{Condition, GPR, GPRKind, Immediate32, Immediate64, Instruction, Label, Memory, Scale, Size};

const _: () = const {
    // Checks some invariants about the size of the models.

    assert!(size_of::<GPR>() == 1, "GPR size must be 1 byte");
    assert!(size_of::<Option<GPR>>() == 1, "Option<GPR> size must be 1 byte");
    assert!(size_of::<GPRKind>() == 1, "GPRKind size must be 1 byte");
    assert!(size_of::<Option<GPRKind>>() == 1, "Option<GPRKind> size must be 1 byte");
    assert!(size_of::<Size>() == 1, "Size size must be 1 byte");
    assert!(size_of::<Option<Size>>() == 1, "Option<Size> size must be 1 byte");
    assert!(size_of::<Immediate32>() == 4, "Immediate size must be 4 bytes");
    assert!(size_of::<Immediate64>() == 8, "Immediate64 size must be 8 bytes");
    assert!(size_of::<Memory>() <= 8, "Memory size must be at most 8 bytes");
    assert!(size_of::<Label>() == 4, "Label size must be 4 bytes");
    assert!(size_of::<Scale>() == 1, "Scale size must be 1 byte");
    assert!(
        size_of::<Instruction>() <= 16,
        "Instruction size must be at most 16 bytes"
    );
    assert!(size_of::<Condition>() == 1, "Condition size must be 1 byte");
};
