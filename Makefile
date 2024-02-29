override RV64_TOOLCHAIN := riscv64-unknown-elf
override TARGET := riscv64imac-unknown-none-elf
BUILD_MODE := debug

kernel.elf:
ifeq ($(BUILD_MODE), release)
	cargo build --$(BUILD_MODE)
else
	cargo build
endif
	mkdir build
	cp target/$(TARGET)/$(BUILD_MODE)/riscy build/$@

qemu: kernel.elf
	qemu-system-riscv64 -machine virt -cpu rv64 -bios none -m 128M -kernel build/$<

kernel.disasm: kernel.elf
	$(RV64_TOOLCHAIN)-objdump -Cd build/$< > build/$@

clean:
	cargo clean
	rm -rf build
