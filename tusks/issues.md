# Tusks — Known Issues / Bugs

## 1. `use` imports not visible in function signatures

**Symptom:** A `use` statement placed directly before a function inside a `#[tusks()]` module,
or at module level inside `pub mod cli { ... }`, is not recognized for type resolution in
function signatures:

```rust
use crate::datetime::LocalDateTime; // ← not visible in signature below
pub fn summary(from: Option<LocalDateTime>) -> u8 { ... } // E0425
```

`super::LocalDateTime` also fails — tusks seems to generate code in a context
where `super` does not resolve as expected:

```
error[E0425]: cannot find type `LocalDateTime` in module `super`
```

**Workaround:** Use the full `crate::` path directly in the signature:

```rust
pub fn summary(from: Option<crate::datetime::LocalDateTime>) -> u8 { ... }
```

## 2. `Clone` required on argument types

**Symptom:** Custom types used as command arguments must implement `Clone`, otherwise:

```
the trait bound `LocalDateTime: Clone` is not satisfied [E0277]
required by a bound introduced by this call [E0277]
```

**Workaround:** Add `#[derive(Clone)]` to the type:

```rust
#[derive(Clone)]
pub struct LocalDateTime {
    pub utc: chrono::DateTime<chrono::Utc>,
}
```

This is not documented in tusks and is a non-obvious requirement — clap itself
does not require `Clone` on argument types when using `FromStr`.
