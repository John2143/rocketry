use core::alloc::GlobalAlloc;

pub struct AVRAlloc;

impl AVRAlloc {
    pub const fn new() -> Self {
        AVRAlloc
    }
}

unsafe impl GlobalAlloc for AVRAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}
