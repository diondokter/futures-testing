#![no_std]
#![no_main]

async fn foo(val: i32) -> i32 {
    val * 2
}

fn bar() -> impl Future<Output = i32> {
    async { foo(5).await }
}

#[cortex_m_rt::entry]
fn main() -> ! {
    core::hint::black_box(block_on(foo(5)));
    core::hint::black_box(block_on(bar()));

    loop {}
}

use core::future::Future;
use core::panic::PanicInfo;
use core::pin::Pin;
use core::ptr;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

static VTABLE: RawWakerVTable = RawWakerVTable::new(
    |_| RawWaker::new(ptr::null(), &VTABLE),
    |_| {},
    |_| {},
    |_| {},
);

#[inline(never)]
fn block_on<F: Future>(mut fut: F) -> F::Output {
    // safety: we don't move the future after this line.
    let mut fut = unsafe { Pin::new_unchecked(&mut fut) };

    let raw_waker = RawWaker::new(ptr::null(), &VTABLE);
    let waker = unsafe { Waker::from_raw(raw_waker) };
    let mut cx = Context::from_waker(&waker);
    loop {
        if let Poll::Ready(res) = fut.as_mut().poll(&mut cx) {
            return res;
        }
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}
