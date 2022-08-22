use process_memory::{Memory, CopyAddress, PutAddress,TryIntoProcessHandle, Architecture,};
#[allow(unused_imports)]
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
#[cfg(windows)]
use winapi::shared::minwindef;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::ptr::null_mut;


pub trait ProcessHandleExt {
    /// Returns `true` if the `ProcessHandle` is not null, and `false` otherwise.
    fn check_handle(&self) -> bool;
    /// Return the null equivalent of a `ProcessHandle`.
    #[must_use]
    fn null_type() -> ProcessHandle;
    /// Set this handle to use some architecture
    #[must_use]
    fn set_arch(self, arch: Architecture) -> Self;
}

#[cfg(windows)]
pub type Pid = minwindef::DWORD;
//#[cfg(windows)]
//pub type ProcessHandle = (winapi::um::winnt::HANDLE, Architecture);
#[cfg(windows)]
#[derive(Copy, Clone, Debug)]
pub struct HANDLE(winapi::um::winnt::HANDLE);
#[cfg(windows)]
unsafe impl Send for HANDLE{}
#[cfg(windows)]
unsafe impl Sync for HANDLE{}
#[cfg(windows)]
#[derive(Debug, Clone, Copy)]
pub type ProcessHandle = (HANDLE, Architecture);

#[cfg(windows)]
impl ProcessHandleExt for ProcessHandle {
    #[must_use]
    fn check_handle(&self) -> bool {
        self.0.0.is_null()
    }
    #[must_use]
    fn null_type() -> ProcessHandle {
        (HANDLE(null_mut()), Architecture::from_native())
    }
    #[must_use]
    fn set_arch(self, arch: Architecture) -> Self {
        (self.0, arch)
    }
}

#[cfg(not(windows))]
pub type Pid = libc::pid_t;
#[cfg(not(windows))]
pub type ProcessHandle = (Pid, Architecture);
#[cfg(not(windows))]
impl ProcessHandleExt for ProcessHandle {
    #[must_use]
    fn check_handle(&self) -> bool {
        self.0 != 0
    }
    #[must_use]
    fn null_type() -> Self {
        (0, Architecture::from_native())
    }
    #[must_use]
    fn set_arch(self, arch: Architecture) -> Self {
        (self.0, arch)
    }
}

#[derive(Clone, Debug)]
pub struct DataMember<T> {
    offsets: Vec<usize>,
    process: ProcessHandle,
    _phantom: std::marker::PhantomData<*mut T>,
}

impl<T: Sized + Copy> DataMember<T> {
    #[must_use]
    pub fn new(handle: ProcessHandle) -> Self {
        Self {
            offsets: Vec::new(),
            process: handle,
            _phantom: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn new_offset(handle: ProcessHandle, offsets: Vec<usize>) -> Self {
        Self {
            offsets,
            process: handle,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Sized + Copy> Memory<T> for DataMember<T> {
    fn set_offset(&mut self, new_offsets: Vec<usize>) {
        self.offsets = new_offsets;
    }

    fn get_offset(&self) -> std::io::Result<usize> {
        self.process.get_offset(&self.offsets)
    }

    fn read(&self) -> std::io::Result<T> {
        let offset = self.process.get_offset(&self.offsets)?;
        // This can't be [0_u8;size_of::<T>()] because no const generics.
        // It will be freed at the end of the function because no references are held to it.
        let mut buffer = vec![0_u8; std::mem::size_of::<T>()];
        self.process.copy_address(offset, &mut buffer)?;
        Ok(unsafe { (buffer.as_ptr() as *const T).read_unaligned() })
    }

    fn write(&self, value: &T) -> std::io::Result<()> {
        use std::slice;
        let offset = self.process.get_offset(&self.offsets)?;
        let buffer: &[u8] =
            unsafe { slice::from_raw_parts(value as *const _ as _, std::mem::size_of::<T>()) };
        self.process.put_address(offset, &buffer)
    }
}
