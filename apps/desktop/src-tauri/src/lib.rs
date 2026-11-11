// 应用库入口：供桌面二进制与将来可能的集成测试共用。
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("Layress 桌面应用启动失败");
}
