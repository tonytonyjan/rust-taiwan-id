# Taiwan ID

## Usage

```rust
// verify
assert!(taiwan_id::is_valid("A123456789"))

// generate a random ID
taiwan_id::generate()

// generate a random ID for Taipei City
taiwan_id::generate_prefix("A")

// generate a random female ID for Taipei City
taiwan_id::generate_prefix("A2")
```
