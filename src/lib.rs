use core::ffi;
use std::mem;

use windows::{Win32::{Foundation, System::{Diagnostics::Debug, Threading}}, core::Error};


pub fn open_process(pid: u32) -> Result<Foundation::HANDLE, Error> {
    const FULL_ACCESS: u32 = 0x1F0FFF;

    let result = unsafe {
        Threading::OpenProcess(
            Threading::PROCESS_ACCESS_RIGHTS(FULL_ACCESS),
            false,
            pid,
        )
    };

    result.map_err(|err| err.into())
}


pub fn read_value_from_address<T>(process_handle: Foundation::HANDLE, address: u64) -> Result<T, Error>
where
    T: Copy,
{
    let mut buffer: T = unsafe { mem::zeroed() };
    let buffer_pointer = &mut buffer as *mut T;

    let bytes_read: usize = 0;

    let result = unsafe {
        Debug::ReadProcessMemory(
            process_handle,
            address as *const ffi::c_void,
            buffer_pointer as *mut ffi::c_void,
            mem::size_of::<T>(),
            Some(&bytes_read as *const usize as *mut usize),
        )
    };

    result.map(|_| buffer).map_err(|e| e.into())
}


pub fn write_value_to_address<T>(process_handle: Foundation::HANDLE, address: u64, value: T) -> Result<(), Error>
where
    T: Copy,
{
    let buffer_pointer = &value as *const T;

    let bytes_written: usize = 0;

    let result = unsafe {
        Debug::WriteProcessMemory(
            process_handle,
            address as *mut ffi::c_void,
            buffer_pointer as *const ffi::c_void,
            mem::size_of::<T>(),
            Some(&bytes_written as *const usize as *mut usize),
        )
    };

    result.map_err(|err| err.into())
}


#[derive(Debug, Clone, Copy)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
