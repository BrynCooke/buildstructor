# Buildstructor

Derive a builder from methods/constructors using the typesafe builder pattern!

Use this if your method/constructor has:
* Optional parameters.
* A large number of parameters.
* Collections parameters.

## Installation:

Add the dependency to your `Cargo.toml`
```toml
[dependencies]
buildstructor = "*"
```

## Usage / Example:

1. Annotate your `impl` with `#[buildstructor::buildstructor]`.
2. Annotate your `impl` with `#[builder]`.
3. Use your automatically derived builder.

```rust
pub struct MyStruct {
    sum: usize,
}

#[buildstructor::buildstructor]
impl MyStruct {
    #[builder]
    fn new(a: usize, b: usize) -> MyStruct {
        Self { sum: a + b }
    }
    
    #[builder(entry = "more", exit = "add")]
    fn add_more(&mut self, c: usize, d: usize, e: Option<usize>) {
        self.sum += c + d + e.unwrap_or(3);
    }
}

fn main() {
    let mut mine = MyStruct::builder().a(2).b(3).build();
    assert_eq!(mine.sum, 5);
    mine.more().c(1).d(2).add();
    assert_eq!(mine.sum, 11);
}
```

## Motivation

The difference between this and other builder crates is that methods/constructors are used to derive builders rather than structs. This results in a more natural fit with regular Rust code, and no annotation magic to define behavior.

Advantages:

* You can specify fields in your constructor that do not appear in your struct.
* No magic to default values, just use an `Option` param in your methods/constructors and default as normal.
* `async` constructors derives `async` builders.
* Fallible constructors (`Result`) derives fallible builders.
* Special `Vec`, `Deque`, `Heap`, `Set`, `Map` support. Add single or multiple items.
* Generated builders can have receiver, `self`, `&self` and `&mut self` are supported.

This crate is heavily inspired by the excellent [typed-builder](https://github.com/idanarye/rust-typed-builder) crate. It is a good alternative to this crate and well worth considering.

## Recipes

All of these recipes and more can be found in the [examples directory](https://github.com/BrynCooke/buildstructor/tree/main/examples)

Just write your rust code as usual and annotate the constructor impl with `[builder]`

### Mutliple builders
All methods that are annotated with `#[builder]` will create builders. Each builder is named appropriately.
```rust
use std::error::Error;

struct Multi {
    simple: usize
}

#[buildstructor::buildstructor]
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

### Builders with a receiver
Builders can be generated on methods that take `self`, `&self` and `&mut self` as a parameter. This is useful for APIs where you have a large number of parameters some of which may be optional.

```rust
use buildstructor::buildstructor;

#[derive(Default)]
pub struct Client;

#[buildstructor]
impl Client {
    #[builder(entry = "query", exit = "call")]
    fn do_query(self, _simple: String) -> bool {
        true
    }

    #[builder(entry = "query_ref", exit = "call")]
    fn do_query_ref(&self, _simple: String) -> bool {
        true
    }

    #[builder(entry = "query_ref_mut", exit = "call")]
    fn do_query_ref_mut(&mut self, _simple: String) -> bool {
        true
    }
}

fn main() {
    Client::default().query().simple("3".to_string()).call(); // self

    let client = Client::default();
    client.query_ref().simple("3".to_string()).call(); // &self

    let mut client = Client::default();
    client.query_ref_mut().simple("3".to_string()).call(); // &mut self
}
```

### Optional field

Fields that are `Option` will also be optional in the builder. You should do defaulting in your constructor.

```rust
struct MyStruct {
    param: usize
}

#[buildstructor::buildstructor]
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

Note that if a field is an `Option` or collection then if a user forgets to set it a compile error will be generated.

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
struct MyStruct {
    param: String   
}

#[buildstructor::buildstructor]
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
struct MyStruct {
    param: usize
}

#[buildstructor::buildstructor]
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
use std::error::Error;
struct MyStruct {
    param: usize
}

#[buildstructor::buildstructor]
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
struct MyStruct {
    addresses: Vec<String>
}

#[buildstructor::buildstructor]
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
