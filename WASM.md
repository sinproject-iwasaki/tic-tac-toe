# WASM

## Referred

https://github.com/bevyengine/bevy/tree/latest/examples#examples

## Dependencies

No dynamic_linking:

```toml
[dependencies]
# bevy = { version = "0.13.0", features = ["dynamic_linking"] }
# FOR WASM BUILD:
bevy = "0.13.0"
```

## Never check asset meta

```rs
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        ...
        .run();
```

## For Android

Set scale factor override:

```rs
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: WindowResolution::new(800., 600.).with_scale_factor_override(1.0),
            ..default()
        }),
        ..default()
    })
```

## Setup

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
brew install binaryen
cargo install basic-http-server
```

and add [profile.wasm-release]

## Build

```sh
chmod +x build_wasm.sh
./build_wasm.sh
```

## Copy assets

Copy asset files to wasm directory.

## index.html

Create index.html file

## Run

Then serve `wasm` directory to browser. i.e.

```sh
basic-http-server wasm
```
