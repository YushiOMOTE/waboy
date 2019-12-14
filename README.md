# waboy

Rust-generated wasm gameboy emulator.

* Based on [the no-std gameboy emulator](https://github.com/yushiomote/rgy).
* The demo is [here](http://139.180.193.221/).

## Key mapping

| Joypad | Keyboard |
|--------|----------|
| Right  | Right    |
| Left   | Left     |
| Up     | Up       |
| Down   | Down     |
| A      | Z        |
| B      | X        |
| Start  | N        |
| Select | M        |

## Build/Run

* To run on PC.

    ```
    cargo run --release
    ```

* To run on web browser.

    ```
    cargo install cargo-web     # If you don't have it yet.
    cargo web start --release
    ```
