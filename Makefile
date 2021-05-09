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

.PHONY: debug debug-bin gdb qemu qemu-gdb doc clean

debug:
	cargo build

debug-bin: debug
	$(OBJCOPY) -O binary $(TARGET_DEBUG)/$(KERNEL_ELF) $(TARGET_DEBUG)/$(KERNEL_BIN)

gdb: debug
	$(GDB) $(TARGET_DEBUG)/$(KERNEL_ELF) -x debug.gdb

qemu: debug-bin
	$(QEMU) $(QEMU_ARGS) -kernel $(TARGET_DEBUG)/$(KERNEL_BIN)

qemu-gdb: debug-bin
	$(QEMU) $(QEMU_ARGS) -S -s -kernel $(TARGET_DEBUG)/$(KERNEL_BIN)

doc:
	cargo doc --document-private-items --open

clean:
	cargo clean
