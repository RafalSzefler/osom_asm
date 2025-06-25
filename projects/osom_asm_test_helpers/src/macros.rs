#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => {{
        let left_expr = $crate::_hidden::TmpSlicable::as_tmp_slice(&($left));
        let right_expr = $crate::_hidden::TmpSlicable::as_tmp_slice(&($right));
        if left_expr != right_expr {
            panic!(
                r#"assertion `left == right` failed
  left: {:?}
 right: {:?}"#,
                left_expr, right_expr
            )
        }
    }};
}

#[macro_export]
#[doc(hidden)]
macro_rules! _fn_signature {
    ($abi: literal, fn() -> $ret:ty) => {
        unsafe extern $abi fn() -> $ret
    };
    ($abi: literal, fn($arg_type:ty) -> $ret:ty) => {
        unsafe extern $abi fn($arg_type) -> $ret
    };
    ($abi: literal, fn($arg_type:ty $(,$arg_type2:ty)*) -> $ret:ty) => {
        unsafe extern $abi fn($arg_type $(,$arg_type2)*) -> $ret
    };
}

/// Transforms pointerable into a function pointer.
///
/// # Arguments
///
/// * `abi` - The ABI of the function, e.g. `"sysv64"`.
/// * `pointerable` - The pointerable to transform into a function pointer. This
///   is any object that implements `pub fn as_ptr(&self) -> *const u8` function.
/// * `tokens` - optional function signature, e.g. `fn(i32, bool) -> u8`.
#[macro_export]
macro_rules! as_abi_fn {
    ($abi: literal, $pointerable:expr, $($tokens:tt)*) => {
        unsafe {
            let _ptr = ($pointerable).as_ptr() as *const u8;
            type _FnSignature = $crate::_fn_signature!($abi, $($tokens)*);
            std::mem::transmute::<*const u8, _FnSignature>(_ptr)
        }
    };
    ($abi: literal, $pointerable:expr) => {
        $crate::as_abi_fn!($abi, $pointerable, fn() -> i64)
    };
}
