## Rust Conventions
- Use `snake_case` for the file names
- Identitation are 4 spaces, not a tab
- Rust uses ; at the end of the line to signal the end of a expression
- `const` should be all SNAKE_UPPER_CASED
- `for` loops over others

### Match expression
A `match` expression is made up of `arms`. An `arm` concists of a patterns to match against. 

Since `match` is an expression, you can pretty much run any commands beforehand. You just need to capture an enum to handle whatever is the return.
### Compilation and Running are two separate steps
`rustc main.rs` will compile the rust code into the output `main`, which then can be run

`cargo check` is a good command to run every time you change your code to ensure that the code will compile without actually compiling it.

### Cargo
Cargo is Rust's build system and package manager. It also has some commands to help with development like `cargo new hello_cargo`

The default behaviour for versioning in cargo is ^0.0.0


#### Libraries
Cargo libraries can eiher be `binary crates` or `library crates`. The last one are libraries that intented to be used in other programs and can't be executed by themselves.

### STD Libraries
#### Prelude
By default, Rust has a set of items defined in the STD Library that it brings in every program. Those are called Prelude