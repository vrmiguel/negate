use negate::negate;

#[negate]
fn is_even(x: i32) -> bool {
    x % 2 == 0
}

#[negate]
fn is_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    x == y
}

#[allow(dead_code)]
enum UnsignedInteger {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl UnsignedInteger {
    #[negate]
    pub fn is_zero(&self) -> bool {
        let self_as_u64: u64 = self.into();
        self_as_u64 == 0
    }
}

impl From<&UnsignedInteger> for u64 {
    fn from(int: &UnsignedInteger) -> Self {
        match int {
            UnsignedInteger::U8(u) => (*u).into(),
            UnsignedInteger::U16(u) => (*u).into(),
            UnsignedInteger::U32(u) => (*u).into(),
            UnsignedInteger::U64(u) => (*u).into(),
        }
    }
}

#[test]
fn is_equal_test_case() {
    assert!(is_equal(2, 2));
    assert!(is_not_equal(3, 2));
    assert!(!is_equal("Andrew", "Johnathan"));
    assert!(is_not_equal("Andrew", "Johnathan"));
}

#[test]
fn associated_fn_is_zero() {
    let int = UnsignedInteger::U32(5);
    assert!(!int.is_zero());
    assert!(int.is_not_zero());
}

#[test]
fn is_even_test_case() {
    assert!(is_even(2));
    assert!(!is_even(3));
    assert!(is_not_even(3));
    assert!(!is_not_even(4));

    let even_numbers: Vec<_> = (0..100).filter(|x| x % 2 == 0).collect();

    for num in even_numbers {
        assert!(is_even(num));
        assert!(!is_not_even(num));
    }
}
