# TC Kimlik

Tc kimlik is a validator and utility tools for Turkish identification numbers.

### Validating

```rust
let  tc = String::from("40243834390");
tc_kimlik::validate(&tc) // returns true
```

### Generate valid TC Kimlik

```rust
extern crate tc_kimlik;
fn main() {
    println!("New tc kimlik: {}", tc_kimlik::generate());
}
```


