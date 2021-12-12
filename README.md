# rexplode

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

## License

Copyright (c) 2021 woxtu

Licensed under the MIT license.
