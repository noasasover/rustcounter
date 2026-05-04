// SPDX-License-Identifier: GPL-2.0

use kernel::prelude::*;
use core::sync::atomic::{AtomicU64, Ordering};

module! {
    type: RustCounter,
    name: "rustcounter",
    author: "Noa",
    description: "Simple counter module",
    license: "GPL",
}

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct RustCounter;

impl kernel::Module for RustCounter {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let val = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
        pr_info!("rustcounter: loaded {} times\n", val);
        Ok(RustCounter)
    }
}

impl Drop for RustCounter {
    fn drop(&mut self) {
        pr_info!("rustcounter: module unloaded\n");
    }
}
