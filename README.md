# Lumen 🌄

![GitHub](https://img.shields.io/github/license/checksumdev/lumen) ![GitHub top language](https://img.shields.io/github/languages/top/checksumdev/lumen) ![Static Badge](https://img.shields.io/badge/forbid-unsafe_code-green) ![Discord](https://img.shields.io/discord/984852897051312159?label=Support%20Server)

Lumen is an **astronomically fast** ShareX uploader written in Rust using the [Actix Web](https://github.com/actix/actix-web) framework. It is designed to be **lightweight** and **secure**, providing a self-hosted solution for file uploads.

All uploads are encrypted with **AES-256-GCM-SIV** and only the uploader can decrypt the files.

## Installation 🚀

> **Note:** Lumen is currently in development and may not be stable. Use at your own risk.

### Building from source

```bash
# Clone the repository
git clone https://github.com/checksumdev/lumen.git
cd lumen

# Build the project
cargo build --release

# Configure .env
Rename .env.example to .env and change PUBLIC_URL if needed

# Run Lumen 🌄 (after you configured the .env file)
./target/release/lumen # or ./target/release/lumen.exe on Windows

# Lumen is now running on http://localhost:8080
```

## Usage 📝

To use Lumen you first need to create an user with POST request to `/register`. Format for the post request is `{"username": "USERNAME", "password": "PASSWORD"}`. The server should return you the whole user object in the database including the API key which you need to save for later use.

To configure ShareX you need to get the configuration file and change its values to match your installation and user API key. An example ShareX configuration file is provided in the /examples folder: [Lumen.sxcu](examples/Lumen.sxcu). Download the file and open it with your favourite editor. Make sure to change values for `requestUrl` to match your domain and `x-api-key` with your user API key that you saved before, alongside that make sure to edit the end of `deletionUrl` field and replace `API_KEY_HERE` with your user API key

## Nginx config ⚙️

To use Lumen with nginx you can use configuration file provided in the /examples folder: [nginx.conf](examples/nginx.conf).

Make sure to update the values in the configuration file to match your domain and Lumen installation.

```nginx
# Nginx config for lumen
server {
    listen 80;
    server_name lumen.example.com; # Change lumen.example.com to your domain, should also change it in your sharex config

    # redirect http to https
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl;
    server_name lumen.example.com; # Change lumen.example.com to your domain

    # ssl
    ssl_certificate /etc/letsencrypt/live/lumen.example.com/fullchain.pem; # Change lumen.example.com to your domain
    ssl_certificate_key /etc/letsencrypt/live/lumen.example.com/privkey.pem; # Change lumen.example.com to your domain

    # proxy
    location / {
        proxy_pass http://127.0.0.1:8080;
        include /etc/nginx/proxy_params;
    }
}
```

## Benchmarks 📊

These benchmarks were performed on a Ryzen 9 3900X with 32GB of RAM. Feel free to run the benchmarks yourself by running `cargo bench` in the project directory with a release server running.

> **WARNING:** The benchmark will create many 1MB files, and will **DELETE ALL** uploads by the test user when it is finished.

![Benchmark](assets/benchmarks.svg)

## Contributing 🤝

If you love what we're doing, **star the repository**! It helps us out a lot. If you want to contribute and have any major changes in mind, please open an issue first to discuss them. When making commits, please follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification.

<a href="https://star-history.com/#checksumdev/lumen&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=checksumdev/lumen&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=checksumdev/lumen&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=checksumdev/lumen&type=Date" />
  </picture>
</a>

## Support 💬

If you need help with Lumen, feel free to join our [Discord server](https://discord.gg/JcF9sTZPc4) or open an issue.

## License 📜

Lumen is licensed under the [Affero General Public License v3.0](LICENSE).
