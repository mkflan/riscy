// #[naked]
// #[repr(align(4))]
// pub unsafe extern "C" fn pretrap() -> ! {
//     core::arch::asm!();
// }

#[no_mangle]
extern "C" fn trap_handler() {
    todo!();
}
