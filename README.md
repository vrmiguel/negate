# negate

Attribute macro that generates `is_not_something` from `is_something` functions.

## Examples

```rust
#[negate]
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}
```

The macro will expand to this:

```rust
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}
pub fn is_not_even(x: i32) -> bool {
    !(is_even(x))
}
```

```rust
#[negate]
fn is_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    x == y
}
```

The generated code is:

```rust
fn is_not_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    !(is_equal(x, y))
}
```