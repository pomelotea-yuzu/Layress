// Layress 桌面工作台入口。窗口与多 WebView 编排在 add-tauri-page-host 任务 2.x 落地，
// 当前骨架只验证构建与启动链路，不提前宣称页面承载能力已实现。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    layress_desktop_lib::run()
}
