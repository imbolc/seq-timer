[![License](https://img.shields.io/crates/l/seq-timer.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/seq-timer.svg)](https://crates.io/crates/seq-timer)
[![Docs.rs](https://docs.rs/seq-timer/badge.svg)](https://docs.rs/seq-timer)

# seq-timer

A simple timer for sequential events

```rust
use std::{ time::Duration, thread::sleep };

let mut timer = seq_timer::Timer::new();

// starts the first event
timer.start("the first event");
sleep(Duration::from_millis(1));

// finishes the first event and starts the second one
// you can also `.finish()` the current event manually
timer.start("the second event");
sleep(Duration::from_millis(10));

// finishes the last event and prints sorted measurments to stdout:
timer.print();
```
The output would be similar to:
```
the second event | 10078204 ns |  88%
 the first event |  1265423 ns |  11%
```
The timer also implements [Display](core::fmt::Display), but you must
finish the last event manually in this case: `debug!("{}", timer.finish())`

## Contributing

We appreciate all kinds of contributions, thank you!

### Note on README

The `README.md` file isn't meant to be changed directly. It instead generated from the crate's docs
by the [cargo-readme] command:

* Install the command if you don't have it: `cargo install cargo-readme`
* Change the crate-level docs in `src/lib.rs`, or wrapping text in `README.tpl`
* Apply the changes: `cargo readme > README.md`

If you have [rusty-hook] installed the changes will apply automatically on commit.

## License

This project is licensed under the [MIT license](LICENSE).

[cargo-readme]: https://github.com/livioribeiro/cargo-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
