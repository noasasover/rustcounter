# rustcounter

rustcounter is a Linux kernel module written in Rust that creates a character device at `/dev/rustcounter`.

Reading from the device returns the current counter value, and writing to the device increments the counter.

## Build

Run:

```bash
make
