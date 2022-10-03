Prometheus meets Serde
======================

This crate is a Rust library for [Prometheus labels]. It is built
upon [Serde], a high performance generic serialization framework.

[Serde]: https://github.com/serde-rs/serde
[Prometheus labels]: https://github.com/prometheus/docs/blob/main/content/docs/instrumenting/exposition_formats.md#text-format-details

## Installation

This crate works with Cargo and can be found on [crates.io] with a `Cargo.toml` like:

```toml
[dependencies]
serde_prometheus_labels = "0.1"
```

The documentation is available on [docs.rs].

[crates.io]: https://crates.io/crates/serde_prometheus_labels
[docs.rs]: https://docs.rs/serde_prometheus_labels/0.1.0/

## Getting help

You can find me on IRC either in `##rust` or `#rust-fr` on
[Libera.Chat](https://libera.chat). If IRC is not your thing, I am happy to
respond to [GitHub issues](https://github.com/nox/serde_prometheus_labels/issues/new)
as well.

## License

`serde_prometheus_labels` is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `serde_prometheus_labels` by you, as defined in the Apache-2.0
license shall be dual licensed as above, without any additional terms or conditions.
