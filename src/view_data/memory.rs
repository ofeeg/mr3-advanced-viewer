use process_memory::Architecture;
#[allow(unused_imports)]
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
#[cfg(windows)]
use winapi::shared::minwindef;
#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::ptr::null_mut;
use std::process::Child;
#[cfg(not(windows))]
use libc::{c_void, iovec, process_vm_readv, };

pub trait TryIntoProcessHandle {

    fn try_into_process_handle(&self) -> std::io::Result<ProcessHandle>;
}

pub trait CopyAddress
{
    fn copy_address(&self, addr: usize, buf: &mut [u8]) -> std::io::Result<()>;

    fn get_offset(&self, offsets: &[usize]) -> std::io::Result<usize> {
        // Look ma! No unsafes!
        let mut offset: usize = 0;
        let noffsets: usize = offsets.len();
        let mut copy = vec![0_u8; self.get_pointer_width() as usize];
        for next_offset in offsets.iter().take(noffsets - 1) {
            offset += next_offset;
            self.copy_address(offset, &mut copy)?;
            offset = self.get_pointer_width().pointer_from_ne_bytes(&copy);
        }

        offset += offsets[noffsets - 1];
        Ok(offset)
    }
    fn get_pointer_width(&self) -> Architecture;
}

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
#[cfg(windows)]
/// A `std::process::Child` has a `HANDLE` from calling `CreateProcess`.
impl TryIntoProcessHandle for Child {
    fn try_into_process_handle(&self) -> std::io::Result<ProcessHandle> {
        Ok((HANDLE(self.as_raw_handle() as _), Architecture::from_native()))
    }
}

#[cfg(windows)]
impl TryIntoProcessHandle for minwindef::DWORD {
    fn try_into_process_handle(&self) -> std::io::Result<ProcessHandle> {
        let handle = unsafe {
            winapi::um::processthreadsapi::OpenProcess(
                winapi::um::winnt::PROCESS_CREATE_THREAD
                    | winapi::um::winnt::PROCESS_QUERY_INFORMATION
                    | winapi::um::winnt::PROCESS_VM_READ
                    | winapi::um::winnt::PROCESS_VM_WRITE
                    | winapi::um::winnt::PROCESS_VM_OPERATION,
                winapi::shared::minwindef::FALSE,
                *self,
            )
        };
        if handle == (0 as winapi::um::winnt::HANDLE) {
            Err(std::io::Error::last_os_error())
        } else {
            Ok((HANDLE(handle), Architecture::from_native()))
        }
    }
}


#[cfg(not(windows))]
pub type Pid = libc::pid_t;
#[cfg(not(windows))]
pub type ProcessHandle = (HANDLE, Architecture);
#[cfg(not(windows))]
#[derive(Copy, Clone, Debug)]
pub struct HANDLE(Pid);
#[cfg(not(windows))]
impl ProcessHandleExt for ProcessHandle {
    #[must_use]
    fn check_handle(&self) -> bool {
        self.0.0 != 0
    }
    #[must_use]
    fn null_type() -> Self {
        (HANDLE(0), Architecture::from_native())
    }
    #[must_use]
    fn set_arch(self, arch: Architecture) -> Self {
        (self.0, arch)
    }
}
#[cfg(not(windows))]
/// A `Child` always has a pid, which is all we need on Linux.
impl TryIntoProcessHandle for Child {
    fn try_into_process_handle(&self) -> std::io::Result<ProcessHandle> {
        #[allow(clippy::cast_possible_wrap)]
        Ok((HANDLE(self.id() as Pid), Architecture::from_native()))
    }
}

#[cfg(not(windows))]
impl TryIntoProcessHandle for Pid {
    fn try_into_process_handle(&self) -> std::io::Result<ProcessHandle> {
        Ok((HANDLE(*self), Architecture::from_native()))
    }
}

#[cfg(not(windows))]
impl CopyAddress for ProcessHandle {
    #[allow(clippy::inline_always)]
    #[inline(always)]
    fn get_pointer_width(&self) -> Architecture {
        self.1
    }

    fn copy_address(&self, addr: usize, buf: &mut [u8]) -> std::io::Result<()> {
        let local_iov = iovec {
            iov_base: buf.as_mut_ptr() as *mut c_void,
            iov_len: buf.len(),
        };
        let remote_iov = iovec {
            iov_base: addr as *mut c_void,
            iov_len: buf.len(),
        };
        let result = unsafe { process_vm_readv(self.0.0, &local_iov, 1, &remote_iov, 1, 0) };
        if result == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
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



pub trait Memory<T>
{
    fn set_offset(&mut self, new_offsets: Vec<usize>);
    fn get_offset(&self) -> std::io::Result<usize>;
    fn read(&self) -> std::io::Result<T>;
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

}
