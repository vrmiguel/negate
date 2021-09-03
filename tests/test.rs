use negate::*;

#[negate]
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

#[test]
fn is_even_test_case() {
    assert!(is_even(2));
    assert!(!is_even(3));
    assert!(is_not_even(3));
    assert!(!is_not_even(4));
}
