BUILD_DIR=./target/aarch64-unknown-none-softfloat/debug
PREFIX=aarch64-none-elf
cargo build
$PREFIX-objcopy -O binary $BUILD_DIR/nekos_arm $BUILD_DIR/nekos_arm.bin
mkimage -A arm64 -C none -O linux -T kernel -d $BUILD_DIR/nekos_arm.bin -a 0x40080000 -e 0x40080040 $BUILD_DIR/nekos_arm.uimg
