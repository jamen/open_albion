// use winapi::shared::windef::*;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::*;
use winapi::um::*;

use processthreadsapi::*;

use std::ptr::null_mut;
use std::ffi::CString;

pub struct Injector {
    pub process_handle: HANDLE,
    pub thread_handle: HANDLE
}

impl Injector {
    pub fn create_process(executable_path: &str) -> Result<Self, u32> {
        let mut process_info: PROCESS_INFORMATION = Default::default();
        let mut startup_info: STARTUPINFOA = Default::default();

        startup_info.cb = std::mem::size_of::<STARTUPINFOA>() as u32;

        if unsafe {
            CreateProcessA(
                null_mut(),
                executable_path.as_ptr() as LPSTR,
                null_mut(),
                null_mut(),
                0,
                0, // winbase::CREATE_SUSPENDED,
                null_mut(),
                null_mut(),
                &mut startup_info,
                &mut process_info,
            )
        } == 0 {
            return Err(1)
        }

        Ok(
            Injector {
                process_handle: process_info.hProcess,
                thread_handle: process_info.hThread,
            }
        )
    }

    pub fn inject_dll(&mut self, dll_path: &str) -> Result<(), u32> {
        let dll_path_in_remote = unsafe {
            memoryapi::VirtualAllocEx(
                self.process_handle,
                null_mut(),
                dll_path.len() + 1,
                winnt::MEM_RESERVE | winnt::MEM_COMMIT,
                winnt::PAGE_EXECUTE_READWRITE
            )
        };

        if dll_path_in_remote.is_null() {
            return Err(2)
        }

        if unsafe {
            memoryapi::WriteProcessMemory(
                self.process_handle,
                dll_path_in_remote,
                CString::new(dll_path).unwrap().as_ptr() as LPVOID,
                dll_path.len() + 1,
                null_mut()
            )
        } == 0 {
            return Err(3)
        }

        let load_library_in_remote = {
            let module_name = CString::new("kernel32.dll").unwrap();
            let proc_name = CString::new("LoadLibraryA").unwrap();
            let module_handle = unsafe { libloaderapi::GetModuleHandleA(module_name.as_ptr()) };
            unsafe { libloaderapi::GetProcAddress(module_handle, proc_name.as_ptr()) }
        };

        if load_library_in_remote.is_null() {
            return Err(4)
        }

        let remote_thread: HANDLE = unsafe {
            CreateRemoteThread(
                self.process_handle,
                null_mut(),
                0,
                Some(*(&load_library_in_remote as *const _ as *const unsafe extern "system" fn(LPVOID) -> DWORD)),
                dll_path_in_remote,
                0,
                null_mut(),
            )
        };

        unsafe {
            synchapi::WaitForSingleObject(remote_thread, winbase::INFINITE)
        };

        if !self.process_handle.is_null() {
            unsafe { handleapi::CloseHandle(self.process_handle) };
        }

        if !self.thread_handle.is_null() {
            unsafe { handleapi::CloseHandle(self.thread_handle) };
        }

        Ok(())
    }
}