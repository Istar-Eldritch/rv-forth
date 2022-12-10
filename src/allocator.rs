// in src/allocator/bump.rs
//

use core::alloc::{GlobalAlloc, Layout};

#[global_allocator]
static ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new(50000, 98000));

pub struct Mutex<A> {
    inner: spin::Mutex<A>,
}

impl<A> Mutex<A> {
    pub const fn new(inner: A) -> Self {
        Mutex {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
            allocations: 0,
        }
    }
}

unsafe impl GlobalAlloc for Mutex<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO alignment and bounds check
        let mut alloc = self.lock();
        alloc.next = alloc.next + layout.size();
        alloc.allocations += 1;
        alloc.next as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut alloc = self.lock();
        alloc.allocations -= 1;

        if alloc.allocations == 0 {
            alloc.next = alloc.heap_start;
        }
    }
}
