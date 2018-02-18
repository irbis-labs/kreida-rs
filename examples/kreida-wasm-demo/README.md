# kreida-rs WAsm Demo

## Build

### Requirements

#### [Emscripten](https://kripken.github.io/emscripten-site/docs/index.html)

* [Building graphical applications to JS in stable Rust](https://gregkatz.github.io/2017-05-20-rust-emscripten.html)
* [The Path to Rust on the Web](http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/)

> First, add the wasm32 target via rustup:

```bash
rustup target add wasm32-unknown-emscripten
```

> Now, install the emscripten SDK.

```bash
curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar -xzf emsdk-portable.tar.gz
source emsdk-portable/emsdk_env.sh
emsdk update
```

And then you can compile latest sdk from sources (this will take nearly an hour, a lot of RAM, and about 20 Gb of HDD)

```bash
emsdk install sdk-incoming-64bit
emsdk activate sdk-incoming-64bit
```

or you can install a precompiled binary (run `emsdk list` to check available versions)

```bash
emsdk install sdk-1.37.33-64bit
emsdk activate sdk-1.37.33-64bit
```

> You may need to run `source emsdk-portable/emsdk_env.sh` one extra time

#### [Cargo-Web](https://crates.io/crates/cargo-web)

``` sh
cargo install cargo-web
```

## Build and Run

``` sh
cargo web start
```

or

``` sh
cargo web start --release
```

and then open `http://127.0.0.1:8000` or `http://[::1]:8000` in your browser.


## Screenshots

![](assets/screenshot/spiro.png)
![](assets/screenshot/vortex.png)


## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
