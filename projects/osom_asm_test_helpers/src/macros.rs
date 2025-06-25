#[macro_export]
macro_rules! assert_eq_hex {
    ($left:expr, $right:expr) => ({
        let left_expr = $crate::_hidden::TmpSlicable::as_tmp_slice(&($left));
        let right_expr = $crate::_hidden::TmpSlicable::as_tmp_slice(&($right));
        if left_expr != right_expr {
            panic!(r#"assertion `left == right` failed
  left: {:?}
 right: {:?}"#, left_expr, right_expr)
        }
    });
}
