use libc::{c_void, sysctlbyname};
use std::ffi::CString;
use std::io;
use std::mem;
use std::ptr;

fn main() {
    let name = CString::new("hw.acpi.video.lcd0.active").unwrap();
    let value: i32 = 0;
    let size = mem::size_of::<i32>();

    let ret = unsafe {
        sysctlbyname(
            name.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            &value as *const _ as *const c_void,
            size,
        )
    };

    if ret == -1 {
        eprintln!("Failed to turn off display: {}", io::Error::last_os_error());
    } else {
        println!("Display turned off successfully.");
    }
}
