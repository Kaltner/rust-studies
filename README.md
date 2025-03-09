## Onwership
[What is Onwership Documentation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

Variables are dropped from memory as soon as they are out of scope.

That means when you pass a variables that can be cloned (Stack variables) to a function, unless you use it as a reference or clone it, it will give the ownership to the function

Some variables that the size are know by compile time cannot be cloned, but they can be moved. Truple can be one of those, as long as each element of the Truple are not loaded in the Stack (Such as a String)

By using references (`&`), a function can borrow a variable value. However, that means the variable cannot be changed. To be able to change a reference, the original variable needs to be multable and the reference needs to be multable.

Another caveat is that when creating references, to avoid race conditions and such, only one multable reference can be created.

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

### No Null
Rust has no Null value. The closest it has is part of the Enum `Option` that has a possible value `None`.

The reasoning is that it creates a bunch of verifications and bugs because every single value of the language ends up in a scenario of "A value can be null or not-null" 

### Code Organization
Packages, Crates, Modules and use; Path
**Packages:** Cargo features that let you build, test and share crates (Libs). Can contain multiple binary crates, but usualy only one library crate. Needs at least one crate.
**Crates:** A tree of modules that produces a library or executable (binary). `
**Modules and use:** Let you control the organization, scope and privacy of paths
**Paths:** A way of naming an item, such as a struct, function or module

A package with both `lib.rs`  and `main.rs` should use the binary portion as consumer of the lib.rs
  
#### Guidelines for creating modules
- Start with a crate root
- Declare modules:
    - Inline 
    - In a file with the name of the module: Eg `src/garden.rs`
    - In a folder: Eg `src/garden/mod.rs`
- Declare sub-modules
    - Follow the same lineas as above
- Path to the public module anywhere in the code: `crate::garden::vegetables::Aspargus` 
- Private vs Public. `pub mod` vs `mod`
- `use` at the start of the files to use all around the files


#### Libraries
Cargo libraries can eiher be `binary crates` or `library crates`. The last one are libraries that intented to be used in other programs and can't be executed by themselves.

### Standard Libraries
#### Prelude
By default, Rust has a set of items defined in the STD Library that it brings in every program. Those are called Prelude

## Collections

Contain multiple values that can be stored in Heap, so it can easilly expand and shrink during the runtime.
- Vector (Stores a variable number of values)
- String (Collection of characters)
- Hash Map (Asssociate a value to a key. A map)

### Storing UTF-8 Encoded Text
