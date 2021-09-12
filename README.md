# existential

[![docs.rs]](https://docs.rs/existential)
[![crates.io]](https://crates.io/crates/existential)
[![github]](https://github.com/steffahn/existential)
[![MIT / Apache 2.0 licensed]](#License)
[![unsafe forbidden]](https://github.com/rust-secure-code/safety-dance/)

[github]: https://img.shields.io/badge/github-steffahn/existential-yellowgreen.svg
[crates.io]: https://img.shields.io/crates/v/existential.svg?maxAge=86400
[MIT / Apache 2.0 licensed]: https://img.shields.io/crates/l/existential.svg?maxAge=2592000
[docs.rs]: https://docs.rs/existential/badge.svg
[unsafe forbidden]: https://img.shields.io/badge/unsafe-forbidden-success.svg


[Existential types](https://wiki.haskell.org/Existential_type) in Rust, offering existential
quantification over lifetimes, but as a library. This works because Rust has
[parametricity](https://en.wikipedia.org/wiki/Parametricity) for generic lifetime arguments.

_Work in progress; this crate is still undocumented._

## License
Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.
