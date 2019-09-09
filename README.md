# kanji-number-parser

Convert Chinese numeral `String` to `BigUInt`.

## Getting Started

Simply add to your Cargo.toml:

```toml
[dependencies]
kanji-number-parser = "0.1.0"
```

## Example

```rust
use kanji_number_parser::parse;
fn main(){
    println!("一億五千万: {}", parse(String::from("一億五千万")).unwrap());
    // => 一億五千万: 150000000
    println!("十二無量大数: {}", parse(String::from("十二無量大数")).unwrap());
    // => 十二無量大数: 1200000000000000000000000000000000000000000000000000000000000000000000
}
```
