Programatically navigate typescript AST for React components. The webassembly version and python bindings are also included in the timestone_python and timestone_wasm folders. 

We use the [Oxc](https://github.com/oxc-project/oxc) typescript parser to navigate the AST. Kudos to the Oxc team for making it super easy to use!. The following functionality is available:

- [x] Get list React components and subcomponents as a hierarchy
- [x] Get the list of imported components and libraries
- [x] Get the the start and end line of each component
- [ ] Get the list of props and prop types for each component

# Usage

## To add to your project

The package is still in early development and is not yet published to crates.io. You can use it by adding the following to your `Cargo.toml`:

```toml
[dependencies]
ts_react_ast = { git = "https://github.com/sarath-menon/tsx_morph.git" }
```

## To run examples

```
git clone https://github.com/sarath-menon/tsx_morph.git
cd tsx_morph
cargo run --example <example_name>
```