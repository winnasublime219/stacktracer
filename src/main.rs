#![cfg(all(target_os = "windows", target_arch = "x86_64"))]

use std;
use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::dbghelp::{
    SYMOPT_DEFERRED_LOADS, SYMOPT_UNDNAME, SymCleanup, SymInitializeW, SymSetOptions,
};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::{HANDLE, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

mod cli;
mod procenum;
mod trace;

fn stacktrace(pid: u32, tid: u32) {
    let target_tids = procenum::collect_threads(pid, tid);

    unsafe {
        // First open a handle to the thread
        let h_process: HANDLE =
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid);
        if h_process.is_null() {
            eprintln!(
                "[!] OpenProcess failed for PID {} (error {})",
                pid,
                GetLastError()
            );
            return;
        }
        // println!("[+] Opened handle to Process:\t{:?}", h_process);

        // Initialize symbols
        SymSetOptions(SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
        if SymInitializeW(h_process, std::ptr::null(), TRUE) == 0 {
            eprintln!("[!] SymInitializeW failed (error {})", GetLastError());
            CloseHandle(h_process);
            return;
        }
        // println!("[+] Initialized Symbols!");

        for x in &target_tids {
            trace::trace_thread_stack(pid, h_process, *x);
        }

        if FALSE == SymCleanup(h_process) {
            eprintln!(
                "[!] SymCleanup failed for PID {} (error {})",
                pid,
                GetLastError()
            );
        }
        if !h_process.is_null() {
            CloseHandle(h_process);
        }
    }
}

fn main() {
    let c_args: cli::CliArgs = cli::CliArgs::parse();
    if !c_args.hide_banner {
        cli::CliArgs::banner();
        println!();
    }

    // println!("[+] Targeting PID:\t\t{}", c_args.pid);

    if c_args.pid == std::process::id() {
        eprintln!("[-] DO NOT RUN THIS PROGRAM AGAINST THE CURRENT PROCESS");
        std::process::exit(-1);
    }
    stacktrace(c_args.pid, c_args.tid);
}
