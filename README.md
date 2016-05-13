# jalali

Safe bindings for [libjalali](https://github.com/ashkang/jcal) library.

## Installation

You must have installed [libjalali](https://github.com/ashkang/jcal) before hand.

Then add this line to your *Cargo.toml* dependencies:

```toml
jalali = "0.1"
```

And then execute:

    $ cargo build

This crate uses the static library by default. If you want to
compile using the shared library, execute the following command:

    $ cargo build --features "use-shared-jalali"
