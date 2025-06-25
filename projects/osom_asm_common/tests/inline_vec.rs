use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use osom_asm_common::InlineVec;

#[test]
fn test_inlined_array() {
    const MAX: usize = 100;
    let mut vec = Vec::<u32>::with_capacity(MAX);
    let mut arr = InlineVec::<u32, 5>::new();

    for i in 0..MAX {
        let no = i as u32;
        vec.push(no);
        arr.push(no);
        assert_eq!(arr.len(), vec.len());
        assert_eq!(arr.as_slice(), vec.as_slice());
    }
}

fn test_drop<const N: usize>() {
    const MAX: usize = 100;

    #[derive(Debug, Clone)]
    struct Foo {
        counter: Arc<AtomicUsize>,
    }

    impl Drop for Foo {
        fn drop(&mut self) {
            self.counter.fetch_add(1, Ordering::SeqCst);
        }
    }

    let counter = Arc::new(AtomicUsize::new(0));

    let mut arr = InlineVec::<Foo, N>::new();

    for _ in 0..MAX {
        let foo = Foo {
            counter: counter.clone(),
        };
        arr.push(foo);
    }

    assert_eq!(counter.load(Ordering::SeqCst), 0);

    drop(arr);

    assert_eq!(counter.load(Ordering::SeqCst), MAX);
}

#[test]
fn test_inlined_array_drop() {
    test_drop::<1>();
    test_drop::<5>();
    test_drop::<25>();
    test_drop::<100>();
    test_drop::<250>();
    test_drop::<1000>();
}
