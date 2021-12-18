# rexplode

[![Crates.io](https://img.shields.io/crates/v/rexplode?style=flat-square)](https://crates.io/crates/rexplode)

Generate strings from the given regular expression.

```console
$ rexplode 'Number [0-6]'
Number 0
Number 1
Number 2
Number 3
Number 4
Number 5
Number 6
```

### Supported syntax

|Syntax|Example|
|:--|:--|
|Character classes|`[abc]` `[a-c]`|
|Composites|`ab` `a\|b`|
|Repetitions|`a?` `a{2}` `a{0,2}`|
|Groups|`(a)` `(?:a)`|

## Installation

Download the binary from the release page:

https://github.com/woxtu/rexplode/releases

Or install with Cargo:

```console
$ cargo install rexplode
```

## License

Copyright (c) 2021 woxtu

Licensed under the MIT license.
