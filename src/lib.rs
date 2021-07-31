/*!
```rust
println!("Hello from doctest!");
```
*/
pub fn nonmain() {
    println!("Hello, world!");
}

#[test]
fn test() {
    println!("Hello from unit test");
}
