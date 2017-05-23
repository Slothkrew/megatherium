# megatherium
Slothbot just got an upgrade!

## How to run
---
#### For everyone
* Create a `config.json` file with the following:
    ```
    {
        "bot_token": "<the bot token>",
        "sqlite_path": "<path to the sqlite database>"
    }
    ```
* See the platform specifics below.

#### Windows
* On windows get [openssl (the installer for developers)](https://slproweb.com/products/Win32OpenSSL.html).
* Set the `INCLUDE` and `LIBPATH` environment variables to point to the `<openssl dir>\include` folder.
* Add a `config.json` containing the `bot_token`.
* Run with `cargo run` from the root directory.

#### Ubuntu 16.04 LTS
* Install `libsqlite3-dev` using `aptitude`.
* Add a `config.json` containing the `bot_token`.
* Run with `cargo run` from the root directory.