//! Nebulet ABI

/// Various test ABIs.
pub mod test;
/// ABIs for working with processes.
pub mod process;
/// ABIs for IPC
pub mod ipc;
/// ABIs for manipulating handles
pub mod handle;
/// ABIs for random numbers
pub mod rand;
/// Intrinsics
pub mod intrinsics;
/// ABIs for I/O
pub mod io;
/// ABIs for drivers
pub mod driver;
/// ABIs for irqs
pub mod interrupt;
/// ABIs for events
pub mod event;
/// ABIs for threads
pub mod thread;
/// ABIs for pretty fast exclusion
pub mod pfex;
/// ABIs for interfacing with generic objects
pub mod object;
// /// ABIs for services
// pub mod service;