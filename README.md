# Simctl

This crates provides a safe wrapper around the `simctl` utility that ships with
Xcode.

---

🚨 __Important:__ this library only works if Xcode is installed and currently
only works with Xcode 12.

---

## Example

```rust
use simctl::{Simctl, DeviceQuery};

let simctl = Simctl::new();
let device = simctl.list()?.devices().iter()
    .available()
    .by_name("iPhone SE (2nd generation)")
    .next().unwrap();
let _ = device.boot();
device.launch("com.apple.mobilesafari").exec()?;
let image = device.io().screenshot(
    simctl::io::ImageType::Png,
    simctl::io::Display::Internal,
    simctl::io::Mask::Ignored,
)?;
device.shutdown()?;
```

## Operations

The following operations are currently supported by this crate. For a full list
of operations that are available in the original CLI, run `xcrun simctl`.

### Supported Operations

- [x] boot
- [x] get_app_container
- [x] getenv
- [x] install
- [x] io screenshot
- [x] keychain reset
- [x] launch
- [x] list
- [x] openurl
- [x] privacy
- [x] push
- [x] shutdown
- [x] status_bar
- [x] terminate
- [x] ui
- [x] uninstall

### Unsupported Operations

- [ ] addmedia
- [ ] clone
- [ ] create
- [ ] delete
- [ ] diagnose
- [ ] erase
- [ ] icloud_sync
- [ ] install_app_data
- [ ] io enumerate
- [ ] io poll
- [ ] io recordVideo
- [ ] keychain add-cert
- [ ] keychain add-root-cert
- [ ] logverbose
- [ ] pair
- [ ] pair_activate
- [ ] pbcopy
- [ ] pbpaste
- [ ] pbsync
- [ ] rename
- [ ] spawn
- [ ] unpair
- [ ] upgrade
