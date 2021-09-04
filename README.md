# negate

Attribute macro that generates `is_not_something` from `is_something` functions.

## Examples

```rust
use negate::negate;

pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}

assert!(is_not_even(5));

struct Word(&'static str);

impl Word {
    pub fn new(word: &'static str) -> Self {
        Self (word)
    }

    #[negate]
    pub fn is_uppercase(&self) -> bool {
        self.0 == self.0.to_uppercase()
    }
}
let my_name = Word::new("My Name");

// We generated `is_not_uppercase`!
assert!(my_name.is_not_uppercase());
```

### How do these generated functions look like?

```rust
#[negate]
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}
```

Will expand to:

```rust
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}
pub fn is_not_even(x: i32) -> bool {
    !is_even(x)
}
```

Using generics is likewise not a problem

```rust
#[negate]
fn is_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    x == y
}
```

The generated negated code is:

```rust
fn is_not_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    !is_equal(x, y)
}
```

```rust
struct Word(&'static str);

impl Word {
    #[negate]
    pub fn is_uppercase(&self) -> bool {
        self.0 == self.0.to_uppercase()
    }
}
```

Becomes:

```rust
struct Word(&'static str);

impl Word {
    pub fn is_uppercase(&self) -> bool {
        self.0 == self.0.to_uppercase()
    }

    pub fn is_not_uppercase(&self) -> bool {
        !self.is_uppercase()
    }
}
```