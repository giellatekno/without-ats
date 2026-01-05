# without_ats

A one-function library with a function to remove every occurence of `"@...@"` from
a string.

This is useful for us to remove the diacritics from analyser output.


## function: without_ats_iter

Return an iterator of all the string slices of `s` that are not "enclosed" in "@..@".

Example:

```rust
use without_ats::without_ats_iter;

let clean = without_ats_iter("@AAA@Clean@BBB@String@CCC@");
assert_eq!(String::from_iter(clean), String::from("CleanString"));
```

### function: without_ats

A convenience function that returns a new `String` instead of an iterator over the
subslices.

```rust
use without_ats::without_ats;

let clean = without_ats("@AAA@Clean@BBB@String@CCC@");
assert_eq!(clean, String::from("CleanString"));
```
