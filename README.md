<div class="title-block" style="text-align: center;" align="center">

# `scopefn` <!-- omit in toc -->

</div>

`scopefn` is a Rust crate that provides scope functions which inspired by Kotlin's scope functions. It allows you to write more concise and expressive code by providing a set of functions that operate on a value within a specific scope.

## Example Usage


### Debugging

```rust
fn with_debug(data: Vec<i32>) -> Vec<i32> {
    data
        .apply(|d| debug!(d))
        .do_what_you_want()
        .apply(|d| debug!(d))
}
```

### Sorting

```rust
fn sort_them(data: Vec<i32>) -> Vec<i32> {
    data.apply_mut(|d| d.sort())
}
```
