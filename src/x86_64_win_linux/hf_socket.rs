use core::arch::asm;

use crate::stack_push;

fn hf_socket_impl(buffer: [u8; 1], r8: *mut u8, stack_ptr: &mut *mut u8) -> *mut u8 {
    unsafe {
        let mut fd: i64;
        asm!(
            "mov rax, 41",          // syscall: sys_socket
            "mov rdi, 2",          // family
            "mov rsi, {0}",        // type
            "mov rdx, 0",          // protocol
            "syscall",             // invoke syscall
            in(reg) buffer[0] as i64,
            out("rax") fd,
        );
        stack_push(stack_ptr, fd as u8);
    }
    r8
}

#[no_mangle]
pub extern "C" fn hf_socket(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    super::hf_func_wrapper(r8, stack_ptr, hf_socket_impl);
}
