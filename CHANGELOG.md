# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
