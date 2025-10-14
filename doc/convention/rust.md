# Naming

```rust
// inside user.rs
pub fn get_user() -> User {}  // ❌ redundant

// already know the module is user `user::get()`
pub fn get() -> User {}      // ✅
pub fn from() -> User {}

// `user::user()`
pub fn user() -> User {}       // ❌ redundant

pub fn apple() -> Apple {}    // ✅ clean, declarative

pub fn set_user() {}          // ✅ Imperative, clear intention to do something, not return something even if it does
pub fn mint() {}              // ✅
pub fn revoke_role() {}       // ✅
```


# Order
```rust
#[macro_export]
macro_rules! foo {}
macro_rules! bar {}

pub trait Foo {}
pub trait Bar {}
trait Buz {}

pub type Result<T> = ::std::result::Result<T, Error>;

#[repr(u8)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Error {
    UnreachableBar,
    MissingFoo
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Core<A, B>
where
    A: Foo,
    B: Bar {
    pub foo: A,
    pub bar: B    
}

impl<A, B> Core<A, B>
where
    A: Foo,
    B: Bar {
    pub fn new(foo: A, bar: B) -> Self {
        Self {
            foo,
            bar
        }
    }
}

impl<A, B> Default for Core<A, B> {
    
}

impl<A, B> From<(A, B)> for Core<A, B>
where
    A: Foo,
    B: Bar {
        
}

pub fn foo() {}
pub fn bar() {}
fn buz() {}
```