# Perif

An application to control lightning, sidetone and check battery status on your peripherals.


## Install

Before you install Perif, make sure you have installed:
* meson
* ninja
* Rust (cargo)

If you do not want/need support for bluetooth devices, you can choose to not include the bluetooth udev rules by passing `-Dbluetooth=false` to `meson build`.

```terminal
git clone https://github.com/olback/perif && cd perif
cargo build --release
cargo run --release --bin gen-rules
meson build
sudo ninja -C build install
```


## Uninstall
```terminal
sudo ninja -C uninstall
```


### Does your device not show up?

If your device is supported, but does not show up? Please open an issue, make sure to include make, model, vid and pid. If your device is not yet supported by Perif, you can add it by following the steps [here](ADD_DEVICE.md).


### Inspiration

This project is inspired by [Sapd/HeadsetControl](https://github.com/Sapd/HeadsetControl).
