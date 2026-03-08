// use std::ffi::CStr;
use std::mem;
use std::path::Path;
// use std::ptr;
use winapi::shared::minwindef::{DWORD, FALSE};
// use winapi::shared::ntdef::NULL;
use winapi::um::dbghelp::{
    AddrModeFlat, IMAGEHLP_MODULEW64, STACKFRAME64, SYMBOL_INFOW, StackWalk64, SymFromAddrW,
    SymFunctionTableAccess64, SymGetModuleBase64, SymGetModuleInfoW64,
};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetThreadContext, OpenThread};
use winapi::um::winnt::{
    CONTEXT, CONTEXT_FULL, HANDLE, IMAGE_FILE_MACHINE_AMD64, THREAD_GET_CONTEXT,
    THREAD_QUERY_INFORMATION,
};

const MAX_SYM_NAME_LEN: usize = 512;

fn resolve_address(h_process: HANDLE, addr: u64) -> String {
    unsafe {
        // ── 1. Try to get the module that contains `addr` ──
        let mut mod_info: IMAGEHLP_MODULEW64 = mem::zeroed();
        mod_info.SizeOfStruct = mem::size_of::<IMAGEHLP_MODULEW64>() as DWORD;
        let has_module = SymGetModuleInfoW64(h_process, addr, &mut mod_info) != 0;

        // Extract just the file name from the wide-string image path
        // e.g. "C:\Windows\System32\ntdll.dll" → "ntdll.dll"
        let module_name = if has_module {
            let raw = wide_to_string(&mod_info.ImageName);
            Path::new(&raw)
                .file_name()
                .map(|f| f.to_string_lossy().into_owned())
                .unwrap_or(raw)
        } else {
            String::new()
        };

        let buf_size = mem::size_of::<SYMBOL_INFOW>() + MAX_SYM_NAME_LEN * mem::size_of::<u16>();
        let mut buf = vec![0u8; buf_size];
        let sym = buf.as_mut_ptr() as *mut SYMBOL_INFOW;
        (*sym).SizeOfStruct = mem::size_of::<SYMBOL_INFOW>() as u32;
        (*sym).MaxNameLen = MAX_SYM_NAME_LEN as u32;

        let mut sym_displacement: u64 = 0;

        // https://learn.microsoft.com/en-us/windows/win32/debug/retrieving-symbol-information-by-address
        if SymFromAddrW(h_process, addr, &mut sym_displacement, sym) != 0 {
            // We have a symbol name (wide)
            let name_slice =
                std::slice::from_raw_parts((*sym).Name.as_ptr(), (*sym).NameLen as usize);
            let sym_name = String::from_utf16_lossy(name_slice);

            if module_name.is_empty() {
                format!("{sym_name}+0x{sym_displacement:x}")
            } else {
                format!("{module_name}!{sym_name}+0x{sym_displacement:x}")
            }
        } else if has_module {
            // No exported/debug symbol – fall back to module-relative offset
            let offset = addr.wrapping_sub(mod_info.BaseOfImage);
            format!("{module_name}+0x{offset:x}")
        } else {
            // Unknown region – raw address
            format!("0x{addr:x}")
        }
    }
}

/// Convert a null-terminated wide-char (`u16`) buffer to a Rust `String`.
fn wide_to_string(buf: &[u16]) -> String {
    let len = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..len])
}

pub fn trace_thread_stack(pid: u32, h_process: HANDLE, tid: u32) {
    let h_thread: HANDLE;
    //    println!("[+] Targeting TID:\t\t{}", tid);
    unsafe {
        h_thread = OpenThread(THREAD_GET_CONTEXT | THREAD_QUERY_INFORMATION, FALSE, tid);
        if h_thread.is_null() {
            eprintln!(
                "[-] OpenThread failed for TID {} (error {})",
                tid,
                winapi::um::errhandlingapi::GetLastError()
            );
            return;
        }
        // println!("[+] Opened handle to Thread:\t{:?}", h_thread);

        loop {
            let mut ctx: CONTEXT = mem::zeroed();
            ctx.ContextFlags = CONTEXT_FULL;

            if GetThreadContext(h_thread, &mut ctx) == 0 {
                eprintln!("[!] GetThreadContext failed (error {})", GetLastError());
                break;
            }
            // println!("[+] Fetched thread context");
            let mut frame: STACKFRAME64 = mem::zeroed();
            frame.AddrPC.Offset = ctx.Rip;
            frame.AddrPC.Mode = AddrModeFlat;
            frame.AddrFrame.Offset = ctx.Rbp;
            frame.AddrFrame.Mode = AddrModeFlat;
            frame.AddrStack.Offset = ctx.Rsp;
            frame.AddrStack.Mode = AddrModeFlat;

            let mut entries: Vec<String> = Vec::new();

            loop {
                let ok = StackWalk64(
                    IMAGE_FILE_MACHINE_AMD64 as DWORD,
                    h_process,
                    h_thread as HANDLE,
                    &mut frame,
                    &mut ctx as *mut CONTEXT as *mut _,
                    None,                           // ReadMemoryRoutine
                    Some(SymFunctionTableAccess64), // FunctionTableAccessRoutine
                    Some(SymGetModuleBase64),       // GetModuleBaseRoutine
                    None,                           // TranslateAddress
                );

                if ok == 0 || frame.AddrPC.Offset == 0 {
                    break;
                }

                entries.push(resolve_address(h_process, frame.AddrPC.Offset));
            }

            println!("PID\t| TID\t| STACK FRAME");
            if entries.is_empty() {
                println!("{}\t| {}\t| ", pid, tid);
            } else {
                println!("{}\t| {}\t| {}", pid, tid, entries.join(","));
            }
            break;
        }

        if !h_thread.is_null() {
            CloseHandle(h_thread);
        }
    }
}
