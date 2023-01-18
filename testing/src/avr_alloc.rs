use core::{alloc::GlobalAlloc, cell::Cell};

pub struct AVRAlloc;

impl AVRAlloc {
    pub const fn new() -> Self {
        AVRAlloc
    }
}

static mut HEAP_CUR: Cell<*mut u8> = Cell::new(0x2800 as *mut _); //6kb of internal SRAM, shared with stack
const HEAP_END: *mut u8 = 0x2a00 as *mut _; //200B of heap

unsafe impl GlobalAlloc for AVRAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let h = HEAP_CUR.get();
        //TODO alignment?
        let mut pre_amount = h as usize % layout.align();
        if pre_amount > 0 {
            pre_amount = layout.align() - pre_amount;
        }

        let ret_ptr = h.offset(pre_amount as isize);
        let new_heap = ret_ptr.offset(layout.size() as isize);
        if new_heap > HEAP_END {
            panic!("Heap End");
        }
        HEAP_CUR.set(new_heap);

        ret_ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        //noop xd
    }
}
