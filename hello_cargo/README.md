Create a new project with `cargo new <project_name>`

To run (again, you're going to need Rust, and Cargo, installed...):

```sh
# compile (outputs code from src folder to target/debug/<project_name>)
cargo build

# run the executable
./target/debug/hello_cargo
```

Other useful commands:
* `cargo run`: compile and run in one fell swoop (and if the `.rs` file hasn't changed, it will just run the executable!)
* `cargo check`: check the code compiles without creating the executable
* `cargo build --release`: compiles with optimisations - executable will be in `target/release` rather than `target/debug`
