BUILD_DIR=./target/aarch64-unknown-none/release
PREFIX=aarch64-none-elf
cargo build --release
$PREFIX-objcopy -O binary $BUILD_DIR/nekos_arm $BUILD_DIR/nekos_arm.bin
mkimage -A arm64 -C none -O u-boot -T kernel -d $BUILD_DIR/nekos_arm.bin -a 0x40080000 -e 0x40080000 $BUILD_DIR/nekos_arm.uimg
