macro_rules! fragment_at_index {
    ($asm:expr, $index:expr) => {{
        unsafe {
            &*(($asm)
                .fragments
                .as_ptr()
                .add($index as usize)
                .cast::<crate::assembler::implementation::fragment::Fragment>())
        }
    }};
}

pub(super) use fragment_at_index;

macro_rules! fragment_at_index_mut {
    ($asm:expr, $index:expr) => {
        unsafe {
            &mut *(($asm)
                .fragments
                .as_mut_ptr()
                .add($index as usize)
                .cast::<crate::assembler::implementation::fragment::Fragment>())
        }
    };
}

pub(super) use fragment_at_index_mut;

macro_rules! fragment_end {
    ($asm:expr) => {
        unsafe {
            crate::assembler::implementation::macros::fragment_at_index!($asm, $asm.last_fragment_offset)
                .next()
                .cast_const()
        }
    };
}

pub(super) use fragment_end;
