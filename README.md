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
fn main() {
    // Create some data.
    let ddata0 = vec![1, 2, 2, 2, 2, 3, 4, 5, 5, 5, 5, 5, 5];

    // Create other example parameters.
    let flags = 0;              // Must be zero.
    let max_window_size = 1024;
    let dictionary_size = 0;
    let progress = |original_size: i64, compressed_size: i64| {
        println!("Original size: {}, Compressed Size: {}", original_size, compressed_size);
    };
    let mut stats = apultra::Stats::default();

    // Compress data.
    let cdata_res = apultra::compress(
        &ddata0,
        flags,
        max_window_size,
        dictionary_size,
        Some(Box::new(progress)), // Pass callback closure. None can also be passed.
        Some(&mut stats),         // Pass stats structure. None can also be passed.
    );

    // Check.
    let cdata0 = match cdata_res {
        | Err(err) => {
            println!("Error: {}", err);
            return;
        }
        | Ok(cdata) => cdata,
    };

    // Decompress data.
    let ddata_res = apultra::decompress(&cdata0, dictionary_size, flags);

    // Check.
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
