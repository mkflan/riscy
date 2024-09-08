BUILD_MODE := debug
override RV64_TOOLCHAIN := riscv64-elf
override KERNEL_PATH := target/riscv64imac-unknown-none-elf/$(BUILD_MODE)/riscy

CARGO_ARGS = 

ifeq ($(BUILD_MODE),release)
	CARGO_ARGS += --$(BUILD_MODE)
endif

QEMU_OPTS = -cpu rv64 -machine virt -m 128M -no-shutdown -serial mon:stdio -kernel $(KERNEL_PATH)

qemu:
	cargo run $(CARGO_ARGS) -- $(QEMU_OPTS)

qemudbg: 
	cargo run $(CARGO_ARGS) -- $(QEMU_OPTS) -S -s

gdb: 
	gdb-multiarch $(KERNEL_PATH) -ex "target remote localhost:1234" -ex "set architecture riscv:rv64"
	
kernel.disasm:
	cargo build $(CARGO_ARGS) 
	$(RV64_TOOLCHAIN)-objdump -Cd $(KERNEL_PATH) > $@

.PHONY: clean
clean:
	cargo clean
	rm -f kernel.disasm
