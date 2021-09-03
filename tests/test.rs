use negate::*;

#[negate]
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

#[negate]
fn is_equal<T>(x: T, y: T) -> bool where T: Eq  {
    x == y
}

#[test]
fn is_equal_test_case() {
    assert!(is_equal(2, 2));
    assert!(is_not_equal(3, 2));
    assert!(!is_equal("Andrew", "Johnathan"));
    assert!(is_not_equal("Andrew", "Johnathan"));
}

#[test]
fn is_even_test_case() {
    assert!(is_even(2));
    assert!(!is_even(3));
    assert!(is_not_even(3));
    assert!(!is_not_even(4));
}
