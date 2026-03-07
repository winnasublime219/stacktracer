use std;
use std::mem;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Thread32First, Thread32Next,
    THREADENTRY32, TH32CS_SNAPTHREAD,
};
use winapi::um::handleapi::CloseHandle;
use winapi::shared::minwindef::FALSE;

pub fn collect_threads(pid: u32, tid: u32) -> Vec<u32> {
    let mut threads = Vec::new();

    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);

        if snapshot.is_null() {
            return threads;
        }

        let mut entry: THREADENTRY32 = mem::zeroed();
        entry.dwSize = mem::size_of::<THREADENTRY32>() as u32;

        if Thread32First(snapshot, &mut entry) == FALSE {
            CloseHandle(snapshot);
            return threads;
        }

        loop {
            if entry.th32OwnerProcessID == pid {
                if tid == 0 {
                    threads.push(entry.th32ThreadID);
                }
                else {
                    if tid == entry.th32ThreadID {
                        threads.push(tid);
                        CloseHandle(snapshot);
                        return threads;
                    }
                }
            }

            if Thread32Next(snapshot, &mut entry) == FALSE {
                break;
            }
        }

        CloseHandle(snapshot);
    }

    if tid != 0 {
        eprintln!("[-] Target thread not found!");
        std::process::exit(-1);
    }
    threads
}
