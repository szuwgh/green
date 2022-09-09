#![allow(missing_docs)]

use kernel::bindings::register_filesystem;
use kernel::fsparam_u32oct;
use kernel::prelude::*;
use kernel::{str::CStr, ThisModule};

module! {
        type: HelloWorld,
        name: "hello_world",
        author: "Douglas Su",
        description: "A simple hello world example",
        license: "GPL v2",
}

#[no_mangle]
static mut ramfs_fs_type: file_system_type = file_system_type {
    name: c_str!("hellofs").as_char_ptr(),
    init_fs_context: Some(ramfs_init_fs_context),
    parameters: ramfs_fs_parameters.as_ptr(),
    kill_sb: Some(ramfs_kill_sb),
    fs_flags: RAMFS_RUST_FS_USERNS_MOUNT,
    ..c_default_struct!(file_system_type)
};

struct HelloWorld;

impl kernel::Module for HelloWorld {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        pr_info!("Hello world from rust!\n");

        Ok(HelloWorld)
    }
}

impl Drop for HelloWorld {
    fn drop(&mut self) {
        pr_info!("Bye world from rust!\n");
    }
}
