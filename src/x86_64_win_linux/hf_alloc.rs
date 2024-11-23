fn hf_alloc_impl(buffer: [u8; 4], r8: *mut u8) -> *mut u8 {
    let size = u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]) as usize;

    unsafe {
        let new_size = super::MEM_SIZE + size;
        let old_offset = r8 as u64 as i64 - super::MEM_PTR as u64 as i64;
        super::MEM_PTR = alloc::alloc::realloc(
            super::MEM_PTR,
            alloc::alloc::Layout::from_size_align_unchecked(new_size, 1),
            new_size,
        );
        let new_addr = super::MEM_PTR as u64 as i64 + old_offset;
        new_addr as *mut u8
    }
}

#[no_mangle]
pub extern "C" fn hf_alloc(r8: *mut *mut u8, stack_ptr: *mut *mut u8) {
    super::hf_func_wrapper(r8, stack_ptr, hf_alloc_impl)
}
