# Guessing Game

Small CLI game - guess the number between 1 and 100

To run: `cargo run` (build and execute)

Learnings:
* `use` statements (bring in any spicy types)
* `mut` for mutable variables (they are immutable by default)
* `<type>::<fn>()` The function is associated with the _type_, rather than an instance of it
* `<instance>.<fn>()` Calling a method on an instance
* `new` is a function on many types for creating a new value
* `&<arg>` means the argument is a reference (immutable by default)
* `&mut <arg>` for a mutable reference
* `Result` type (there is a generic one and submodule specific ones)
  * Are _enums_ (fixed set of values - variants). For Result: `Ok` (contains result) or `Err` (contains error info)
  * A `Result` instance has a `.expect("Display error message")` method
    * If the instance is an `Err`, `.expect` crashes the app and displays the message passed to it
    * If the instance is an `Ok`, `.expect` pulls the result value from `Ok` and returns it
* To install a dependency (a crate), add it to the `[dependencies]` in `Cargo.toml`
  * `rand = "0.5.5"` (Actually saying `"^0.5.5"`)
  * `cargo build` will look for changes in deps and install
  * `Cargo.lock` is created with exact dependency versions for future installs until you update explicitly
  * `cargo update` will check for updates to patch versions
  * Change the `Cargo.toml` file to update minor or major versions
* Assigning types upon declaration: `let num: u32 = 5`
* `match` (kind of like a switch statement) - with run the `arm` that defined pattern matches for
* `.cmp` method - can be called on anything that can be compared and passed a reference to what you want to compare with
* `println!("Place{} example", "holder")` ...though usually holder would be a variable lol
