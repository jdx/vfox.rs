# Changelog

## [0.3.3](https://github.com/jdx/vfox.rs/compare/v0.3.2..v0.3.3) - 2024-11-12

### 🐛 Bug Fixes

- installing when tmp is on a different drive by [@jdx](https://github.com/jdx) in [bfc9678](https://github.com/jdx/vfox.rs/commit/bfc967830ea4c704f3a16d31e49f037a208c39ec)
- ensure tmp dir parent is created by [@jdx](https://github.com/jdx) in [d82537d](https://github.com/jdx/vfox.rs/commit/d82537dc8e8df878df1e4818c693c293d7ada3d3)

## [0.3.2](https://github.com/jdx/vfox.rs/compare/v0.3.1..v0.3.2) - 2024-11-11

### 🐛 Bug Fixes

- increase html parser limit by [@jdx](https://github.com/jdx) in [1bae49d](https://github.com/jdx/vfox.rs/commit/1bae49dc01fa82c7c7344972c80647390eb4ced7)
- set OS_TYPE,ARCH_TYPE by [@jdx](https://github.com/jdx) in [9851ae2](https://github.com/jdx/vfox.rs/commit/9851ae2c607adf39b798da525949285bdcb31774)
- added more runtime config by [@jdx](https://github.com/jdx) in [#59](https://github.com/jdx/vfox.rs/pull/59)

### 🔍 Other Changes

- updated deps by [@jdx](https://github.com/jdx) in [7b6a78b](https://github.com/jdx/vfox.rs/commit/7b6a78b432beee7575163864875377e56c6669c1)

## [0.3.1](https://github.com/jdx/vfox.rs/compare/v0.3.0..v0.3.1) - 2024-11-06

### 🔍 Other Changes

- added cargo-binstall to mise.toml by [@jdx](https://github.com/jdx) in [#54](https://github.com/jdx/vfox.rs/pull/54)

## [0.3.0](https://github.com/jdx/vfox.rs/compare/v0.2.2..v0.3.0) - 2024-11-05

### 🚀 Features

- add http module by [@jdx](https://github.com/jdx) in [#52](https://github.com/jdx/vfox.rs/pull/52)

## [0.2.2](https://github.com/jdx/vfox.rs/compare/v0.2.1..v0.2.2) - 2024-11-04

### 🔍 Other Changes

- Remove `dbg!` from `mise_path` plugin to reduce noise by [@joshbode](https://github.com/joshbode) in [#50](https://github.com/jdx/vfox.rs/pull/50)

### New Contributors

- @joshbode made their first contribution in [#50](https://github.com/jdx/vfox.rs/pull/50)

## [0.2.1](https://github.com/jdx/vfox.rs/compare/v0.2.0..v0.2.1) - 2024-10-26

### 🐛 Bug Fixes

- added "send" feature to mlua by [@jdx](https://github.com/jdx) in [49cef81](https://github.com/jdx/vfox.rs/commit/49cef815ac946ee93af016d42751ce25e1fc65ce)

## [0.2.0](https://github.com/jdx/vfox.rs/compare/v0.1.6..v0.2.0) - 2024-10-26

### 🚀 Features

- added mise_env/mise_path hooks by [@jdx](https://github.com/jdx) in [6621c24](https://github.com/jdx/vfox.rs/commit/6621c2474d4094586c8fd80962fcc5639dcbe646)

### 🔍 Other Changes

- updated mlua by [@jdx](https://github.com/jdx) in [8944a73](https://github.com/jdx/vfox.rs/commit/8944a7356d1ba404bcb697b564a939c6067b62cc)

## [0.1.6](https://github.com/jdx/vfox.rs/compare/v0.1.4..v0.1.6) - 2024-10-14

### 🐛 Bug Fixes

- hide logs by default by [@jdx](https://github.com/jdx) in [19d1ddd](https://github.com/jdx/vfox.rs/commit/19d1dddfc7302978ead7dfd5aadcffaa6d60fb3e)

## [0.1.4] - 2024-10-14

### 🐛 Bug Fixes

- clippy bug by [@jdx](https://github.com/jdx) in [3bfe31e](https://github.com/jdx/vfox.rs/commit/3bfe31e0f8e1690d3ad6bf6f62bed9c773927ae9)
- make installs without archives work by [@jdx](https://github.com/jdx) in [15a7bad](https://github.com/jdx/vfox.rs/commit/15a7badcdf7a0b95d31d4e7366ef2a39abba53f9)

### 📚 Documentation

- use dashes in CHANGELOG by [@jdx](https://github.com/jdx) in [dacfe24](https://github.com/jdx/vfox.rs/commit/dacfe2465745aee8b0369e652a41c92f2e062ff5)

### 🔍 Other Changes

- migrate away from deprecated git-cliff syntax by [@jdx](https://github.com/jdx) in [49f9f6f](https://github.com/jdx/vfox.rs/commit/49f9f6f1b27f85f5f5aef100430d149c3782dbb1)
- added some gitignores by [@jdx](https://github.com/jdx) in [7ec0306](https://github.com/jdx/vfox.rs/commit/7ec03064bcc169d34a4cad7dd5b63e257a6c3672)
- added extra logging info by [@jdx](https://github.com/jdx) in [0643f92](https://github.com/jdx/vfox.rs/commit/0643f9205cc851eb3f6d258c79cabdf2153acdf5)

### 📦️ Dependency Updates

- update rust crate clap to v4.5.18 by [@renovate[bot]](https://github.com/renovate[bot]) in [#35](https://github.com/jdx/vfox.rs/pull/35)
- update rust crate serde_json to v1.0.128 by [@renovate[bot]](https://github.com/renovate[bot]) in [#36](https://github.com/jdx/vfox.rs/pull/36)
- update rust crate once_cell to v1.20.1 by [@renovate[bot]](https://github.com/renovate[bot]) in [#38](https://github.com/jdx/vfox.rs/pull/38)
- update rust crate clap to v4.5.20 by [@renovate[bot]](https://github.com/renovate[bot]) in [#39](https://github.com/jdx/vfox.rs/pull/39)
- update rust crate homedir to v0.3.4 by [@renovate[bot]](https://github.com/renovate[bot]) in [#40](https://github.com/jdx/vfox.rs/pull/40)

## [0.1.3](https://github.com/jdx/vfox.rs/compare/v0.1.2..v0.1.3) - 2024-08-28

### 🐛 Bug Fixes

- amd64 arch by [@jdx](https://github.com/jdx) in [05a8885](https://github.com/jdx/vfox.rs/commit/05a88857583171acf11746a270399b8a7c263551)

### 🔍 Other Changes

- enable release-plz workflow button by [@jdx](https://github.com/jdx) in [302c970](https://github.com/jdx/vfox.rs/commit/302c970871706207107b6c84ce7849a1cc854b4b)

### 📦️ Dependency Updates

- update rust crate xx to v1.1.8 by [@renovate[bot]](https://github.com/renovate[bot]) in [#32](https://github.com/jdx/vfox.rs/pull/32)
- update rust crate serde_json to v1.0.127 by [@renovate[bot]](https://github.com/renovate[bot]) in [#25](https://github.com/jdx/vfox.rs/pull/25)
- update rust crate reqwest to v0.12.7 by [@renovate[bot]](https://github.com/renovate[bot]) in [#34](https://github.com/jdx/vfox.rs/pull/34)

## [0.1.2](https://github.com/jdx/vfox.rs/compare/v0.1.1..v0.1.2) - 2024-08-18

### 🐛 Bug Fixes

- working with more plugins by [@jdx](https://github.com/jdx) in [8c37c7c](https://github.com/jdx/vfox.rs/commit/8c37c7c2f8e40c448cc25618b566c533c8d66f2d)

## [0.1.1](https://github.com/jdx/vfox.rs/compare/v0.1.0..v0.1.1) - 2024-08-18

### 🐛 Bug Fixes

- various hook fixes by [@jdx](https://github.com/jdx) in [31e485a](https://github.com/jdx/vfox.rs/commit/31e485a128d1899cc3749d64f94aaad8a29b8b3c)

### 🧪 Testing

- ignore flaky test by [@jdx](https://github.com/jdx) in [b15a063](https://github.com/jdx/vfox.rs/commit/b15a0632fd6f9fe2e8f95b5eb3bff1feffe53ace)

### 🔍 Other Changes

- fix release-plz by [@jdx](https://github.com/jdx) in [190324b](https://github.com/jdx/vfox.rs/commit/190324b797ea7e4113950b3916469b9556f39805)

### 📦️ Dependency Updates

- update rust crate scraper to 0.20 by [@renovate[bot]](https://github.com/renovate[bot]) in [#28](https://github.com/jdx/vfox.rs/pull/28)

## [0.1.0] - 2024-08-17

### 🚀 Features

- hash by [@jdx](https://github.com/jdx) in [a30f8d4](https://github.com/jdx/vfox.rs/commit/a30f8d4d5856920b704f1e3b47f418ebab3df083)
- install function by [@jdx](https://github.com/jdx) in [37a1614](https://github.com/jdx/vfox.rs/commit/37a1614612fdcc051a2a4e1fb5a1ba616e72b76a)
- list available sdks by [@jdx](https://github.com/jdx) in [6a26ecf](https://github.com/jdx/vfox.rs/commit/6a26ecf1bfe4e9a7976e0b9af227d8047a6cade9)
- added env-keys hook/cli by [@jdx](https://github.com/jdx) in [78e18ef](https://github.com/jdx/vfox.rs/commit/78e18ef72a6fd5be8e58a7bb78d1eeae49879750)
- added metadata exporting by [@jdx](https://github.com/jdx) in [4fb7901](https://github.com/jdx/vfox.rs/commit/4fb7901869f294ba5abed95a66adc3e844dbf4d2)
- parse_legacy_file hook by [@jdx](https://github.com/jdx) in [dca4f3e](https://github.com/jdx/vfox.rs/commit/dca4f3e38d5c96050a5aaf0aeb82cef98a226ebf)
- stubbed out pre-use hook by [@jdx](https://github.com/jdx) in [1bd8988](https://github.com/jdx/vfox.rs/commit/1bd89885c7b06296666a6b985371775590826a4b)

### 🐛 Bug Fixes

- windows compat by [@jdx](https://github.com/jdx) in [1e1a99a](https://github.com/jdx/vfox.rs/commit/1e1a99ae6b610203252b9ce1ef03af3d67de5318)

### 🚜 Refactor

- refactor mod_json by [@jdx](https://github.com/jdx) in [9c8c648](https://github.com/jdx/vfox.rs/commit/9c8c648480accdaad99936f7fd377b7f9114bc88)

### 🧪 Testing

- tests for lua mods by [@jdx](https://github.com/jdx) in [40a9397](https://github.com/jdx/vfox.rs/commit/40a939704fb2e92bf8fabfbcc093004513ff27e7)
- fixed tests by [@jdx](https://github.com/jdx) in [99b5b6e](https://github.com/jdx/vfox.rs/commit/99b5b6e2bf03606d803f32b88862ca4517e5400a)
- fixing tests by [@jdx](https://github.com/jdx) in [3b5b13a](https://github.com/jdx/vfox.rs/commit/3b5b13a46698cca3e2b15a144a0a309309e46d3f)
- fixing tests by [@jdx](https://github.com/jdx) in [42ed58e](https://github.com/jdx/vfox.rs/commit/42ed58eda826e3910b4590488357873894764418)
- fixing tests by [@jdx](https://github.com/jdx) in [49bda33](https://github.com/jdx/vfox.rs/commit/49bda337c97ef8648934f76b4f26fe8a2d0f1db8)
- fixing tests by [@jdx](https://github.com/jdx) in [8814cfd](https://github.com/jdx/vfox.rs/commit/8814cfdfc276fb3abdf3fd8d8f2d7586888e9748)

### 🔍 Other Changes

- init by [@jdx](https://github.com/jdx) in [2bce6d5](https://github.com/jdx/vfox.rs/commit/2bce6d5ad1a8c690786a5c88b30084afa29962c2)
- wip by [@jdx](https://github.com/jdx) in [38b187b](https://github.com/jdx/vfox.rs/commit/38b187b6a94bcfb1dfa349cde18e7ce57e235e49)
- Squashed 'plugins/vfox-nodejs/' content from commit aa6dc5f by [@jdx](https://github.com/jdx) in [b2afd07](https://github.com/jdx/vfox.rs/commit/b2afd0700aa0053149653a6289ab3bfc0cdd5558)
- added git-cliff by [@jdx](https://github.com/jdx) in [56544e8](https://github.com/jdx/vfox.rs/commit/56544e880921c3f350f29a44fcaea0f02647b855)
- wip by [@jdx](https://github.com/jdx) in [db9d04a](https://github.com/jdx/vfox.rs/commit/db9d04ab06adf63c504989a0e07992f917f11ac5)
- wip by [@jdx](https://github.com/jdx) in [da50d76](https://github.com/jdx/vfox.rs/commit/da50d7686abcf4d723f5e24c0822c0629cc2e71e)
- wip by [@jdx](https://github.com/jdx) in [d3ad443](https://github.com/jdx/vfox.rs/commit/d3ad44337f42d5ffdeb60ddff0e809a42bf58330)
- wip by [@jdx](https://github.com/jdx) in [8009bd7](https://github.com/jdx/vfox.rs/commit/8009bd7a7465949ab258814691c5a22288e24ff5)
- wip by [@jdx](https://github.com/jdx) in [23d13ca](https://github.com/jdx/vfox.rs/commit/23d13ca871a638f01c6c8f0cd651181e4509969f)
- Update README.md by [@jdx](https://github.com/jdx) in [9202c26](https://github.com/jdx/vfox.rs/commit/9202c266c95ae083dca10c5792e0b257a614a641)
- wip by [@jdx](https://github.com/jdx) in [bbf2c43](https://github.com/jdx/vfox.rs/commit/bbf2c43f68d635f26ac5826ea5eb5abd03d264c9)
- dagger by [@jdx](https://github.com/jdx) in [40fe8bf](https://github.com/jdx/vfox.rs/commit/40fe8bf3518839210c8691a56021477f45610e15)
- created lua_mod directory by [@jdx](https://github.com/jdx) in [c4f26e3](https://github.com/jdx/vfox.rs/commit/c4f26e3a4c1c63c5c205a465f8402e8f21bdfe91)
- wip by [@jdx](https://github.com/jdx) in [c395efd](https://github.com/jdx/vfox.rs/commit/c395efde6bbfc131e3b4e9eb0e4cdf69b0269daf)
- added mod_strings by [@jdx](https://github.com/jdx) in [b5090e7](https://github.com/jdx/vfox.rs/commit/b5090e7a5dbb95870bc857f8938f241097f0a5c9)
- added mod_archiver by [@jdx](https://github.com/jdx) in [032cf2e](https://github.com/jdx/vfox.rs/commit/032cf2e3ee3147db5e953671e25fbebc3c35fc45)
- wip by [@jdx](https://github.com/jdx) in [68643d0](https://github.com/jdx/vfox.rs/commit/68643d01300552301dc29383afeffd41755ec78c)
- wip by [@jdx](https://github.com/jdx) in [8df7b41](https://github.com/jdx/vfox.rs/commit/8df7b4103c696272f066f03e952ff9d18b861a42)
- wip by [@jdx](https://github.com/jdx) in [b264b9e](https://github.com/jdx/vfox.rs/commit/b264b9e04a752841804cff08790a3de1794a2068)
- wip by [@jdx](https://github.com/jdx) in [3ff2588](https://github.com/jdx/vfox.rs/commit/3ff25883bd9493a71061217dfe915fa82adaf432)
- wip by [@jdx](https://github.com/jdx) in [348db8d](https://github.com/jdx/vfox.rs/commit/348db8d209e1962cfbaefcd3d891c64e9ebcdf16)
- wip by [@jdx](https://github.com/jdx) in [b1014ac](https://github.com/jdx/vfox.rs/commit/b1014ac16d07e35033f19815c035fa7ad0fc8888)
- wip by [@jdx](https://github.com/jdx) in [a2f9bf4](https://github.com/jdx/vfox.rs/commit/a2f9bf458237f8e478f6ba7d26a19132159cb4d3)
- wip by [@jdx](https://github.com/jdx) in [26aa83e](https://github.com/jdx/vfox.rs/commit/26aa83e62541ff492a1fc156c2ce53ce9bcb8825)
- cli stub by [@jdx](https://github.com/jdx) in [882fdef](https://github.com/jdx/vfox.rs/commit/882fdef500681997dd4954aa00e06d6ad5607089)
- wip by [@jdx](https://github.com/jdx) in [0007113](https://github.com/jdx/vfox.rs/commit/0007113ae50b73fb9b1559a2ba7bdae96fe9f65f)
- added list/install commands by [@jdx](https://github.com/jdx) in [51c0649](https://github.com/jdx/vfox.rs/commit/51c06497d0222e46e804bc729bef77e174560aaf)
- bump xx by [@jdx](https://github.com/jdx) in [f1bf9f7](https://github.com/jdx/vfox.rs/commit/f1bf9f7f4b0acb38e378731afa7924f56727fb32)
- bump xx by [@jdx](https://github.com/jdx) in [4c86d97](https://github.com/jdx/vfox.rs/commit/4c86d972236da085382fc9bb6e8a6ff57ba73f4b)
- download partially implemented by [@jdx](https://github.com/jdx) in [3bbfc53](https://github.com/jdx/vfox.rs/commit/3bbfc532aa9ae12bb021eb98ea118a7005a56497)
- file downloading working by [@jdx](https://github.com/jdx) in [3f6a4db](https://github.com/jdx/vfox.rs/commit/3f6a4db4d3f04935d8971bc50dc40fc962cfaac4)
- tweaks by [@jdx](https://github.com/jdx) in [20f7e8d](https://github.com/jdx/vfox.rs/commit/20f7e8d4d648a49094b5ba3aaeb6eda8a8b783bf)
- updated homedir by [@jdx](https://github.com/jdx) in [101716d](https://github.com/jdx/vfox.rs/commit/101716d7419ee91732fb71fd635c846b8fd9952f)
- added release-plz action by [@jdx](https://github.com/jdx) in [ff7bff7](https://github.com/jdx/vfox.rs/commit/ff7bff7cc5b8192dd9beb77bdc69ae6b89df41a5)
- use mise in CI by [@jdx](https://github.com/jdx) in [1e51931](https://github.com/jdx/vfox.rs/commit/1e51931e86b490f80d88b4364e03e5bd63e60040)
- added git-cliff config by [@jdx](https://github.com/jdx) in [714c039](https://github.com/jdx/vfox.rs/commit/714c039e749bbcdb8368bb392f4a12234ee0cc1a)
- clean up deps by [@jdx](https://github.com/jdx) in [a3172a9](https://github.com/jdx/vfox.rs/commit/a3172a9879b5d85aeefe7f1f63ec8cf2a079c340)
- ignore changelog for prettier by [@jdx](https://github.com/jdx) in [46ad07b](https://github.com/jdx/vfox.rs/commit/46ad07b3e2506329727f669097656ab78a4841c9)

### 📦️ Dependency Updates

- update rust crate xx to 0.3.0 by [@renovate[bot]](https://github.com/renovate[bot]) in [#6](https://github.com/jdx/vfox.rs/pull/6)
- lock file maintenance by [@renovate[bot]](https://github.com/renovate[bot]) in [#7](https://github.com/jdx/vfox.rs/pull/7)
- lock file maintenance by [@renovate[bot]](https://github.com/renovate[bot]) in [#8](https://github.com/jdx/vfox.rs/pull/8)
- update rust crate url to v2.5.2 by [@renovate[bot]](https://github.com/renovate[bot]) in [#20](https://github.com/jdx/vfox.rs/pull/20)
- update actions/checkout action to v4 by [@renovate[bot]](https://github.com/renovate[bot]) in [#19](https://github.com/jdx/vfox.rs/pull/19)
- update github artifact actions to v4 by [@renovate[bot]](https://github.com/renovate[bot]) in [#21](https://github.com/jdx/vfox.rs/pull/21)

<!-- generated by git-cliff -->
