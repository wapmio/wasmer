# `wasmer-js` [![Build Status](https://github.com/wasmerio/wasmer/workflows/build/badge.svg?style=flat-square)](https://github.com/wasmerio/wasmer/actions?query=workflow%3Abuild) [![Join Wasmer Slack](https://img.shields.io/static/v1?label=Slack&message=join%20chat&color=brighgreen&style=flat-square)](https://slack.wasmer.io) [![MIT License](https://img.shields.io/github/license/wasmerio/wasmer.svg?style=flat-square)](https://github.com/wasmerio/wasmer/blob/master/LICENSE) [![crates.io](https://img.shields.io/crates/v/wasmer-js.svg)](https://crates.io/crates/wasmer-js)

[`Wasmer`](https://wasmer.io/) is the most popular
[WebAssembly](https://webassembly.org/) runtime for Rust (...and also
the fastest). This runtime is an adapted version of the Wasmer API that compiles to
WebAssembly via `wasm-bindgen`.

`wasmer-js` uses the same WebAssembly runtime of your environment (browser or Node.js).

## Usage

```rust
use wasmer_js::{Store, Module, Instance, Value, imports};

fn main() -> anyhow::Result<()> {
    let module_wat = r#"
    (module
    (type $t0 (func (param i32) (result i32)))
    (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
        get_local $p0
        i32.const 1
        i32.add))
    "#;

    let store = Store::default();
    let module = Module::new(&store, &module_wat)?;
    // The module doesn't import anything, so we create an empty import object.
    let import_object = imports! {};
    let instance = Instance::new(&module, &import_object)?;

    let add_one = instance.exports.get_function("add_one")?;
    let result = add_one.call(&[Value::I32(42)])?;
    assert_eq!(result[0], Value::I32(43));

    Ok(())
}
```

## Config flags

Wasmer has the following configuration flags:
* `wat` (enabled by default): It allows to read WebAssembly files in their text format.
  *This feature is normally used only in development environments*

# Build

You can use `wasm-pack` to build wasmer-js:

```
wasm-pack build --release
```

> The provided `wasmer_js.wasm` file should weight around 75kB when optmized via `wasm-opt` and stripped via `wasm-strip`, so it's relatively slim.

# Test

```
wasm-pack test --node
```

---

Made with ❤️ by the Wasmer team, for the community