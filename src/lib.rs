use std::ptr::{null, null_mut};

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

pub fn draw_square(target_screen: *mut citro2d_sys::C3D_RenderTarget, x: f32, y: f32) {
    unsafe {
        let clear_color = C2D_Color32(0xFF, 0xD8, 0xB0, 0xFF);

        let clr_rec_1 = C2D_Color32(0x9A, 0x6C, 0xB9, 0xFF);
        let clr_rec_2 = C2D_Color32(0xFF, 0xFF, 0x2C, 0xFF);
        let clr_rec_3 = C2D_Color32(0xD8, 0xF6, 0x0F, 0xFF);
        let clr_rec_4 = C2D_Color32(0x40, 0xEA, 0x87, 0xFF);

        C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
        C2D_TargetClear(target_screen, clear_color);
        C2D_SceneBegin(target_screen);

        C2D_DrawRectangle(
            x, y, 0.0, 50.0, 50.0, clr_rec_1, clr_rec_2, clr_rec_3, clr_rec_4,
        );

        C3D_FrameEnd(0);
    }
}

pub fn create_sprite_sheet(filename: &str) -> Option<C2D_SpriteSheet> {
    unsafe {
        let sprite_sheet = C2D_SpriteSheetLoad(filename.as_ptr());
        if sprite_sheet.is_null() {
            None
        } else {
            Some(sprite_sheet)
        }
    }
}

pub fn create_sprite_from_sheet(sheet: C2D_SpriteSheet, index: usize) -> C2D_Sprite {
    let mut sprite = default_sprite();

    unsafe {
        C2D_SpriteFromSheet(&mut sprite, sheet, index);
    }

    sprite
}

fn default_sprite() -> C2D_Sprite {
    C2D_Sprite {
        image: C2D_Image {
            tex: null_mut(),
            subtex: null(),
        },
        params: C2D_DrawParams {
            pos: C2D_DrawParams__bindgen_ty_1 { x: 0.0, y: 0.0, w: 0.0, h: 0.0 },
            center: C2D_DrawParams__bindgen_ty_2 { x: 0.0, y: 0.0 },
            depth: 0.0,
            angle: 0.0,
        },
    }
}

pub fn draw_sprite(target_screen: *mut citro2d_sys::C3D_RenderTarget, sprite: &mut C2D_Sprite, x: f32, y: f32) {

    unsafe {
        let clear_color = C2D_Color32(0xFF, 0xD8, 0xB0, 0xFF);

        C2D_SpriteSetPos(sprite, x, y);
        C3D_FrameBegin(C3D_FRAME_SYNCDRAW);
        C2D_TargetClear(target_screen, clear_color);
        C2D_SceneBegin(target_screen);

        C2D_DrawSprite(sprite);

        C3D_FrameEnd(0);
    }
}
