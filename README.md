# StackTracer 

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/whokilleddb)

The goal of this project is find valid stack frames which you can replicate in your payloads. In C2s like BRC4, you can specify a stack frame which looks legit - and this can help in increasing the evasiveness of your shellcode. Using this tool you can examine legit processes for their stack frames and copy them over.

## Building 

Building this tool is as easy as rust makes it:

```
$ cargo build --release
```

Note that you might need to install the `x86_64-pc-windows-gnu` toolchain

## Usage:

To view a detailed usage guide, you can pass the `--help` flag:

```
Z:\> stackfinder.exe --help
  _____ _             _ _______
 / ____| |           | |__   __|
| (___ | |_ __ _  ___| | _| |_ __ __ _  ___ ___ _ __
 \___ \| __/ _` |/ __| |/ / | '__/ _` |/ __/ _ \ '__|
 ____) | || (_| | (__|   <| | | | (_| | (_|  __/ |
|_____/ \__\__,_|\___|_|\_\_|_|  \__,_|\___\___|_|
                                      DB @whokilleddb

A rust program to print the stack trace of a given thread

Usage: stackfinder.exe [OPTIONS] --pid <pid>

Options:
      --pid <pid>    Process ID
      --tid <tid>    Thread ID (defaults to 0) [default: 0]
      --hide-banner  Hide the banner
  -h, --help         Print help
  -V, --version      Print version
```

The only required option is the `--pid` flag. To enumerate a process without targetting a particular thread, you go like:

```
Z:\> stackfinder.exe --pid 1234
```

To enumerate a specific thread, you can specify the ThreadID using the `--tid` flag:

```
Z:\>.\stackfinder.exe --pid 3688 --tid 7336
  _____ _             _ _______
 / ____| |           | |__   __|
| (___ | |_ __ _  ___| | _| |_ __ __ _  ___ ___ _ __
 \___ \| __/ _` |/ __| |/ / | '__/ _` |/ __/ _ \ '__|
 ____) | || (_| | (__|   <| | | | (_| | (_|  __/ |
|_____/ \__\__,_|\___|_|\_\_|_|  \__,_|\___\___|_|
                                      DB @whokilleddb

PID     | TID   | STACK FRAME
3688    | 7336  | win32u.dll!NtUserWaitMessage+0x14,USER32.DLL!IsDialogMessageA+0x3ba,USER32.DLL!Ordinal2635+0x267,USER32.DLL!SoftModalMessageBox+0x5b6,USER32.DLL!MessageBoxIndirectA+0x562,USER32.DLL!MessageBoxTimeoutW+0x18f,USER32.DLL!MessageBoxTimeoutA+0x100,USER32.DLL!MessageBoxA+0x45,0x25c70680050,KERNEL32.DLL+0x0
```

_Hope this helps!_
