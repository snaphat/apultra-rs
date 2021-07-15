# apultra-rs
[![Rust](https://github.com/snaphat/apultra-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/snaphat/apultra-rs/actions/workflows/rust.yml)

Rust bindings for [apultra](https://github.com/emmanuel-marty/apultra) compression/decompression.

## Example

Clone the repository:
```
git clone --recurse-submodules git@github.com:snaphat/apultra-rs.git
```

Add the following to your Cargo.toml:
```toml
[dependencies.apultra]
path = "apultra-rs"
```

Example Usage in main.rs:
```rust
use apultra;

fn main() {
    // Create some data.
    let ddata0 = vec![1, 2, 3, 4, 5];

    // Compress data.
    let cdata_res = apultra::compress(&ddata0, 0, 0, 0, None, None);
    let cdata0 = match cdata_res {
        | Err(err) => {
            println!("Error: {}", err);
            return;
        }
        | Ok(cdata) => cdata,
    };

    // Decompress data.
    let ddata_res = apultra::decompress(&cdata0, 0, 0);
    let ddata1 = match ddata_res {
        | Err(err) => {
            println!("Error: {}", err);
            return;
        }
        | Ok(ddata) => ddata,
    };

    // Verify result.
    assert_eq!(ddata0, ddata1);
}
```
