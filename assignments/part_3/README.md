# Embassy assignment

This code requires the use of the nightly compiler.
This should be downloaded automatically due to the `rust-toolchain.toml` file.

You can install and use the toolchain easily by running:
```
rustup toolchain install nightly
cargo +nightly build
```

If you don't want to provide the `+nightly` every time, we can make it the default:

```
rustup default nightly
```

You can find more embassy examples for the nrf52840 here: https://github.com/embassy-rs/embassy/tree/master/examples/nrf52840/src/bin

You can try getting the lis3dh to work in embassy with this driver: https://crates.io/crates/lis3dh-async
