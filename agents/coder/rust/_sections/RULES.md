<!-- Rust-specific rules -->

### Workflow

- Run `cargo check` after every change to catch compilation errors early.
- After a task is complete, run `cargo test` to verify nothing is broken.

### Style Guide

#### Module Organization

`mod.rs` files should only contain `pub` declarations — no implementation logic.

```rust
// ✅ Good: mod.rs
pub mod parser;
pub mod validator;

// ❌ Bad: mod.rs with logic
pub fn process_data() { /* implementation */ }
```

#### Test Grouping

Group similar tests using `mod` blocks.

#### File Granularity

Favor multiple small, focused files over monolithic ones.

- ✅ `user_validator.rs`, `user_sanitizer.rs`, `user_formatter.rs`
- ❌ `user_utils.rs` with 1000+ lines

#### File Headers

Start each file with a one-line comment explaining its purpose. If editing a file and purpose changes, update it.

```rust
// HTTP client wrapper with retry logic and error handling
```
