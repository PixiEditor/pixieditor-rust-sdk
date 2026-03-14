# pixieditor-rust-sdk
An experimental SDK for experimental PixiEditor Extensions

It is far from being complete, but it's already useful if you don't need to build UIs with FlyUI.

The main benefit of using Rust instead of primarily developed C# SDK is the bundle size and WASI compilation support. C# sdk currently produces a blob that weighs about 9MB uncompressed, whereas Rust ~20KB.

`lib.rs` is the main part where you can do something, it contains raw ABI exports, imports and safe wrappers for them. 

# Hello world

in `lib.rs`

do
```rs
pub extern "C" fn initialize() {
    println!("Hello world!");
}
```

# Compilation

Follow https://wasmbyexample.dev/examples/wasi-hello-world/wasi-hello-world.rust.en-us

# .pixiext

Currently this SDK does not bundle any scripts producing .pixiext, but it's as simple as zipping produced .wasm along with extension.json and renaming the archive to `YourExtName.pixiext`

Sample `extension.json`

```json
{
  "displayName": "Sample Extension - Hello World",
  "uniqueName": "yourCompany.Samples.HelloWorld",
  "description": "Sample extension for PixiEditor",
  "version": "1.0.0",
  "author": {
    "name": "PixiEditor",
    "email": "info@pixieditor.net",
    "website": "https://pixieditor.net"
  },
  "publisher": {
    "name": "PixiEditor",
    "email": "info@pixieditor.net",
    "website": "https://pixieditor.net"
  },
  "contributors": [
    {
      "name": "flabbet",
      "email": "some@mail.com",
      "website": "https://github.com/flabbet"
    }
  ],
  "license": "MIT",
  "categories": [
    "Extension"
  ]
}
```

put .pixiext to `%localappdata%/PixiEditor/Extensions/`
