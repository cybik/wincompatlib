<h1 align="center">🦀 libaragal</h1>

Rust library that contains a set of formalized interfaces to run applications and games on unix-like systems using either Wine, Proton, or even as Native applications.

## Why though?

This library started as a simple, formalized interface to launch payloads into WINE as simply 
as possible. It then grew into this interesting mess because the old man on the team had ideas
of grandeur. As many Canadians do, he ended up apologizing for that.

## Examples

### Run cmd.exe using system wine

```rust
use wincompatlib::prelude::*;

// Run cmd.exe using system wine
Wine::default().run("cmd");

// Ask for cmd's help
let child = Wine::default().run_args(["cmd", "/c", "help"]).unwrap();

println!("Help: {}", &String::from_utf8_lossy(&child.wait_with_output().unwrap()));
```

### Print wine version

```rust
use wincompatlib::prelude::*;

// Print wine version
println!("Wine version: {:?}", Wine::default().version().unwrap());
```

### Run cmd.exe using custom wine, and then stop it

```rust
use wincompatlib::prelude::*;

let wine = Wine::from_binary("/path/to/wine");

// Run cmd.exe using custom wine
// and then stop it
wine.run("cmd");
wine.stop_processes(true);
```

### Print DXVK version

```rust
// Requires "dxvk" feature (enabled by default)
use wincompatlib::prelude::*;

match Dxvk::get_version("/path/to/prefix") {
    Ok(Some(version)) => println!("DXVK applied: {}", version),
    Ok(None) => println!("DXVK is not applied"),
    Err(err) => eprintln!("Failed to get DXVK version: {}", err)
}
```

### Install DXVK

```rust
// Requires "dxvk" feature (enabled by default)
use wincompatlib::prelude::*;

Wine::default()
    .install_dxvk("/path/to/dxvk-x.y.z", InstallParams::default())
    .expect("Failed to install DXVK");
```

Author: [Nikita Podvirnyy](https://github.com/krypt0nn)

Licensed under [MIT](LICENSE)
