use core::arch::asm;

fn hf_printchar_impl(buffer: [u8; 1], r8: *mut u8) -> *mut u8 {
    unsafe {
        asm!(
            "mov rax, 1",          // syscall: sys_write
            "mov rdi, 1",          // file descriptor: stdout
            "mov rsi, {0}",        // buffer
            "mov rdx, 1",          // buffer length
            "syscall",             // invoke syscall
            in(reg) buffer.as_ptr(),
            out("rax") _,
        );
    }

    r8
}

fn hf_printflush_impl(buffer: [u8; 0], r8: *mut u8) -> *mut u8 {
    unsafe {
        asm!(
            "mov rax, 1",          // syscall: sys_write
            "mov rdi, 1",          // file descriptor: stdout
            "mov rsi, {0}",        // buffer
            "mov rdx, 1",          // buffer length
            "syscall",             // invoke syscall
            in(reg) [0xa].as_ptr(),
            out("rax") _,
        );
    }

    r8
}

#[no_mangle]
pub extern "C" fn hf_printchar(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    super::hf_func_wrapper(r8, stack_ptr, hf_printchar_impl);
}

#[no_mangle]
pub extern "C" fn hf_printflush(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    super::hf_func_wrapper(r8, stack_ptr, hf_printflush_impl);
}
