# Changelog

## 0.1.0 (2023-10-31)


### ⚠ BREAKING CHANGES

* headers cannot have `_` in them.
* refactor api structure, header names, add delete and purge endpoints.

### Features

* add `/config` endpoint for generating configs ([3ca49f4](https://github.com/ChecksumDev/lumen/commit/3ca49f4b050ef0ceb893e1ab4695bb94444f2f06))
* init ([3e9006d](https://github.com/ChecksumDev/lumen/commit/3e9006db6dbf2649056f86c8cdc5bbb732c22d73))
* optional encryption (opt-out) ([4697443](https://github.com/ChecksumDev/lumen/commit/4697443be243e69b2d3d8b4d443311f3277d7a66))
* refactor api structure, header names, add delete and purge endpoints. ([5428de1](https://github.com/ChecksumDev/lumen/commit/5428de110bd6ac24abbb348e885217471f99f2ff))


### Bug Fixes

* actually update their quota ([31d7408](https://github.com/ChecksumDev/lumen/commit/31d7408f9ce42c07aef53f31fb65297df138c8fe))
* add payload limit ([cb4cc6b](https://github.com/ChecksumDev/lumen/commit/cb4cc6b24016093c027d046769047c3f935b880f))
* better logging + forbid unsafe code ([1b2c22c](https://github.com/ChecksumDev/lumen/commit/1b2c22c959904e92a6dd2daeaa3c275a8e53b64f))
* centralized storage ([a2bf524](https://github.com/ChecksumDev/lumen/commit/a2bf524265b24ef0bc03399760203a064b0e525e))
* headers cannot have `_` in them. ([3d0b00b](https://github.com/ChecksumDev/lumen/commit/3d0b00b176f00417738ce2726ce254f5802bf7f3))
* input validation on registration ([18cb7d6](https://github.com/ChecksumDev/lumen/commit/18cb7d69867fe7cd41e95e2c8b25ed9a9833a4f2))
* move uploads to `data/uploads` and database to `data` ([ebaaa87](https://github.com/ChecksumDev/lumen/commit/ebaaa876f670a34e2c152ab8865ed232574290f2))
* use proper generation methods for `key` and `nonce`. ([2d33cbb](https://github.com/ChecksumDev/lumen/commit/2d33cbb723396bc424f77b4a891be32c6fe3a8a6))


### Performance Improvements

* add release profile and upload benchmark ([b466add](https://github.com/ChecksumDev/lumen/commit/b466add7729dca031b77e173506ea344cf293a75))
* improve encryption module ([7ca3057](https://github.com/ChecksumDev/lumen/commit/7ca3057bd8bcdd6631cd89953011eea61f71b8c7))
* more detailed benchmark ([a5e9aa3](https://github.com/ChecksumDev/lumen/commit/a5e9aa31a84f7bae676ea6ce65263adccd0e700c))
* more efficient `/` endpoint ([f75947a](https://github.com/ChecksumDev/lumen/commit/f75947a2a08df86516f075d2bbe514796c4c37d8))
