use core::sync::atomic::{AtomicU32, Ordering};

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
