# bluetooth-mesh

![Rust](https://github.com/mfiumara/ble-mesh/workflows/Rust/badge.svg)

A rust bluetooth mesh implementation for linux. The goal for this repo is to achieve rust bindings for the bluez mesh cfgclient, as opposed to the more common way of integrating with dbus. This should result in a more cleaner easier to understand rust API. This repo does not aim to provide support for Windows or macOS and is intended for linux platforms only.

## Dependencies

This repo tries to find some dependencies on the host system using the `pkc_config` crate. In order to find these the following packages must be installed on the system:

```
libell-dev
json-c
```

Other libraries needed for compilation are `libbluetooth-internal.la` and `libshared-ell.la`, which have to be built from the included bluez / ell git submodules. For now building bluez by itself is necessary to link against these libraries, as it seems unnecessary to port the makefiles to the rust build script.
