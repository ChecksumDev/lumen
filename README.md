# Lumen 🌄

Lumen is an **astronomically fast** ShareX uploader written in Rust using the [Actix Web](https://github.com/actix/actix-web) framework. Lumen is designed to be **lightweight** and **secure**.

All uploads are encrypted with **AES-256-GCM-SIV** and only the uploader can decrypt the files. Lumen is designed to be self-hosted and is easy to deploy.

## Installation 🚀

> **Note:** Lumen is currently in development and may not be stable. Use at your own risk.

```bash
# Clone the repository
git clone https://github.com/checksumdev/lumen.git
cd lumen

# Build the project
cargo build --release

# Run Lumen! 🎉
./target/release/lumen # or ./target/release/lumen.exe on Windows
```

## Usage 📝

An example ShareX configuration is [provided in the examples folder](examples/Lumen.sxcu). You can import this configuration by downloading the file and opening it.

You will need to update some of the values in the configuration file to match your Lumen installation.

## Contributing 🤝

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. Please use the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for commit messages.

## License 📜

Lumen is licensed under the [Affero General Public License v3.0](LICENSE).
