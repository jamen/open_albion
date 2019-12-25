#![cfg(windows)]
#![allow(non_snake_case, unused_variables)]

use std::ptr::null_mut;
use std::mem;
use std::ffi::CString;

use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::{HINSTANCE,DWORD,LPVOID,BOOL,TRUE,LPARAM,LPDWORD,WPARAM,LRESULT};
use winapi::shared::windef::HWND;

use winapi::um::processthreadsapi::{GetCurrentProcessId,CreateThread};

use winapi::um::consoleapi::AllocConsole;

use winapi::um::winnt::{DLL_PROCESS_ATTACH,DLL_PROCESS_DETACH};

use winapi::um::winuser::GWL_WNDPROC;
use winapi::um::winuser::WNDPROC;
use winapi::um::winuser::{EnumWindows,GetWindowThreadProcessId,GetWindowLongPtrA,SetWindowLongPtrA,DefWindowProcA};

#[derive(Debug)]
struct FableWindowSearch {
    process_id: DWORD,
    hwnd: HWND
}

static mut FABLE_WND_PROC: WNDPROC = None;

#[no_mangle]
extern "system" fn DllMain(dll_handle: HINSTANCE, fdv_reason: DWORD, lpv_reserved: LPVOID) -> BOOL {
    match fdv_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { CreateThread(null_mut(), 0, Some(init), null_mut(), 0, null_mut()) };
        },
        DLL_PROCESS_DETACH => {},
        _ => {}
    }
    TRUE
}

extern "system" fn init(lpThreadParameter: LPVOID) -> DWORD {
    // unsafe { AllocConsole() };

    // Fable window search
    // let process_id = unsafe { GetCurrentProcessId() };

    // let mut fable_window_search = FableWindowSearch {
    //     process_id: process_id,
    //     hwnd: null_mut(),
    // };

    // while fable_window_search.hwnd == null_mut() {
    //     unsafe { EnumWindows(Some(find_fable_window), &mut fable_window_search as *mut FableWindowSearch as LPARAM) };
    // }

    // // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    // // note: mutable statics can be mutated by multiple threads: aliasing violations or data races will cause undefined
    // unsafe {
    //     FABLE_WND_PROC = Some(*(&GetWindowLongPtrA(fable_window_search.hwnd, GWL_WNDPROC) as *const _ as *const WNDPROC));
    // }

    // unsafe {
    //     SetWindowLongPtrA(fable_window_search.hwnd, GWL_WNDPROC, mem::transmute::<WNDPROC, LONG_PTR>(Some(wnd_proc_hook) as i32));
    // }

    // unsafe { ExitProcess(0) };

    0
}

// Fable window search callbacks

// extern "system" fn find_fable_window(hwnd: HWND, search: LPARAM) -> BOOL {
//     let mut search = unsafe { &mut *(search as *mut FableWindowSearch) };

//     let mut process_id = 1 as DWORD;

//     unsafe { GetWindowThreadProcessId(hwnd, &mut process_id as LPDWORD) };

//     if process_id == search.process_id {
//         search.hwnd = hwnd;
//         0 as BOOL
//     } else {
//         1 as BOOL
//     }
// }

// unsafe extern "system" fn wnd_proc_hook(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
//     match FABLE_WND_PROC {
//         Some(wnd_proc) => {
//             wnd_proc(hwnd, msg, wparam, lparam)
//         }
//         None => {
//             DefWindowProcA(hwnd, msg, wparam, lparam)
//         }
//     }
// }