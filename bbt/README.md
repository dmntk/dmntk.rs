**DMNTK** | Decision Model and Notation Toolkit

# Black-box tests

[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
[![Contributor Covenant][coc-badge]](https://github.com/dmntk/dmntk.rs/blob/main/CODE_OF_CONDUCT.md)

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE
[coc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

## Overview

Black-box tests for **DMNTK** project.

The **DMNTK** project is tested automatically with all types of tests provided by Rust,
and additionally with test runner that executes all available compatibility tests.

Nevertheless, this set of black-box tests is maintained in the following purposes:
- testing the edge cases,
- diagnostics of reported issues,
- visualization of functionalities provided by **DMNTK**,
- preparing documentation ([dmntk.io](https://dmntk.io)).

## Running black-box tests

Run all tests:

```
$ ./bbt.sh
```

Run tests in specified directory, e.g. in `cli/noargs`: 

```
$ ./bbt.sh cli/noargs
```

## Test directories structure

Tests are organized in directories starting from the root directory named `./tests`.
Each directory may contain either subdirectories or test files.

## Test files structure

There are always four test files prepared for each test:
1. Text file containing tested expression, decision table or DMN model, may have any name.
2. Text file containing test execution context, may have any name.
3. Text file containing expected result, should always be named `expected`.
4. Shell script containing a command that runs a test, should always be named `run.sh`.

## Example test

Directory `tests/feel/addition/0001` contains a test that checks addition operation of two numbers:
1. Tested expression is `1 + 1` and is saved in `0001.feel` file.
2. Tested expression context is empty `{}` and is saved in `0001.ctx` file.
3. Expected value is `2` and is saved in `expected` file.
4. Script that executes a test contains a command `dmntk efe 0001.ctx 0001.feel` and is saved in `run.sh` file.

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE))

at your option.

## Contribution

We **appreciate any contributions** from the community to help improve our project.
If you would like to get involved, please don't hesitate to reach out to us.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.