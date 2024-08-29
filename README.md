# riscy
A simple RISC-V operating system written in Rust.

## Roadmap

- Runtime
  - [x] Link kernel into higher half
  - [ ] Write bootstrap assembly code
- Memory Management
  - [ ] Physical Memory Manager 
  - [ ] Virtual Memory Manager via paging
  - [ ] Heap Allocator
- Kernel Sanity
  - [ ] Tests
  - [ ] Debugging
  - [ ] Logger
- Scheduling
  - [ ] Basic task scheduler that can handle processes and threads
  - [ ] Synchronization primitives
- Drivers
  - [x] UART
- Userspace
  - [ ] Loading and executing programs
  - [ ] System call interface
  - [ ] Shell interface
- Other
  - [ ] Interrupt and trap handling
  - [ ] IPC
  - [ ] Virtual File System
  - [ ] Move away from Makefile to Rust-based build system
