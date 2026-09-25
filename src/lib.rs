use citro2d_sys::*;
use citro3d_sys::*;

pub const TOP: u8 = 0;
pub const LEFT: u8 = 0;

pub const BOTTOM: u8 = 1;
pub const RIGHT: u8 = 1;

pub fn init() {
    unsafe {
        C3D_Init(C3D_DEFAULT_CMDBUF_SIZE as usize);
        C2D_Init(C2D_DEFAULT_MAX_OBJECTS as usize);
        C2D_Prepare();
    }
}

pub fn end() {
    unsafe {
        C2D_Fini();
        C3D_Fini();
    }
}

pub fn create_top_screen() -> *mut citro2d_sys::C3D_RenderTarget {
    unsafe { C2D_CreateScreenTarget(TOP, LEFT) }
}

pub fn draw_square(target_screen: *mut citro2d_sys::C3D_RenderTarget) {
    unsafe {
        let clear_color = C2D_Color32(0xFF, 0xD8, 0xB0, 0xFF);

        let clr_rec_1 = C2D_Color32(0x9A, 0x6C, 0xB9, 0xFF);
        let clr_rec_2 = C2D_Color32(0xFF, 0xFF, 0x2C, 0xFF);
        let clr_rec_3 = C2D_Color32(0xD8, 0xF6, 0x0F, 0xFF);
        let clr_rec_4 = C2D_Color32(0x40, 0xEA, 0x87, 0xFF);

        const SCREEN_WIDTH: f32 = 400.0;

        C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
        // C2D_TargetClear(target_screen, clear_color);
        C2D_SceneBegin(target_screen);

        C2D_DrawRectangle(
            SCREEN_WIDTH - 50.0,
            0.0,
            0.0,
            10.0,
            10.0,
            clr_rec_1,
            clr_rec_2,
            clr_rec_3,
            clr_rec_4,
        );
       
        C3D_FrameEnd(0);
    }
}
