use crate::alloc::{GlobalAlloc, Layout, System};
use crate::cell::UnsafeCell;
use risc0_zkvm_platform::{memory, WORD_SIZE};

struct BumpPointerAlloc {
    head: UnsafeCell<usize>,
    end: usize,
}
// SAFETY: single threaded environment
unsafe impl Sync for BumpPointerAlloc {}

static mut HEAP: BumpPointerAlloc =
    BumpPointerAlloc { head: UnsafeCell::new(memory::HEAP.start()), end: memory::HEAP.end() };

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let head = HEAP.head.get();

        // move start up to the next alignment boundary
        let alloc_start = (*head + WORD_SIZE) & !(WORD_SIZE - 1);
        let alloc_end = alloc_start.checked_add(layout.size()).unwrap();
        if alloc_end > HEAP.end {
            panic!("out of heap");
        } else {
            *head = alloc_end;
            alloc_start as *mut u8
        }
    }

    #[inline]
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // this allocator never deallocates memory
    }
}
