KERNEL_ELF = nekos_arm
KERNEL_BIN = $(KERNEL_ELF).bin

TARGET = aarch64-unknown-none-softfloat
TARGET_DEBUG = target/$(TARGET)/debug
TARGET_RELEASE = target/$(TARGET)/release

PREFIX = aarch64-none-elf
GDB = $(PREFIX)-gdb
OBJCOPY = $(PREFIX)-objcopy

QEMU = qemu-system-aarch64
QEMU_ARGS = -machine virt -m 1G -cpu cortex-a53 -nographic

RUSTFLAGS = 
#RUSTFLAGS = -C llvm-args=-global-isel=false

PINEPHONE_RUST_FEATURES = --no-default-features --features=bsp_pinephone

.PHONY: debug debug-bin gdb qemu qemu-gdb doc clean

check:
	RUSTFLAGS="$(RUSTFLAGS)" cargo check

debug:
	RUSTFLAGS="$(RUSTFLAGS)" cargo build

debug-bin: debug
	$(OBJCOPY) -O binary $(TARGET_DEBUG)/$(KERNEL_ELF) $(TARGET_DEBUG)/$(KERNEL_BIN)

test:
	RUSTFLAGS="$(RUSTFLAGS)" cargo test

gdb: debug
	$(GDB) $(TARGET_DEBUG)/$(KERNEL_ELF) -x debug.gdb

qemu: debug-bin
	$(QEMU) $(QEMU_ARGS) -kernel $(TARGET_DEBUG)/$(KERNEL_BIN)

qemu-gdb: debug-bin
	$(QEMU) $(QEMU_ARGS) -S -s -kernel $(TARGET_DEBUG)/$(KERNEL_BIN)

check-pp:
	cargo check $(PINEPHONE_RUST_FEATURES)

debug-pp:
	cargo build $(PINEPHONE_RUST_FEATURES)

debug-pp-bin: debug-pp
	$(OBJCOPY) -O binary $(TARGET_DEBUG)/$(KERNEL_ELF) $(TARGET_DEBUG)/$(KERNEL_BIN)

pinephone: debug-pp-bin
	mkimage -A arm64 -C none -O u-boot -T kernel -d $(TARGET_DEBUG)/$(KERNEL_BIN) -a 0x40080000 -e 0x40080000 pinephone.uimg

doc:
	cargo doc --document-private-items --open

clean:
	cargo clean
