# negate

negate is a simple attribute macro that negates a given function.


## Usage

### `#[negate]`

Given a function of the form `is_*` that returns a boolean value, the macro will create a `is_not_*` function that negates the given function.


```rust
struct Word(&'static str);

impl Word {
    pub fn new(word: &'static str) -> Self {
        Self (word)
    }

    #[negate] // <- negate will implement a `is_not_uppercase` function!
    pub fn is_uppercase(&self) -> bool {
        self.0 == self.0.to_uppercase()
    }
}
let my_name = Word::new("My Name");

assert!(my_name.is_not_uppercase());
```
    

### `#[negate(name = "...")]`

Using the name attribute allows you to set the name of the generated function. This also allows the usage of the [negate] macro with functions that do not start with `is_`.

```rust
use negate::negate;

pub enum TaskState {
    Ready,
    Finished,
}

pub struct Reactor {
    tasks: HashMap<usize, TaskState>,
}

impl Reactor {
    // Generates the `is_finished` function
    #[negate(name = "is_finished")]
    pub fn is_ready(&self, id: usize) -> bool {
        self.tasks.get(&id).map(|state| match state {
            TaskState::Ready => true,
            _ => false,
        }).unwrap_or(false)
    }
}
```

### `#[negate(docs = "...")]`

Using the docs attribute allows you to customize the doc-string of the generated function.

```rust
use negate::negate;
#[negate(name = "is_odd", docs = "returns true if the given number is odd")]
fn is_even(x: i32) -> bool {
   x % 2 == 0
}
assert!(is_odd(5));
```


## How does the generated code look like?

### Non-associated functions

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

/// This is an automatically generated function that denies [`is_even`].
/// Consult the original function for more information.
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

The generated negated function:

```rust
/// This is an automatically generated function that denies [`is_equal`].
/// Consult the original function for more information.
fn is_not_equal<T>(x: T, y: T) -> bool
where
    T: Eq,
{
    !is_equal(x, y)
}
```

### Associated functions

```rust
struct BigInt {
    ..
};

impl BigInt {
    #[negate(name = "is_negative", docs = "Returns true if the number is negative and false if the number is zero or positive.")]
    pub fn is_positive(&self) -> bool { .. }
}
```

Becomes:

```rust
impl BigInt {
    pub fn is_positive(&self) -> bool { .. }
    
    /// Returns true if the number is negative and false if the number is zero or positive.
    pub fn is_negative(&self) -> bool {
        !self.is_positive()
    }
}
```