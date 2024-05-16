## Machine UUID

#### A library that retrieves UUID for a machine

### OS Support

1. Windows - depends on [WMIC](https://www.dedoimedo.com/computers/windows-wmic.html)
2. Linux - depends on [/etc/machine-id](http://man7.org/linux/man-pages/man5/machine-id.5.html)
3. macOS - depends on [ioreg](https://www.commandlinefu.com/commands/view/24201/get-hardware-uuid-in-mac-os-x)

### Usage

```rust
let uuid = machineid::get();

// Based on OS, UUID format will differ
// Windows
assert_eq!("140EF834-2DB3-0F7A-27B4-4CEDFB73167C", uuid);

// Based on OS, UUID format will differ
// Linux
assert_eq!("92cc698195f84d3b85f1cfb0a09e957f", uuid);

// Based on OS, UUID format will differ
// macOS
assert_eq!("F7FA2B78-F7D4-5B1B-A4E3-BACB1BBD95A1", uuid)
```

### TODO

1. [Adhere to API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html)
2. Improve error reporting.
3. Improve test for Linux UUID to use -

### _Add this to your Cargo.toml:_

```toml
[dependencies]
machine_uuid = "0.2.0"
```
