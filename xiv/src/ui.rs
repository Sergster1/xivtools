#[cfg(windows)]
use {winapi::shared::minwindef::UINT, winapi::um::winuser::PostMessageA};

use log;
use std::thread::sleep;
use std::time::Duration;

// Wait |s| seconds, fractions permitted.
pub fn wait(s: f32) {
    let ms = (s * 1000 as f32) as u64;
    sleep(Duration::from_millis(ms));
}

#[cfg(windows)]
mod constants {
    use winapi::um::winuser::*;
    pub const KEY_UP: i32 = VK_NUMPAD8;
    pub const KEY_DOWN: i32 = VK_NUMPAD2;
    pub const KEY_LEFT: i32 = VK_NUMPAD4;
    pub const KEY_RIGHT: i32 = VK_NUMPAD6;
    pub const KEY_CONFIRM: i32 = VK_NUMPAD0;
    pub const KEY_FORWARD: i32 = VK_NUMPAD9;
    pub const KEY_BACKWARD: i32 = VK_NUMPAD7;
    pub const KEY_CANCEL: i32 = VK_DECIMAL;
    pub const KEY_ENTER: i32 = VK_RETURN;
    pub const KEY_ESCAPE: i32 = VK_ESCAPE;
    pub const MSG_KEY_UP: u32 = WM_KEYUP;
    pub const MSG_KEY_DOWN: u32 = WM_KEYDOWN;
    pub const MSG_KEY_CHAR: u32 = WM_CHAR;
}

#[cfg(not(windows))]
mod constants {
    pub const KEY_UP: i32 = 0;
    pub const KEY_DOWN: i32 = 0;
    pub const KEY_LEFT: i32 = 0;
    pub const KEY_RIGHT: i32 = 0;
    pub const KEY_CONFIRM: i32 = 0;
    pub const KEY_FORWARD: i32 = 0;
    pub const KEY_BACKWARD: i32 = 0;
    pub const KEY_CANCEL: i32 = 0;
    pub const KEY_ENTER: i32 = 0;
    pub const KEY_ESCAPE: i32 = 0;
    pub const MSG_KEY_UP: u32 = 0;
    pub const MSG_KEY_DOWN: u32 = 0;
    pub const MSG_KEY_CHAR: u32 = 0;
}

pub fn cursor_down(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_DOWN);
}

pub fn cursor_up(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_UP);
}

pub fn cursor_left(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_LEFT);
}

pub fn cursor_right(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_RIGHT);
}

pub fn cursor_backward(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_BACKWARD)
}

pub fn cursor_forward(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_FORWARD);
}

pub fn press_confirm(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_CONFIRM);
}

pub fn press_cancel(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_CANCEL);
}

pub fn press_enter(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_ENTER);
    wait(1.0);
}

pub fn press_escape(xiv_handle: &super::XivHandle) {
    send_key(xiv_handle, constants::KEY_ESCAPE);
}

pub fn send_string(xiv_handle: &super::XivHandle, s: &str) {
    log::trace!("sending string: '{}'\n", s);
    for c in s.chars() {
        send_char(xiv_handle, c);
    }
}

pub fn open_craft_window(xiv_handle: &super::XivHandle) {
    // TODO: This should be configurable
    send_key(xiv_handle, 'N' as i32);
    wait(1.0);
}

pub fn clear_window(xiv_handle: &super::XivHandle) {
    println!("clearing window...");
    // Hitting escape closes one window each.
    for _ in 0..2 {
        press_escape(xiv_handle);
    }

    // Cancelling twice will close the System menu if it is open.
    press_cancel(xiv_handle);
    press_cancel(xiv_handle);
    wait(1.0);
    // Clear anything in the input box.
    press_enter(xiv_handle);
    press_enter(xiv_handle);
    wait(1.0);
}

pub fn send_char(xiv_handle: &super::XivHandle, c: char) {
    send_msg(&xiv_handle, constants::MSG_KEY_CHAR, c as i32);
    wait(0.05);
}

fn send_key(xiv_handle: &super::XivHandle, c: i32) {
    send_msg(&xiv_handle, constants::MSG_KEY_DOWN, c);
    send_msg(&xiv_handle, constants::MSG_KEY_UP, c);
    wait(0.150)
}

// Send a character/key to the XIV window
fn send_msg(_xiv_handle: &super::XivHandle, _msg: u32, _key: i32) {
    #[cfg(windows)]
    unsafe {
        PostMessageA(_xiv_handle.hwnd, _msg as UINT, _key as usize, 0);
    }
}
