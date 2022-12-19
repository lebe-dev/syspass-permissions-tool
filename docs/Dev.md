# For developers

## Getting started

1. Use `docker-compose.yml` to deploy sysPass dev instance.

2. Prepare `spt.yml` file.

## How to build

1. Install [Rust](https://www.rust-lang.org/tools/install)

2. Build:

    ```shell
    cargo build --release && mv build/release/syspass-permissions-tool spt
    ```
