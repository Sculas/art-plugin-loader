# art-plugin-loader

A very simple plugin loader for the Android Runtime (ART) that bypasses the `-Xplugin` requirement.

I haven't tested all versions, but this should work on **Android 8.0+**. Testing is appreciated!

## Usage

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
art-plugin-loader = { git = "https://github.com/Sculas/art-plugin-loader" }

[build-dependencies]
cc = "1.2"
```

Create a new `plugin.cpp` file in a `cpp` directory with the following contents:

```cpp
#include "runtime.h" // art/runtime/runtime.h (how you get these headers is up to you)

extern "C" bool ArtPlugin_Initialize() {
    art::Runtime* runtime = art::Runtime::Current();
    // ...
    return true;
}

extern "C" bool ArtPlugin_Deinitialize() {
    // ...
    return true;
}
```

Then, in your `build.rs` file, add the following to link your plugin:

```rs
fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .file("cpp/plugin.cpp")
        .compile("myartplugin");
}
```

Then, in your `lib.rs` file, add the following to load your plugin:

```rs
use art_plugin_loader::ArtPluginLoader;

fn main() {
    let loader = ArtPluginLoader::new().expect("failed to initialize plugin loader");
    let plugin_path = ... // get the path to the currently loaded library
    loader.load_plugin(plugin_path).expect("failed to load plugin");
}
```

## License

This project is licensed under the MIT license.
