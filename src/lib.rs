#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SMILEY: &[u8; 8] = include_bytes!(concat!(env!("OUT_DIR"), "/smiley.raw"));

#[no_mangle]
fn update() {
    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);

    let gamepad = unsafe { *GAMEPAD1 };
    if gamepad & BUTTON_1 != 0 {
        unsafe { *DRAW_COLORS = 4 }
    }

    blit(SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    text("Press X to blink", 16, 90);
}
