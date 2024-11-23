use core::arch::asm;

use alloc::alloc::Layout;

#[global_allocator]
static ALLOCATOR: HfAllocator = HfAllocator;

struct HfAllocator;

unsafe impl alloc::alloc::GlobalAlloc for HfAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let addr: *mut u8;
        let res: isize = 0;

        asm!(
            "syscall",
            in("rax") 9, // syscall number for mmap
            in("rdi") 0, // addr
            in("rsi") layout.size(), // length
            in("rdx") 3, // prot (PROT_READ | PROT_WRITE)
            in("r10") 34, // flags (MAP_PRIVATE | MAP_ANONYMOUS)
            in("r8") -1, // fd
            in("r9") 0, // offset
            lateout("rax") addr,
            options(nostack)
        );

        if res < 0 {
            panic!("Failed to allocate!");
        } else {
            addr as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let res: isize;

        unsafe {
            asm!(
                "syscall",
                in("rax") 11, // syscall number for munmap
                in("rdi") ptr, // addr
                in("rsi") layout.size(), // length
                lateout("rax") res,
                options(nostack)
            );
        }

        if res < 0 {
            // Handle error if needed
            panic!("Failed to deallocate!");
        }
    }
}
