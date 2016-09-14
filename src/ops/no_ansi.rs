#[cfg(target_os = "windows")]
use kernel32::{GetConsoleScreenBufferInfoEx, FillConsoleOutputAttribute, GetStdHandle};
#[cfg(target_os = "windows")]
use winapi::{CONSOLE_SCREEN_BUFFER_INFOEX, STD_OUTPUT_HANDLE, SMALL_RECT, COORD};
use self::super::super::util::{closest_colour, mul_str};
use image::{self, GenericImage, DynamicImage, Pixel};
use std::mem;


/// Display the specified image in the default console using WinAPI.
#[cfg(target_os = "windows")]
pub fn write_no_ansi(img: &DynamicImage) {
    let (width, height) = img.dimensions();
    print!("{}", mul_str("\n", height as usize));

    let console_h = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    let mut console_info = CONSOLE_SCREEN_BUFFER_INFOEX {
        cbSize: mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32,
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: SMALL_RECT {
            Left: 0,
            Top: 0,
            Right: 0,
            Bottom: 0,
        },
        dwMaximumWindowSize: COORD { X: 0, Y: 0 },
        wPopupAttributes: 0,
        bFullscreenSupported: 0,
        ColorTable: [0; 16],
    };
    unsafe { GetConsoleScreenBufferInfoEx(console_h, &mut console_info) };
    let colors =
        console_info.ColorTable.iter().map(|cr| image::Rgb([(cr & 0xFF) as u8, ((cr & 0xFF00) >> 8) as u8, ((cr & 0xFF0000) >> 16) as u8])).collect::<Vec<_>>();

    for y in 0..height {
        for x in 0..width {
            let closest_clr = closest_colour(img.get_pixel(x, y).to_rgb(), &colors) as u16;
            unsafe {
                FillConsoleOutputAttribute(console_h,
                                           (console_info.wAttributes & 0xFF0F) | (closest_clr << 4),
                                           1,
                                           COORD {
                                               X: x as i16,
                                               Y: console_info.dwCursorPosition.Y - (height as i16 - y as i16),
                                           },
                                           &mut 0);
            }
        }
    }
}

/// Display the specified image in the default console using WinAPI.
///
/// Or, actually, don't. This is Linux, after all...
#[cfg(not(target_os = "windows"))]
pub fn write_no_ansi(_: &DynamicImage) {}
