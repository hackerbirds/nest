# nest

## build

`cargo build` should do it though you probably need rust nightly

## run

`qemu-system-aarch64 -machine virt -cpu cortex-a57 -nographic -kernel target/aarch64-sdm670/debug/nest`

Control-A + X to exit