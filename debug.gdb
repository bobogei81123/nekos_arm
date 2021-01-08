set disassemble-next-line on
set confirm off
add-symbol-file target/aarch64-unknown-none/debug/nekos_arm
target remote tcp::1234
set arch aarch64
