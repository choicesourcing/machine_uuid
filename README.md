# Machine UUID
### A library that retrieves UUID for a machine

## OS Support
1. Windows - depends on [WMIC](https://www.dedoimedo.com/computers/windows-wmic.html)
2. Linux - depends on [/etc/machine-id](http://man7.org/linux/man-pages/man5/machine-id.5.html)

## Usage
```
let uuid = machineid::get();

// Based on OS, UUID format will differ
// Windows
assert_eq!("140EF834-2DB3-0F7A-27B4-4CEDFB73167C", uuid);
 
// Based on OS, UUID format will differ
// Linux
assert_eq!("92cc698195f84d3b85f1cfb0a09e957f", uuid);
```

## TODO
1. (Adhere to API Guidelines)[https://rust-lang.github.io/api-guidelines/checklist.html]
2. More tests...

## Add this to your Cargo.toml:
```
[dependencies]
machine_uuid = "0.1.0"
```
