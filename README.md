### Rust binding to libmath.so

This example project shows how to create bindings in Rust to a c library. In particular it is the [libmath library from this repo](https://github.com/levpaul/c-library-example).

#### How

This was very easy, basically from the C source repo I ran:

```bash
bindgen math/math.h  -o /path/to/this/repo/src/math.rs
```

Then I just needed to add `LD_LIBRARY_PATH` _and_ `LIBRARY_PATH` to the `build.rs` script (and set that up in `Cargo.toml`). From here I could import the `math` mod in `main.rs` and use it just like I could in C.

#### Requirements

- libmath.so installed in system libraries _or_ installed to `/opt/lib` (build.rs sets this up)
- bindgen installed if you want to regenerate `src/math.rs` (just *cargo install bindgen*)

### Notes

This was just a toy project to see what is involved in making Rust bindings for a C library from scratch. As such there is probably plenty left to be desired here. For starters I passed no extra flags to bindgen for my bindings - so things may be a bit verbose.

### Related Readings:

- https://medium.com/dwelo-r-d/using-c-libraries-in-rust-13961948c72a
- https://rust-lang.github.io/rust-bindgen/
- https://doc.rust-lang.org/nomicon/ffi.html
