# rustcounter

rustcounter is a small Linux kernel module written in Rust.

It builds as an out-of-tree kernel module and logs a message when loaded or unloaded.

## Build

Run:

make RUSTC=/usr/bin/rustc

## Run

Run:

sudo insmod rustcounter.ko
sudo dmesg | tail

Expected log:

rustcounter: loaded 1 times

## Unload

Run:

sudo rmmod rustcounter
sudo dmesg | tail

Expected log:

rustcounter: module unloaded

## Design Notes

This module uses Rust inside the Linux kernel. It defines a RustCounter module using the module macro and implements kernel::Module.

The counter is stored as a static AtomicU64. Because the module is fully unloaded with rmmod, the counter resets when the module is loaded again.

## Future Work

- Add a /dev/rustcounter character device
- Make writes increment the counter
- Make reads return the current count
- Add a reset command
- Add a /proc entry

## License

GPL-2.0
