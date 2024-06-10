BUILD_MODE := debug
override RV64_TOOLCHAIN := riscv64-unknown-elf
override TARGET := riscv64imac-unknown-none-elf
override BIN_DIR := target/$(TARGET)/$(BUILD_MODE)
override KERNEL_PATH := $(BIN_DIR)/riscy

CARGO_ARGS = 

ifeq ($(BUILD_MODE),release)
	CARGO_ARGS += --$(BUILD_MODE)
endif

kernel.elf:
	cargo rustc $(CARGO_ARGS)

QEMU_OPTS = -cpu rv64 -machine virt -m 128M -nographic -serial mon:stdio -kernel $(KERNEL_PATH)

qemu: kernel.elf
	qemu-system-riscv64 $(QEMU_OPTS)

qemudbg: kernel.elf
	qemu-system-riscv64 $(QEMU_OPTS) -S -s

lldb: 
	rust-lldb --arch riscv64 $(KERNEL_PATH)
	
kernel.disasm: kernel.elf
	$(RV64_TOOLCHAIN)-objdump -Cd $(KERNEL_PATH) > $@

.PHONY: clean
clean:
	cargo clean
	rm -f kernel.disasm
