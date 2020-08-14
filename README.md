# ble-mesh
![Rust](https://github.com/mfiumara/ble-mesh/workflows/Rust/badge.svg)

A rust bluetooth mesh implementation for Linux. The goal of this repo is to achieve rust bindings for bluez mesh, as opposed to the more common way of interfacing with dbus. The resulting rust bindings are cleaner and easier to use and understand than bindings created for dBus. This repo does not aim to provide support for Windows or macOS and is intended for Linux platforms only. I am developing this under WSL2 ubuntu and Manjaro Linux and aim to support at least these two distributions.

## Dependencies

In order to run this package you will need to build bluez from source and compile in mesh support. Besides just building bluez, mesh additionally needs some cryptography support built into the kernel.

### Kernel requirements

1. A minimum of kernel version 4.9 or later is required

2. The kernel must at a minimum have the following .config options turned on:
	CONFIG_CRYPTO_USER
	CONFIG_CRYPTO_USER_API
	CONFIG_CRYPTO_USER_API_AEAD
	CONFIG_CRYPTO_USER_API_HASH

	CONFIG_CRYPTO_AES
	CONFIG_CRYPTO_CCM
	CONFIG_CRYPTO_AEAD
	CONFIG_CRYPTO_CMAC

To check if your kernel supports the above, you can issue the following:

```
zcat /proc/config.gz | grep CONFIG_CRYPTO_USER
zcat /proc/config.gz | CONFIG_CRYPTO_AES
```

If one of the kernel options has a line which looks like `# CONFIG_CRYPTO_USER is not set`, you will have to rebuild your kernel. [Check this guide](https://www.maketecheasier.com/build-custom-kernel-ubuntu/) on how to rebuild a kernel on ubuntu.

### Bluez dependencies

This crate builds bluez from source using the rust build script. In order to build bluez you need to install bluez's build dependencies. On debian-based distros (Ubuntu) install the required dependencies as follows:

```bash
sudo apt-get build-dep bluez
sudo apt-get install libjson-c-dev clang llvm
```

## Building ble-mesh

Once you adhere to above dependencies, you'll need to bootstrap bluez once:

```bash
cd modules/bluez && ./bootstrap
```

Then you can build the crate as usual using `cargo`:

```bash
cargo build
```

This will bootstrap bluez, then builds and links the libraries against the rust bindings.
