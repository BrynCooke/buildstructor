# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.3.1 - 2022-06-09

[#60](https://github.com/BrynCooke/buildstructor/issues/55)
Fix impl with concrete types in generics.

```
#[buildstructor]
impl Foo<usize> {
    #[builder]
    fn bound_new(simple: usize) -> Foo<usize> {
        Self { simple }
    }
}
```
Previously the generated builder method was not including concrete generic type. In this case `usize`.

## 0.3.0 - 2022-05-23

[#55](https://github.com/BrynCooke/buildstructor/issues/55)
Lifetimes are supported now.

[#52](https://github.com/BrynCooke/buildstructor/issues/52)
Add docs to generated builder.
In addition, a type alias is introduced for the initial builder type so that:
1. the docs look nice
2. the builder can be passed to a function (although this is of limited real world use).

[#4](https://github.com/BrynCooke/buildstructor/issues/4)
Use `#[inline(always)]` on generated code.

## 0.2.0 - 2022-05-10
[#45](https://github.com/BrynCooke/buildstructor/issues/45)
Major refactor to expand the scope of buildstructor.

To provide more control over generated builders and allow builders for methods with receivers the top level annotation has changed:

`#[buildstructor::builder]` => `#[buildstructor::buildstructor]`
 
1. Annotate the impl with: `#[buildstructor::buildstructor]`
2. Annotate methods to create a builders for with: `#[builder]`

```rust
#[buildstructor::buildstructor]
impl Foo {
    #[builder]
    fn new(simple: String) -> Foo {
        Self { simple }
    }
}
```

You can configure your builder using the `#[builder]` annotation, which has the following attributes:
* `entry` => The entry point for your builder. If not specified then the pre-existing rules around `new/*_new` are used.
* `exit` => The terminal method for the generated builder. Defaults to `builder` for constructors and `call` for methods.

In addition, you can now specify builders on methods that take self:

```rust
#[derive(Default)]
pub struct Client;

#[buildstructor::buildstructor]
impl Client {
    #[builder(entry = "phone", exit = "call")]
    fn phone_call(self, _simple: String) {}
}

fn main() {
    Client::default().phone().simple("3").call();
}
```
Note, if method parameters begin with `_` then this is stripped for the builder method names.

## 0.1.12 - 2022-05-06
[#39](https://github.com/BrynCooke/buildstructor/issues/39)
Visibility of builder now matches the visibility of each constructor.

## 0.1.11 - 2022-05-06
[#39](https://github.com/BrynCooke/buildstructor/issues/39)
Visibility of builder now matches the visibility of each constructor.

[#28](https://github.com/BrynCooke/buildstructor/issues/28)
Generalize replacing of self in return type.

## 0.1.10 - 2022-05-04
[#30](https://github.com/BrynCooke/buildstructor/issues/30)
The original token stream is output if there are compile errors.
This allows IDEs to auto complete during periods of invalid code.

## 0.1.9 - 2022-04-26
[#24](https://github.com/BrynCooke/buildstructor/issues/24)
Simple types are now given Into treatment globally.

## 0.1.8 - 2022-04-25
[#5](https://github.com/BrynCooke/buildstructor/issues/5)
Simple types are now given Into treatment when inserting to collection via singular form.  

## 0.1.7 - 2022-04-23
[#18](https://github.com/BrynCooke/buildstructor/issues/18) Relaxed collection support.
Collections type matching is relaxed to the following:

| Type Name | Method used to insert |
|-----------|-----------------------|
| ...Buffer | push(_)               |
| ...Deque  | push(_)               |
| ...Heap   | push(_)               |
| ...Set    | insert(_)             |
| ...Stack  | push(_)               |
| ...Map    | insert(_, _)          |
| Vec       | push(_)               |

## 0.1.6 - 2022-04-23
[#14](https://github.com/BrynCooke/buildstructor/issues/14) Generics ordering bug.
Generics were not being consistently ordered, which caused issues if there were generics on the impl type and also in a where clause.

## 0.1.5 - 2022-04-11
### Added
[#9](https://github.com/BrynCooke/buildstructor/issues/9) Add `*_new` support.
Any method named `new` or has a suffix `_new` will create a builder.
Builders methods are named appropriately. e.g. `try_new` -> `try_build`.
### Fixed
[#11](https://github.com/BrynCooke/buildstructor/issues/11) Fix multiple builders in the same module.
Removes the use of wildcard imports to builder modules to fix name clashes. 

[#8](https://github.com/BrynCooke/buildstructor/issues/8) Fix constructors that return `Self`
`Self` on builders needed to be converted to the target type. 

## 0.1.4 - 2022-03-30
### Fixed
[#6](https://github.com/BrynCooke/buildstructor/issues/6) Fix generics on collections.
This mostly rolls back the changes in [#1](https://github.com/BrynCooke/buildstructor/issues/1). THe examples have been updated to show the correct way to use into with a collection.

## 0.1.3 - 2022-03-30
### Fixed
[#1](https://github.com/BrynCooke/buildstructor/issues/1) Fix generics on collections

## 0.1.2 - 2022-03-30
### Changed
Improve readme

Add rust doc to `[builder]`

## 0.1.1 - 2022-03-29

### Changed
Improve readme

## 0.1.0 - 2022-003-29

### Added
Initial release
