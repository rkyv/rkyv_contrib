# rkyv Community Contributions &emsp; [![Discord]][discord invite] [![License]][license path]

[Discord]: https://img.shields.io/discord/822925794249539645
[discord invite]: https://discord.gg/65F6MdnbQh
[License]: https://img.shields.io/badge/license-MIT-blue.svg
[license path]: https://github.com/djkoloski/rkyv_contrib/blob/master/LICENSE
[rkyv]: https://github.com/djkoloski/rkyv

A collection of crates and utilities for [rkyv][rkyv] written by the community.

## Crates

- `rkyv_wrappers`: Common and specialized wrapper types

## Contributing

Contributions of any form are always welcome! When contributing code, it's recommended that you:

- Join the [Discord][discord invite] to get community feedback on your code and idea.
- Consider how your code composes with others.
- Write tests to prove your code's correctness, especially doc tests.
- Run the tests and make sure your pull requests will keep others' code working.
- Make sure your code is suitably commented and formatted. All public items should have doc
  comments.

This repository has a more permissive contribution model than [rkyv][rkyv]. With that in mind, some
contributions may not be a good fit for it. If you're not sure whether your contribution fits these,
definitely ask in the [Discord][discord invite]. The ideal contribution fits the following criteria:

- **Small to medium size in both scope and code.** Large contributions are usually best split out
  into a separate repository and crate. Some really good contributions will be big, so don't
  hesitate to ask in the [Discord][discord invite]!
- **Applicable for at least a small number of users.** Extremely niche contributions can decrease
  the signal to noise ratio and discourage the use of crate collections.
- **Only relies on common dependencies.** Dependencies like `serde`, `rand`, and `log` are fine
  since they are widely used. Niche dependencies are discouraged.
- **Provides real benefit to functionality, ergonomics, or performance.** Contributions without a
  compelling reason for use are not a good fit.

When contributing, always follow the
[Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). For escalation or
moderation issues please contact David (@djkoloski).

Unless explicitly stated otherwise, all contributions submitted for inclusion shall be dual-licensed
under the MIT License and Apache License, Version 2.0, without any additional terms or conditions.
