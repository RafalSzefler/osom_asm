use osom_asm_test_helpers::assert_eq_hex;

#[test]
fn test_asserts() {
    assert_eq_hex!([1, 2, 3], [1, 2, 3]);
}

#[test]
#[should_panic]
fn test_asserts_fail() {
    assert_eq_hex!([1, 2, 3], [1, 2, 5]);
}
