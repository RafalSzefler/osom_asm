use core::sync::atomic::{AtomicU32, Ordering};

/// Represents a place in `X86_64` code that other instructions
/// can refer to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
#[repr(transparent)]
pub struct Label {
    value: u32,
}

static LABEL_COUNTER: AtomicU32 = AtomicU32::new(0);

#[must_use]
fn get_next_label_value() -> u32 {
    loop {
        let value = LABEL_COUNTER.load(Ordering::SeqCst);

        assert!(value != u32::MAX, "Label counter overflow.");

        if LABEL_COUNTER
            .compare_exchange(value, value + 1, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
        {
            return value;
        }
    }
}

impl Label {
    /// Creates a new label. Note that each call produces a new
    /// unique label, that won't be equal to any other label.
    ///
    /// # Panics
    ///
    /// Panics if the number of labels overflows. At the moment
    /// the limit is set to `u32::MAX` labels. Globally, during
    /// the execution of the program.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            value: get_next_label_value(),
        }
    }
}

impl Default for Label {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}
