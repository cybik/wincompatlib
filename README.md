<h1 align="center">🦀 wincompatlib</h1>

Rust library that contains a set of interfaces to run windows applications on unix-like systems using Wine

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
