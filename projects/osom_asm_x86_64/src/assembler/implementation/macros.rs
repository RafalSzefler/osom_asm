macro_rules! fragment_at_index {
    ($core_emitter:expr, $index:expr) => {
        unsafe {
            &*(($core_emitter).fragments
                .as_ptr()
                .add($index as usize)
                .cast::<crate::assembler::implementation::fragment::Fragment>())
        }
    };
}

pub(super) use fragment_at_index;

macro_rules! fragment_at_index_mut {
    ($core_emitter:expr, $index:expr) => {
        unsafe {
            &mut *(($core_emitter).fragments
                .as_mut_ptr()
                .add($index as usize)
                .cast::<crate::assembler::implementation::fragment::Fragment>())
        }
    };
}

pub(super) use fragment_at_index_mut;

macro_rules! fragment_end {
    ($core_emitter:expr) => {
        unsafe {
            crate::assembler::implementation::macros::fragment_at_index!($core_emitter, $core_emitter.last_fragment_offset).next().cast_const()
        }
    };
}

pub(super) use fragment_end;
