# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0-rc.2

### Chore

 - <csr-id-fd0f1abb37698fb9f5bba94370378fa3083c17c2/> formatting
 - <csr-id-54d0eeaf839e7215afb0d1f579440551c48e2a62/> linting + formatting
 - <csr-id-743a7622a259ade966331f125b3bace501f808da/> update kira to main (0.9)
 - <csr-id-359093ac9e1e6e2f0151cfb99613bb6eadacbabd/> format

### New Features

 - <csr-id-bd71d8ea1d6719e9adda6c9a2ff0c32650a556c8/> Add `AudioFileHandle::toggle` method for convenience
 - <csr-id-3e52e51077f53f6f47e6ccced4e2882cc3218b3b/> add controllable behavior when playback stopped (and can no longer be resumed)
 - <csr-id-c4357184aa9cf17a32da0d103690d5379bae3c2c/> add set_volume to AudioFileHandle

### Bug Fixes

 - <csr-id-66ebc2312d6420b6941070c5e9a4ebae838ff794/> use `async fn` in `AudioFileLoader`
 - <csr-id-6bb722bb3a62e483efdca6b6bac9f0288fa6827c/> wording and typos from review
 - <csr-id-1718ec4dd076368b63e118edb9e6ec06fb41e607/> update backend settings to support kira 0.9
 - <csr-id-ad090adabafcc33d7e2bb5eeaa044adcca4e3b04/> use linear attenuation in SpatialEmitter by default
 - <csr-id-ee89f202ab401f12053945723aa528875f123025/> use linear attenuation in SpatialEmitter by default

### Other

 - <csr-id-8f3087cf9a1094cc13c472c82ec7c77fda191cbc/> custom sound documentation in example

