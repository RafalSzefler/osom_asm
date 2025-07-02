macro_rules! fragment_at_index {
    ($asm:expr, $index:expr) => {{
        let fragment;

        #[allow(unused_unsafe)]
        unsafe {
            fragment = &*(($asm)
                .fragments
                .as_ptr()
                .offset($index as isize)
                .cast::<crate::assembler::implementation::fragment::Fragment>());
        }

        fragment
    }};
}

pub(super) use fragment_at_index;

macro_rules! fragment_at_index_mut {
    ($asm:expr, $index:expr) => {{
        let fragment;

        #[allow(unused_unsafe)]
        unsafe {
            fragment = &mut *(($asm)
                .fragments
                .as_mut_ptr()
                .offset($index as isize)
                .cast::<crate::assembler::implementation::fragment::Fragment>());
        }

        fragment
    }};
}

pub(super) use fragment_at_index_mut;

macro_rules! fragment_end {
    ($asm:expr) => {{
        let fragment;

        #[allow(unused_unsafe)]
        unsafe {
            fragment = crate::assembler::implementation::macros::fragment_at_index!($asm, $asm.last_fragment_offset)
                .next()
                .cast_const();
        }

        fragment
    }};
}

pub(super) use fragment_end;
