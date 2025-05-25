#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;

const SMILEY: &[u8; 8] = include_bytes!(concat!(env!("OUT_DIR"), "/smiley.raw"));
const ALL_COL_TEST: &[u8; 16] = include_bytes!(concat!(env!("OUT_DIR"), "/allcoltest.raw"));

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

    unsafe { *DRAW_COLORS = 0x4321 }
    blit(ALL_COL_TEST, 76 + 8, 76, 8, 8, BLIT_2BPP);
}
