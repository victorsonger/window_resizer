use core_foundation::base::TCFType;
use core_foundation::number::CFNumber;
use core_foundation::string::CFString;
use device_query::{DeviceQuery, DeviceState, Keycode};
use tao::platform::macos::WindowBuilderExtMacOS; // 用于处理 macOS 的窗口操作

fn main() {
    let device_state = DeviceState::new();

    println!("程序已启动，监听快捷键 Ctrl + Cmd + R...");

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        // 检查是否按下了 Ctrl + Cmd + R
        if keys.contains(&Keycode::LControl)
            && keys.contains(&Keycode::Meta)
            && keys.contains(&Keycode::R)
        // 这里想要检测到R键，需要把终端添加到系统设置输入监控的白名单，否则只能检测到shift、cmd这些键
        {
            println!("检测到快捷键 Ctrl + Cmd + R!");

            // 在这里触发窗口调整的功能
            adjust_active_window();
        }
    }
}

extern crate cocoa;
extern crate core_graphics;

use cocoa::appkit::NSWindow;
use cocoa::base::{id, nil};
use cocoa::foundation::NSRect;
use core_graphics::display::{
    kCGNullWindowID, kCGWindowListOptionOnScreenOnly, CFArray, CFDictionary, CGMainDisplayID,
    CGWindowListCopyWindowInfo,
};
use std::process;
use std::ptr;

fn adjust_active_window() {
    // 获取当前活动窗口信息
    let active_window = get_active_window_info();

    if let Some(window_info) = active_window {
        // 获取当前窗口的位置和大小
        let mut window_rect = window_info.bounds;
        let window_width = window_rect.size.width;
        let window_height = window_rect.size.height;

        // 计算正方形的边长
        let square_size = window_width.min(window_height);

        // 计算调整后的窗口位置（为了使窗口居中）
        window_rect.size.width = square_size;
        window_rect.size.height = square_size;

        // 将窗口居中在原位置
        window_rect.origin.x += (window_width - square_size) / 2.0;
        window_rect.origin.y += (window_height - square_size) / 2.0;

        // 调整窗口大小
        set_window_size(window_info.window_id, window_rect);
    }
}

fn get_active_window_info() -> Option<CFDictionary> {
    // 获取屏幕上的窗口列表
    let options = kCGWindowListOptionOnScreenOnly;
    let window_list = unsafe { CGWindowListCopyWindowInfo(options, kCGNullWindowID) };

    // 将窗口列表转换为 CFArray
    let array: CFArray = unsafe { CFArray::wrap_under_get_rule(window_list) };

    // 获取当前进程的 PID
    let current_pid = process::id() as i64;
    // 遍历窗口信息
    for i in 0..array.len() {
        // 将 CFArray 中的元素转换为 CFDictionary
        if let Some(window_info) = array.get(i).to::<CFDictionary>() {
            // 获取窗口的 PID (kCGWindowOwnerPID)
            if let Some(window_pid) = window_info
                .find(CFString::new("kCGWindowOwnerPID"))
                .and_then(|value| value.downcast::<CFNumber>())
                .and_then(|number| number.to_i64())
            {
                // 找到与当前进程 PID 相同的窗口（即当前活跃窗口）
                if window_pid == current_pid {
                    return Some(window_info);
                }
            }
        }
    }

    None
}

fn set_window_size(window_id: u32, new_rect: NSRect) {
    // 通过系统 API 调整窗口大小，使用 window_id 来定位窗口
    // Cocoa 和 CoreGraphics 的结合使用可以修改窗口大小和位置

    // 示例代码（未完整实现）：
    unsafe {
        let window: id = window_id as id;
        window.setFrame_display_(new_rect, true);
    }
}
