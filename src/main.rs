use std;
mod procenum;
mod trace;

use winapi::shared::minwindef::{FALSE, TRUE};
use winapi::um::dbghelp::{
    SYMBOL_INFOW, SYMOPT_DEFERRED_LOADS, SYMOPT_UNDNAME, SymInitializeW, SymSetOptions,
};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

fn usage() {
    println!("[+] Usage:\n\tstackfinder.exe PID <TID>\n");
    println!("[+] Description:\n\tPrint the stack trace of a Thread of a given process\n");
    println!(
        "[+] Values:\n\tPID\tPID of process to target\n\tTID\t[Optional] TID of the target thread. If not specified,\n\t\tit will print the stack trace of all current threads."
    );
    std::process::exit(-1);
}

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
        println!("[+] Opened handle to Process:\t{:?}", h_process);

        // Initialize symbols
        SymSetOptions(SYMOPT_UNDNAME | SYMOPT_DEFERRED_LOADS);
        if SymInitializeW(h_process, std::ptr::null(), TRUE) == 0 {
            eprintln!("[!] SymInitializeW failed (error {})", GetLastError());
            CloseHandle(h_process);
            return;
        }
        println!("[+] Initialized Symbols!");

        for x in &target_tids {
            trace::trace_thread_stack(h_process, *x);
        }
        if !h_process.is_null() {
            CloseHandle(h_process);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if (args.len() < 2) || (args.len() > 3) {
        usage();
    }

    let pid: u32;
    let mut tid: u32 = 0;

    pid = match args[1].parse() {
        Ok(id) => id,
        Err(_e) => {
            eprintln!("[-] Provided PID is invalid: {:#?}", args[1]);
            std::process::exit(-1);
        }
    };
    println!("[+] Targeting PID:\t\t{}", pid);

    if args.len() == 3 {
        tid = match args[2].parse() {
            Ok(id) => id,
            Err(_e) => {
                eprintln!("[-] Provided TID is invalid: {:#?}", args[1]);
                std::process::exit(-1);
            }
        };
    }

    if pid == std::process::id() {
        eprintln!("[-] DO NOT RUN THIS PROGRAM AGAINST THE CURRENT PROCESS");
        std::process::exit(-1);
    }
    stacktrace(pid, tid);
}
