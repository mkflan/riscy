# riscy
A simple RISC-V operating system written in Rust.

## Roadmap

- Runtime
  - [x] Link kernel into higher half
  - [ ] Write bootstrap assembly code
- Memory Management
  - [ ] PMM 
  - [ ] VMM
  - [ ] Heap
- Kernel Sanity
  - [ ] Tests
  - [ ] Debugging
  - [ ] Logger
- Drivers
  - [x] UART
  - [ ] Keyboard
- Scheduling
  - [ ] Basic task scheduler
  - [ ] Program execution in user mode
  - [ ] SMP
- Interface
  - [ ] File System
  - [ ] System calls
  - [ ] Shell
- Other
  - [ ] Interrupt and trap handling
  - [ ] Move away from Makefile to Rust-based build system
