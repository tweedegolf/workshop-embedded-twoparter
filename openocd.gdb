# Not used by the VS Code Debugger, only `cargo run`
target remote :3333
set print asm-demangle on
set print pretty on
load
monitor rtt server start 8765 0
monitor rtt setup 0x2003fbc0 0x30 "SEGGER RTT"
monitor rtt start
break main
continue
