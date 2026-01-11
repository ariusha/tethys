#![no_std]
#![feature(unique_rc_arc)]
#![feature(try_trait_v2)]
extern crate alloc;
pub mod exchange;
pub mod message;
// pub mod serialise;
pub mod state;
/*
pub mod interface;
pub mod kickstart;
pub mod memory;
pub mod message2;
pub mod process;
pub mod scheduler;
pub mod serialise;
pub mod tag;
pub mod task;
pub mod tethys;
*/
#[ostd::main]
pub fn main() {
    ostd::prelude::println!(
        "entered saltwater tethys kernel v{}!",
        env!("CARGO_PKG_VERSION")
    );
    // let kickstart = Arc::new(kickstart::kickstart());
    // kickstart.run();
    ostd::prelude::println!(
        "successfully executed saltwater tethys kernel! powering off with success exit code..."
    );
    // ostd::power::poweroff(ostd::power::ExitCode::Success);
}
