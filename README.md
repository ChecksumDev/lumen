# Lumen 🌄

Lumen is an **astronomically fast** ShareX uploader written in Rust using the [Actix Web](https://github.com/actix/actix-web) framework. It is designed to be **lightweight** and **secure**, providing a self-hosted solution for file uploads.

All uploads are encrypted with **AES-256-GCM-SIV** and only the uploader can decrypt the files.

## Installation 🚀

> **Note:** Lumen is currently in development and may not be stable. Use at your own risk.

To install Lumen, follow these steps:

1. Clone the repository:

   ````bash
   git clone https://github.com/checksumdev/lumen.git
   cd lumen
   ```

2. Build the project:

   ````bash
   cargo build --release
   ```

3. Run Lumen:

   ````bash
   ./target/release/lumen # or ./target/release/lumen.exe on Windows
   ```

## Usage 📝

To use Lumen, you need to configure ShareX. An example ShareX configuration file is provided in the examples folder: [Lumen.sxcu](examples/Lumen.sxcu). Download the file and open it.

Make sure to update the values in the configuration file to match your Lumen installation.

## Benchmarks 📊

These benchmarks were performed on a Ryzen 9 3900X with 32GB of RAM. Feel free to run the benchmarks yourself by running `cargo bench` in the project directory.

![Benchmark](assets/benchmarks.svg)

## Contributing 🤝

We welcome pull requests from the community. If you have any major changes, please open an issue first to discuss them. When making commits, please follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

## License 📜

Lumen is licensed under the [Affero General Public License v3.0](LICENSE).
