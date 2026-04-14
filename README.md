<img src="https://raw.githubusercontent.com/nor0x/Untranslocator-XMac/main/art/logo.png" width="200px" />

# Untranslocator
[![Build Status](https://nor0x.visualstudio.com/Untranslocator/_apis/build/status/nor0x.Untranslocator-XMac?branchName=main)](https://nor0x.visualstudio.com/Untranslocator/_build/latest?definitionId=9&branchName=main) [![](https://img.shields.io/nuget/vpre/nor0x.Untranslocator.svg)](https://nuget.org/packages/nor0x.Untranslocator)
[![](https://img.shields.io/nuget/dt/nor0x.Untranslocator)](https://nuget.org/packages/nor0x.Untranslocator)

Untranslocator allows to get the original bundle path from a translocated Xamarin.Mac app. 
## Usage
if a macOS app is translocated `NSBundle.MainBundle.BundlePath` returns a randomized read-only location - something like:
```
/private/var/folders/xx/xxxxxxxxxxx/x/AppTranslocation/xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx/x/MyApp.app
```
we can use this translocated path and use an object of `Untranslocator` to retrieve the original bundle path (i.e. `/Users/nor0x/Downloads`)
```cs
var untrans = new Untranslocator();
var path = untrans.ResolveTranslocatedPath(NSBundle.MainBundle.BundlePath);
```

## Rust usage
This repository now also ships a Cargo package, `untranslocator`, for macOS.

```rust
use std::path::Path;

let original = untranslocator::resolve_translocated_path(Path::new("/Applications/MyApp.app"))?;
```

The crate is macOS-only and links to the native static library from `Untranslocator.Lib`.

## Building the native library
Prerequisites:
- macOS
- Xcode command line tools (`xcodebuild`)

Build the native static library and exported C headers:

```bash
./Untranslocator.Lib/build-lib.sh
```

Generated outputs:
- `Untranslocator.Lib/build/output/libUntranslocator.a`
- `Untranslocator.Lib/build/output/include/Untranslocator.h`
- `Untranslocator.Lib/build/output/include/untranslocator_c.h`

Architecture control:
- By default, the script builds `arm64` and `x86_64` and combines them with `lipo`.
- Override architectures with `UNTRANSLOCATOR_ARCHS`, for example:

```bash
UNTRANSLOCATOR_ARCHS=arm64 ./Untranslocator.Lib/build-lib.sh
```

## Building and testing the Rust crate
From repository root:

```bash
cargo test
```

`build.rs` automatically invokes `Untranslocator.Lib/build-lib.sh` and links `libUntranslocator.a`.

## Release workflow
- Validate native and Rust outputs with `cargo test`.
- Package the crate locally:

```bash
cargo package
```

- Publish when ready:

```bash
cargo publish
```

more info on App Translocation:

https://www.synack.com/blog/untranslocating-apps
