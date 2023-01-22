use core::{alloc::GlobalAlloc, cell::Cell};

pub struct AVRAlloc;

impl AVRAlloc {
    pub const fn new() -> Self {
        AVRAlloc
    }
}

//TODO: Technically, 0x2800 is the start of the SRAM, but that is also used for static variables.
//Give 256B of buffer for statics, just in case. Figure out a better way to do this to not waste
//memory
static mut HEAP_CUR: *mut u8 = 0x2900 as *mut _; //6kb of internal SRAM, shared with stack
const HEAP_END: *mut u8 = 0x2e00 as *mut _; //END - CUR = heap size left

unsafe impl GlobalAlloc for AVRAlloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let a = layout.align();
        let s = layout.size();
        let h = HEAP_CUR;
        //TODO alignment?
        let mut pre_amount = h as usize % layout.align();
        //ufmt::uwrite!(
        //crate::STDOUT,
        //"HEAP_CUR=0x{:x}, pre_amt={}\r\n",
        //(h as u16),
        //pre_amount
        //)
        //.unwrap();
        if pre_amount > 0 {
            pre_amount = layout.align() - pre_amount;
        }

        let ret_ptr = h.offset(pre_amount as isize);
        let new_heap = ret_ptr.offset(layout.size() as isize);
        if new_heap > HEAP_END {
            panic!("Heap End");
        }

        HEAP_CUR = new_heap;

        //ufmt::uwrite!(
        //crate::STDOUT,
        //"alloc a={} s={} ptr=0x{:x} newend=0x{:x} wtf=0x{:x}\r\n",
        //a,
        //layout.size(),
        //(ret_ptr as u16),
        //(new_heap as u16),
        //(HEAP_CUR as u16)
        //)
        //.unwrap();

        ret_ptr
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        //noop xd
    }
}
