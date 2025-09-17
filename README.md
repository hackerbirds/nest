# nest

## build

`cargo build` should do it though you probably need rust nightly

## run

`qemu-system-aarch64 -s -S -machine virt -cpu cortex-a57 -nographic -kernel target/aarch64-sdm670/debug/nest`

Control-A + X to exit

### debugging

`llvm target/aarch64-sdm670/debug/nest`
in llvm:
`gdb-remote localhost:1234`
`c` to continue execution
`dis` to see assembly
`si` to continue instructions one by one
`register read <REGISTER_NAME>`