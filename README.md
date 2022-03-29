# Buildstructor

Derive a builder from a constructor!

Use this if you want a derived builder but with less annotation magic.

-----

## Installation:

### As a [crate](http://crates.io)

********

```toml
[dependencies]
buildstructor = "*"
```

### Compile yourself:

1. Install [Rust and cargo](http://doc.crates.io/)
2. git clone https://github.com/BrynCooke/buildstructor
3. Library: cd buildstructor && cargo build --release --lib
4. You can find the library in target/release

## Usage / Example:

1. Import the `builder` macro.
2. Annotate your `impl` containing a `new` function. Use your automatically derived builder.

```rust
use buildstructor::builder;

pub struct MyStruct {
    sum: usize,
}

#[builder]
impl MyStruct {
    fn new(a: usize, b: usize) -> MyStruct {
        Self { sum: a + b }
    }
}

fn main() {
    let mine = MyStruct::builder().a(2).b(3).build();
    assert_eq!(mine.sum, 5);
}
```

## Motivation

The difference between this and other builder crates is that constructors are used to derive builders rather than structs. This should result in a more natural fit with regular Rust code rather than relying on annotation magic to define behavior.

Advantages:

* You can specify fields in your constructor that do not appear in your struct.
* No magic to default values, just use an `Option` param in your constructor and default as normal.
* `async` constructors derives `async` builders.
* Fallible constructors (`Result`) derives Fallible builders.
* Special `Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet` support. Add single or multiple items.

This crate is heavily inspired by the excellent [typed-builder](https://github.com/idanarye/rust-typed-builder) crate. It is a good alternative to this crate and well worth considering.

## Recipes

All of these recipes and more can be found in the [examples directory](https://github.com/BrynCooke/buildstructor/tree/main/examples)

Just write your rust code as usual and annotate the constructor impl with `[builder]`

### Optional field

Fields that are optional will also be optional in the builder. You should do defaulting in your constructor.

```rust
#[builder]
impl MyStruct {
    fn new(param: Option<usize>) -> MyStruct {
        Self { param: param.unwrap_or(3) }
    }
}

fn main() {
    let mine = MyStruct::builder().simple(2).build();
    assert_eq!(mine.param, 2);
    let mine = MyStruct::builder().build();
    assert_eq!(mine.param, 3);
}
```

### Into field

You can use generics as usual in your constructor.

```rust
#[builder]
impl MyStruct {
    fn new<T: Into<String>>(param: T) -> MyStruct {
        Self { param }
    }
}

fn main() {
    let mine = MyStruct::builder().simple("Hi").build();
    assert_eq!(mine.param, "Hi");
}
```

### Async

To create an `async` builder just make your constructor `async`.

```rust
#[builder]
impl MyStruct {
    async fn new(param: usize) -> MyStruct {
        Self { param }
    }
}

async fn main() {
    let mine = MyStruct::builder().simple(2).build().await;
    assert_eq!(mine.param, 2);
}
```

### Fallible

To create a fallible builder just make your constructor fallible using `Result`. 

```rust
#[builder]
impl MyStruct {
    fn new(param: Option<usize>) -> Result<MyStruct, Box<dyn Error>> {
        Ok(Self { param })
    }
}

fn main() {
    let mine = MyStruct::builder().simple(2).build().unwrap();
    assert_eq!(mine.param, 2);
}
```

### Collections

`Vec`, `HashMap`, `HashSet`, `BTreeMap`, `BTreeSet` parameters are treated specially. Use the plural form in your constructor argument and `buildstructor` will automatically try to figure out the singular form for individual entry.

In the case that a singular form cannot be derived automatically the suffix `_entry` will be used.

```rust
#[builder]
impl MyStruct {
    fn new(addresses: Vec<String>) -> MyStruct {
        Ok(Self { addresses })
    }
}

fn main() {
    let mine = MyStruct::builder()
        .address("Amsterdam")
        .address("Fakenham")
        .addresses(vec!["Norwich", "Bristol"])
        .build();
    assert_eq!(mine.simple, 2);
}
```

There had to be some magic somewhere.

## TODO

* Transfer docs from the constructor to the generated builder methods.
* Better error messages.
* More testing.

PRs welcome!

## License

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as
defined in the Apache-2.0 license, shall be licensed as above, without any additional terms or conditions.
