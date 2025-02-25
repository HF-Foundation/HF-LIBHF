#[cfg(target_os = "linux")]
mod linux_allocator;

mod hf_alloc;
mod hf_socket;
pub use hf_alloc::*;

mod hf_print;
pub use hf_print::*;

use core::arch::asm;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(target_os = "linux")]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "int3",
            "mov rax, 60",  // syscall number for exit
            "mov rdi, 134", // exit code for SIGABRT
            "syscall",
            options(noreturn)
        )
    }
}

#[cfg(target_os = "windows")]
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!(
            "int3",
            "mov rax, 0x3C",  // syscall number for exit
            "mov rdi, 0x1F4", // exit code for SIGABRT
            "syscall",
            options(noreturn)
        )
    }
}

pub fn stack_push(stack_ptr: &mut *mut u8, value: u8) {
    unsafe {
        let start_ptr = STACK.as_ptr() as u64;
        let stack_index = (*stack_ptr as u64) - start_ptr;
        STACK[stack_index as usize] = value;
        *stack_ptr = (*stack_ptr).offset(1);
    }
}

pub fn stack_pop(stack_ptr: &mut *mut u8) -> u8 {
    unsafe {
        let start_ptr = STACK.as_ptr() as u64;
        let stack_index = (*stack_ptr as u64) - start_ptr - 1;
        *stack_ptr = (*stack_ptr).offset(-1);
        STACK[stack_index as usize]
    }
}

/// A beautiful, magical function that saves the r8 register, pops
/// N values from the stack into a buffer, calls the provided function
/// with the buffer, and restores the r8 register.
fn hf_func_wrapper<const N: usize, F: FnOnce([u8; N], *mut u8, &mut *mut u8) -> *mut u8>(
    r8: *mut *mut u8,
    stack_ptr: *mut *mut u8,
    f: F,
) {
    let mut buffer = [0u8; N];
    unsafe {
        let start_ptr = STACK.as_ptr() as u64;
        let stack_index = (*stack_ptr as u64) - start_ptr;
        buffer.copy_from_slice(&STACK[(stack_index - N as u64) as usize..stack_index as usize]);
        *stack_ptr = *stack_ptr.offset(-(N as isize));
        *r8 = f(buffer, *r8, &mut *stack_ptr);
    }
}

const MEM_SIZE_DEFAULT: usize = 50 * 1024 * 1024;

static mut MEM_PTR: *mut u8 = core::ptr::null_mut();
static mut MEM_SIZE: usize = MEM_SIZE_DEFAULT;

static mut STACK: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub extern "C" fn hf_start(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    unsafe {
        *stack_ptr = STACK.as_mut_ptr();
    }

    unsafe {
        MEM_PTR = alloc::alloc::alloc(alloc::alloc::Layout::from_size_align_unchecked(
            MEM_SIZE_DEFAULT,
            1,
        ));

        // TODO: We need to return r8 and stack_ptr somehow

        *r8 = MEM_PTR;
    }
}

#[no_mangle]
pub extern "C" fn hf_exit(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    unsafe {
        alloc::alloc::dealloc(
            MEM_PTR,
            alloc::alloc::Layout::from_size_align_unchecked(MEM_SIZE, 1),
        );
    }
    hf_func_wrapper(r8, stack_ptr, hf_exit_impl);
}

#[cfg(target_os = "linux")]
fn hf_exit_impl(buffer: [u8; 1], _r8: *mut u8, _: &mut *mut u8) -> *mut u8 {
    unsafe {
        asm!(
            "mov rdi, {}",
            "mov rax, 60", // syscall number for exit
            "syscall",
            in(reg) buffer[0] as i64,
            options(noreturn),
        )
    }
}

#[cfg(target_os = "windows")]
fn hf_exit_impl(buffer: [u8; 1], _r8: *mut u8, _: &mut *mut u8) -> *mut u8 {
    unsafe {
        asm!(
            "mov rdi, {}",
            "mov rax, 0x3C", // syscall number for exit
            "syscall",
            in(reg) buffer[0] as i64,
            options(noreturn),
        )
    }
}
