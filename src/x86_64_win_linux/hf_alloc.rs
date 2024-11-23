fn hf_alloc_impl(buffer: [u8; 4], r8: *mut u8) -> *mut u8 {
    let size = u32::from_le_bytes(buffer) as usize;

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
pub extern "C" fn hf_alloc() {
    super::hf_func_wrapper(hf_alloc_impl);
}