### Refactor

 - <csr-id-d7579f2ab70e609c111bd4ffd6d3bba91f47b064/> rename consts from NUM_ prefix to _COUNT suffix
 - <csr-id-5a0e9278eae2a1c7c76f81647d494267afafc51f/> rename consts from NUM_ prefix to _COUNT suffix
 - <csr-id-2f45da39069e15f1a790572b085c346027b966c6/> move audio file impls into submodule

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 25 commits contributed to the release over the course of 50 calendar days.
 - 50 days passed between releases.
 - 16 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 6 unique issues were worked on: [#14](https://github.com/SolarLiner/bevy-kira-components/issues/14), [#16](https://github.com/SolarLiner/bevy-kira-components/issues/16), [#17](https://github.com/SolarLiner/bevy-kira-components/issues/17), [#21](https://github.com/SolarLiner/bevy-kira-components/issues/21), [#7](https://github.com/SolarLiner/bevy-kira-components/issues/7), [#8](https://github.com/SolarLiner/bevy-kira-components/issues/8)

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **[#14](https://github.com/SolarLiner/bevy-kira-components/issues/14)**
    - Add `AudioFileHandle::toggle` convenience method ([`1932421`](https://github.com/SolarLiner/bevy-kira-components/commit/19324217be7cfd74696b12ef8860eaa7f5d2c0c2))
 * **[#16](https://github.com/SolarLiner/bevy-kira-components/issues/16)**
    - Remove `Result<(), E>` from the API ([`93e56f1`](https://github.com/SolarLiner/bevy-kira-components/commit/93e56f18f1ca7b8db54416882d2a674ecad0a8f0))
 * **[#17](https://github.com/SolarLiner/bevy-kira-components/issues/17)**
    - Recreate `audio` and `decodable/custom_sound` examples from `bevy_audio` ([`81b696a`](https://github.com/SolarLiner/bevy-kira-components/commit/81b696a334c33a56f78bcb63c8de7d62fd67e931))
 * **[#21](https://github.com/SolarLiner/bevy-kira-components/issues/21)**
    - Update to Bevy 0.14.0-rc.2 ([`32068fe`](https://github.com/SolarLiner/bevy-kira-components/commit/32068fe39348b4b2f3e011c3a69e3e6573a5f480))
 * **[#7](https://github.com/SolarLiner/bevy-kira-components/issues/7)**
    - Rename consts from NUM_ prefix to _COUNT suffix ([`d7579f2`](https://github.com/SolarLiner/bevy-kira-components/commit/d7579f2ab70e609c111bd4ffd6d3bba91f47b064))
 * **[#8](https://github.com/SolarLiner/bevy-kira-components/issues/8)**
    - Use linear attenuation in SpatialEmitter by default ([`ad090ad`](https://github.com/SolarLiner/bevy-kira-components/commit/ad090adabafcc33d7e2bb5eeaa044adcca4e3b04))
 * **Uncategorized**
    - Use `async fn` in `AudioFileLoader` ([`66ebc23`](https://github.com/SolarLiner/bevy-kira-components/commit/66ebc2312d6420b6941070c5e9a4ebae838ff794))
    - Formatting ([`fd0f1ab`](https://github.com/SolarLiner/bevy-kira-components/commit/fd0f1abb37698fb9f5bba94370378fa3083c17c2))
    - Custom sound documentation in example ([`8f3087c`](https://github.com/SolarLiner/bevy-kira-components/commit/8f3087cf9a1094cc13c472c82ec7c77fda191cbc))
    - Wording and typos from review ([`6bb722b`](https://github.com/SolarLiner/bevy-kira-components/commit/6bb722bb3a62e483efdca6b6bac9f0288fa6827c))
    - Linting + formatting ([`54d0eea`](https://github.com/SolarLiner/bevy-kira-components/commit/54d0eeaf839e7215afb0d1f579440551c48e2a62))
    - Update backend settings to support kira 0.9 ([`1718ec4`](https://github.com/SolarLiner/bevy-kira-components/commit/1718ec4dd076368b63e118edb9e6ec06fb41e607))
    - Update kira to main (0.9) ([`743a762`](https://github.com/SolarLiner/bevy-kira-components/commit/743a7622a259ade966331f125b3bace501f808da))
    - Format ([`359093a`](https://github.com/SolarLiner/bevy-kira-components/commit/359093ac9e1e6e2f0151cfb99613bb6eadacbabd))
    - Add `AudioFileHandle::toggle` method for convenience ([`bd71d8e`](https://github.com/SolarLiner/bevy-kira-components/commit/bd71d8ea1d6719e9adda6c9a2ff0c32650a556c8))
    - Update src/spatial.rs ([`00be819`](https://github.com/SolarLiner/bevy-kira-components/commit/00be819acdf29dd7eae05063ab6c7c1bd3ce2403))
    - Use linear attenuation in SpatialEmitter by default ([`ee89f20`](https://github.com/SolarLiner/bevy-kira-components/commit/ee89f202ab401f12053945723aa528875f123025))
    - Rename consts from NUM_ prefix to _COUNT suffix ([`5a0e927`](https://github.com/SolarLiner/bevy-kira-components/commit/5a0e9278eae2a1c7c76f81647d494267afafc51f))
    - Merge pull request #9 from GitGhillie/audio-control-example ([`184780c`](https://github.com/SolarLiner/bevy-kira-components/commit/184780c7e986eb727f1ef3cac0c2e26f11bce535))
    - No_run on doc example ([`de68f8b`](https://github.com/SolarLiner/bevy-kira-components/commit/de68f8b173439eeb3837f314995c8d37ac4a207c))
    - :chore: formatting ([`f028768`](https://github.com/SolarLiner/bevy-kira-components/commit/f028768bb668735b3b183c5572241ceaf588f45f))
    - Add controllable behavior when playback stopped (and can no longer be resumed) ([`3e52e51`](https://github.com/SolarLiner/bevy-kira-components/commit/3e52e51077f53f6f47e6ccced4e2882cc3218b3b))
    - Move audio file impls into submodule ([`2f45da3`](https://github.com/SolarLiner/bevy-kira-components/commit/2f45da39069e15f1a790572b085c346027b966c6))
    - Add set_volume to AudioFileHandle ([`c435718`](https://github.com/SolarLiner/bevy-kira-components/commit/c4357184aa9cf17a32da0d103690d5379bae3c2c))
    - Merge pull request #3 from SolarLiner/release/0.1.1 ([`c67b5e8`](https://github.com/SolarLiner/bevy-kira-components/commit/c67b5e8866ffc47d3f4dcff841383d050d83a04f))
</details>

## v0.1.1 (2024-04-28)

## v0.1.0 (2024-04-28)

## v0.1.0-rc.0 (2024-04-28)

### Chore

 - <csr-id-b6e64ddea63a51254ed4f02d2b6127b8b1035bfe/> linting + format
 - <csr-id-64a30935c5875f213d76ab70e1357f0352602d5b/> liting + formatting
 - <csr-id-70fbdded7a3e66031425c69bccc5adf71d700db4/> liting + formatting
 - <csr-id-6f91bc8af5a099917ec45b4614aa3fbfd8260ac0/> clippy fixes
 - <csr-id-ad7dbf94352b4537fcf3a62fc0448edc0bd10770/> format
 - <csr-id-a2f3a979cda9c171782f877bc2681beb6bb12c57/> clippy fixes

### Documentation

 - <csr-id-a0a530c39929b8e3efcf8303911eb71c42e47589/> add missing doc comments

### New Features

 - <csr-id-3864d282a56dbe41ccc932bfd48dae479c2464e8/> remove tracks integration
   It's clunky and does not work properly, requires too much fiddling with builders and spawning, I don't like it :(
 - <csr-id-7ccaed59e698c0d7573d39e217f1dac4d1b1f52a/> make diagnostics optional by introducing a feature flag
 - <csr-id-40a9cf8e41eb79d58a162a6b017df4c6212313fa/> make diagnostics optional by introducing a feature flag
 - <csr-id-448694d5d446c852f4eaa688cc36cd78ba925b2b/> refactor plugins to make them generic over the sound asset
 - <csr-id-d54b649295617dbd625f22d978d6644d77b6b5da/> refactor plugins to make them generic over the sound asset
 - <csr-id-9cf11ce23b54ece421e8b9c699861f193da69085/> implement all commands on audio entites + all getters in AudioWorld
 - <csr-id-f2cddf07e947ed3551e6d304bf498b9a46520379/> add diagnostics for num spatial scenes
 - <csr-id-710c788e2d84673dc8886f931987b1c0094a9df6/> spatial audio support

### Bug Fixes

 - <csr-id-aba6fc83005e6ae5b384dd89740ab57bc0cb3b55/> missing third slash for doc comment
 - <csr-id-a8e2b69e33dee886e9262302abaa127c2dcc2b2b/> missing ! in missing_docs attribute
 - <csr-id-1b361488f66b5b5c1ecc52133298c8018547dd13/> spatial system ordering issue + make example more intuitive

### Other

 - <csr-id-415970c56598b36e0c5701578b4308fe3ccac4d9/> custom sound example
 - <csr-id-bdf1bce6bd58280199e795d818b701a07b537e08/> add missing docstrings and an example at crate-level
 - <csr-id-74924f76ff38c9cef5960a5800d7db0a59ceeaf0/> add succint documentation on crate and types
 - <csr-id-e65b6e853bd31aac94a089bd8276d8b2fdc2b509/> initial crate-level docs

### Refactor

 - <csr-id-e4777a76c8442eb6fb3b935585ac8bd34f1543f0/> use EntityHashMap instead of BTreeMap
 - <csr-id-953647b94863fd01436a13a46ed18f39a54e02ba/> simplify plugins by storing maps from entities to handles of different types

### Test

 - <csr-id-8b818f1de215c53e8ba5f79147608e529a2ee163/> fix doctest

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 26 commits contributed to the release over the course of 49 calendar days.
 - 25 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Missing third slash for doc comment ([`aba6fc8`](https://github.com/SolarLiner/bevy-kira-components/commit/aba6fc83005e6ae5b384dd89740ab57bc0cb3b55))
    - Remove tracks integration ([`3864d28`](https://github.com/SolarLiner/bevy-kira-components/commit/3864d282a56dbe41ccc932bfd48dae479c2464e8))
    - Add missing doc comments ([`a0a530c`](https://github.com/SolarLiner/bevy-kira-components/commit/a0a530c39929b8e3efcf8303911eb71c42e47589))
    - Missing ! in missing_docs attribute ([`a8e2b69`](https://github.com/SolarLiner/bevy-kira-components/commit/a8e2b69e33dee886e9262302abaa127c2dcc2b2b))
    - Make diagnostics optional by introducing a feature flag ([`7ccaed5`](https://github.com/SolarLiner/bevy-kira-components/commit/7ccaed59e698c0d7573d39e217f1dac4d1b1f52a))
    - Make diagnostics optional by introducing a feature flag ([`40a9cf8`](https://github.com/SolarLiner/bevy-kira-components/commit/40a9cf8e41eb79d58a162a6b017df4c6212313fa))
    - Custom sound example ([`415970c`](https://github.com/SolarLiner/bevy-kira-components/commit/415970c56598b36e0c5701578b4308fe3ccac4d9))
    - Fix doctest ([`8b818f1`](https://github.com/SolarLiner/bevy-kira-components/commit/8b818f1de215c53e8ba5f79147608e529a2ee163))
    - Linting + format ([`b6e64dd`](https://github.com/SolarLiner/bevy-kira-components/commit/b6e64ddea63a51254ed4f02d2b6127b8b1035bfe))
    - Add missing docstrings and an example at crate-level ([`bdf1bce`](https://github.com/SolarLiner/bevy-kira-components/commit/bdf1bce6bd58280199e795d818b701a07b537e08))
    - Refactor plugins to make them generic over the sound asset ([`448694d`](https://github.com/SolarLiner/bevy-kira-components/commit/448694d5d446c852f4eaa688cc36cd78ba925b2b))
    - Refactor plugins to make them generic over the sound asset ([`d54b649`](https://github.com/SolarLiner/bevy-kira-components/commit/d54b649295617dbd625f22d978d6644d77b6b5da))
    - Liting + formatting ([`64a3093`](https://github.com/SolarLiner/bevy-kira-components/commit/64a30935c5875f213d76ab70e1357f0352602d5b))
    - Add succint documentation on crate and types ([`74924f7`](https://github.com/SolarLiner/bevy-kira-components/commit/74924f76ff38c9cef5960a5800d7db0a59ceeaf0))
    - Implement all commands on audio entites + all getters in AudioWorld ([`9cf11ce`](https://github.com/SolarLiner/bevy-kira-components/commit/9cf11ce23b54ece421e8b9c699861f193da69085))
    - Initial crate-level docs ([`e65b6e8`](https://github.com/SolarLiner/bevy-kira-components/commit/e65b6e853bd31aac94a089bd8276d8b2fdc2b509))
    - Liting + formatting ([`70fbdde`](https://github.com/SolarLiner/bevy-kira-components/commit/70fbdded7a3e66031425c69bccc5adf71d700db4))
    - Use EntityHashMap instead of BTreeMap ([`e4777a7`](https://github.com/SolarLiner/bevy-kira-components/commit/e4777a76c8442eb6fb3b935585ac8bd34f1543f0))
    - Spatial system ordering issue + make example more intuitive ([`1b36148`](https://github.com/SolarLiner/bevy-kira-components/commit/1b361488f66b5b5c1ecc52133298c8018547dd13))
    - Clippy fixes ([`6f91bc8`](https://github.com/SolarLiner/bevy-kira-components/commit/6f91bc8af5a099917ec45b4614aa3fbfd8260ac0))
    - Format ([`ad7dbf9`](https://github.com/SolarLiner/bevy-kira-components/commit/ad7dbf94352b4537fcf3a62fc0448edc0bd10770))
    - Add diagnostics for num spatial scenes ([`f2cddf0`](https://github.com/SolarLiner/bevy-kira-components/commit/f2cddf07e947ed3551e6d304bf498b9a46520379))
    - Simplify plugins by storing maps from entities to handles of different types ([`953647b`](https://github.com/SolarLiner/bevy-kira-components/commit/953647b94863fd01436a13a46ed18f39a54e02ba))
    - Clippy fixes ([`a2f3a97`](https://github.com/SolarLiner/bevy-kira-components/commit/a2f3a979cda9c171782f877bc2681beb6bb12c57))
    - Spatial audio support ([`710c788`](https://github.com/SolarLiner/bevy-kira-components/commit/710c788e2d84673dc8886f931987b1c0094a9df6))
    - Initial commit with components for audio source and tracks ([`6a8d3f1`](https://github.com/SolarLiner/bevy-kira-components/commit/6a8d3f13425b4f334659727788387d9fbcc1955b))
</details>

