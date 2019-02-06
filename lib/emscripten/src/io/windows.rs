use wasmer_runtime_core::vm::Ctx;
use libc::{c_char, c_int};

#[link(name = "c")]
extern "C" {
    #[link_name = "printf"]
    pub fn _printf(s: *const c_char, ...) -> c_int;
}

/// putchar
pub fn putchar(chr: i32, ctx: &mut Ctx) {
    unsafe { libc::putchar(chr) };
}

/// printf
pub fn printf(memory_offset: i32, extra: i32, ctx: &mut Ctx) -> i32 {
    debug!("emscripten::printf {}, {}", memory_offset, extra);
    unsafe {
        let addr = emscripten_memory_pointer!(ctx.memory(0), memory_offset) as _;
        _printf(addr, extra)
    }
}
