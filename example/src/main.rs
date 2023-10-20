#![no_std]
#![no_main]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

struct AContainer {
    a: i32,
}

impl AContainer {
    pub fn do_math_on(&self, b: i32) -> i32 {
        // put breakpoint here
        self.a * 2 + b + 1
    }
}

#[no_mangle]
extern "C" fn math(a: i32, b: i32) -> i32 {
    let container = AContainer { a };
    container.do_math_on(b)
}
