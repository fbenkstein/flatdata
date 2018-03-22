extern crate diff;
extern crate memmap;

pub mod helper;
#[macro_use]
pub mod bytereader;
#[macro_use]
pub mod bytewriter;
#[macro_use]
mod archive;
mod arrayview;
mod error;
mod filestorage;
mod memory;
mod memstorage;
mod multiarrayview;
mod storage;
mod vector;

pub use archive::*;
pub use arrayview::ArrayView;
pub use error::*;
pub use filestorage::FileResourceStorage;
pub use memstorage::MemoryResourceStorage;
pub use multiarrayview::MultiArrayView;
pub use storage::{create_external_vector, MemoryDescriptor, ResourceStorage};
pub use vector::*;
