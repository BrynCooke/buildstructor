# Buildstructor

Derive a builder from a constructor!

Use this if you want a derived builder but with less annotation magic.

## Installation:

Add the dependency to your `Cargo.toml`
```toml
[dependencies]
buildstructor = "*"
```

## Usage / Example:

1. Import the `builder` macro.
2. Annotate your `impl` containing a `new` function. 
3. Use your automatically derived builder.

```rust
use buildstructor::buildstructor;

pub struct MyStruct {
    sum: usize,
}

#[buildstructor]
impl MyStruct {
    #[builder]
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

The difference between this and other builder crates is that constructors are used to derive builders rather than structs. This results in a more natural fit with regular Rust code, and no annotation magic to define behavior.

Advantages:

* You can specify fields in your constructor that do not appear in your struct.
* No magic to default values, just use an `Option` param in your constructor and default as normal.
* `async` constructors derives `async` builders.
* Fallible constructors (`Result`) derives fallible builders.
* Special `Vec`, `Deque`, `Heap`, `Set`, `Map` support. Add single or multiple items.

This crate is heavily inspired by the excellent [typed-builder](https://github.com/idanarye/rust-typed-builder) crate. It is a good alternative to this crate and well worth considering.

## Recipes

All of these recipes and more can be found in the [examples directory](https://github.com/BrynCooke/buildstructor/tree/main/examples)

Just write your rust code as usual and annotate the constructor impl with `[builder]`

### Mutliple constructors
All methods that are suffixed with `_new` will create builders. Each builder is named appropriately.
```rust
use buildstructor::buildstructor;
use std::error::Error;

struct Multi {
    simple: usize
}

#[buildstructor]
impl Multi {
    #[builder]
    fn new(simple: usize) -> Multi {
        Self { simple }
    }
    #[builder]
    fn try_new(simple: usize) -> Result<Multi, Box<dyn Error>> {
        Ok(Self { simple })
    }
    #[builder]
    fn maybe_new(simple: usize) -> Option<Multi> {
        Some(Self { simple })
    }
}

fn main() {
    let regular = Multi::builder().simple(2).build();
    assert_eq!(regular.simple, 2);

    let fallible = Multi::try_builder().simple(2).build().unwrap();
    assert_eq!(fallible.simple, 2);

    let option = Multi::maybe_builder().simple(2).build().unwrap();
    assert_eq!(option.simple, 2);
}

```

### Optional field

Fields that are optional will also be optional in the builder. You should do defaulting in your constructor.

```rust
use buildstructor::buildstructor;
struct MyStruct {
    param: usize
}

#[buildstructor]
impl MyStruct {
    #[builder]
    fn new(param: Option<usize>) -> MyStruct {
        Self { param: param.unwrap_or(3) }
    }
}

fn main() {
    let mine = MyStruct::builder().param(2).build();
    assert_eq!(mine.param, 2);
    let mine = MyStruct::builder().and_param(Some(2)).build();
    assert_eq!(mine.param, 2);
    let mine = MyStruct::builder().build();
    assert_eq!(mine.param, 3);
}
```

### Into field

#### Simple types
Types automatically have into conversion if:
* the type is not a scalar.
* the type has no generic parameters. (this may be relaxed later)
* the type is a generic parameter from the impl or constructor method.

This is useful for Strings, but also other types where you want to overload the singular build method. Create an enum that derives From for all the types you want to support and then use this type in your constructor.

#### Complex types
You can use generics as usual in your constructor. However, this has the downside of not being able to support optional fields.

```rust
use buildstructor::buildstructor;
struct MyStruct {
    param: String   
}

#[buildstructor]
impl MyStruct {
    #[builder]
    fn new<T: Into<String>>(param: T) -> MyStruct {
        Self { param: param.into() }
    }
}

fn main() {
    let mine = MyStruct::builder().param("Hi").build();
    assert_eq!(mine.param, "Hi");
}
```

### Async

To create an `async` builder just make your constructor `async`.

```rust
use buildstructor::buildstructor;
struct MyStruct {
    param: usize
}

#[buildstructor]
impl MyStruct {
    #[builder]
    async fn new(param: usize) -> MyStruct {
        Self { param }
    }
}

#[tokio::main]
async fn main() {
    let mine = MyStruct::builder().param(2).build().await;
    assert_eq!(mine.param, 2);
}
```

### Fallible

To create a fallible builder just make your constructor fallible using `Result`. 

```rust
use buildstructor::buildstructor;
use std::error::Error;
struct MyStruct {
    param: usize
}

#[buildstructor]
impl MyStruct {
    #[builder]
    fn new(param: usize) -> Result<MyStruct, Box<dyn Error>> {
        Ok(Self { param })
    }
}

fn main() {
    let mine = MyStruct::builder().param(2).build().unwrap();
    assert_eq!(mine.param, 2);
}
```

### Collections and maps

Collections and maps are given special treatment, the builder will add additional methods to build the collection one element at a time.


```rust
use buildstructor::buildstructor;
struct MyStruct {
    addresses: Vec<String>
}

#[buildstructor]
impl MyStruct {
    #[builder]
    fn new(addresses: Vec<String>) -> MyStruct {
        Self { addresses }
    }
}

fn main() {
    let mine = MyStruct::builder()
        .address("Amsterdam".to_string())
        .address("Fakenham")
        .addresses(vec!["Norwich".to_string(), "Bristol".to_string()])
        .build();
    assert_eq!(mine.addresses, vec!["Amsterdam".to_string(), 
                                    "Fakenham".to_string(), 
                                    "Norwich".to_string(), 
                                    "Bristol".to_string()]);
}
```

#### Supported types
Collections are matched by type name:

| Type Name | Method used to insert |
|-----------|-----------------------|
| ...Buffer | push(_)               |
| ...Deque  | push(_)               |
| ...Heap   | push(_)               |
| ...Set    | insert(_)             |
| ...Stack  | push(_)               |
| ...Map    | insert(_, _)          |
| Vec       | push(_)               |

If your type does not conform to these patterns then you can use a type alias to trick buildstructor into giving the parameter special treatment.

#### Naming

Use the plural form in your constructor argument and `buildstructor` will automatically try to figure out the singular form for individual entry. For instance:

`addresses` => `address`

In the case that a singular form cannot be derived automatically the suffix `_entry` will be used. For instance:

`frodo` => `frodo_entry` 

#### Into

Adding a singular entry will automatically perform an into conversion if:
* the type is not a scalar.
* the type has no generic parameters. (this may be relaxed later)
* the type is a generic parameter from the impl or constructor method. 

This is useful for Strings, but also other types where you want to overload the singular build method. Create an enum that derives From for all the types you want to support and then use this type in your constructor.

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
