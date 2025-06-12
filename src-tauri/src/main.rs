#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::os::windows::process::CommandExt;
use std::{process::Command, path::PathBuf};
use tauri::utils::platform::current_exe;
use std::fs;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            let exe_path = current_exe().expect("Không thể lấy đường dẫn thực thi app");
            let base_path = exe_path
                .parent().expect("Không có thư mục cha")
                .to_path_buf();

            // --- Khởi động MongoDB local ---
            let mongo_path: PathBuf = base_path
                .join("resources")
                .join("mongodb")
                .join("bin")
                .join("mongod.exe");

            let mongo_data_path: PathBuf = base_path
                .join("resources")
                .join("mongo-data");

            if !mongo_data_path.exists() {
                fs::create_dir_all(&mongo_data_path).expect("Không thể tạo thư mục dữ liệu Mongo");
            }

            Command::new(mongo_path)
                .args([
                    "--dbpath",
                    mongo_data_path.to_str().unwrap(),
                    "--port",
                    "27018",
                ])
                .creation_flags(0x08000000) // 👈 Ẩn cửa sổ
                .spawn()
                .expect("Không thể khởi động MongoDB local");

            // --- Khởi động backend Golang ---
            let backend_path: PathBuf = base_path
                .join("resources")
                .join("bin")
                .join("backend.exe");

            Command::new(backend_path)
                .env("PORT", "4000")
                .env("MONGO_URL", "mongodb://localhost:27018")
                .env("MONGO_NAME", "test2")
                .env("JWT_SECRET", "test")
                .env("MINIO_ACCESS_KEY", "al8KsxHAbLtfNVsX")
                .env("MINIO_SECRET_KEY", "noWZ40KlvEcioZcPhLmMZFcPSkdeuX0K")
                .env("MINIO_ENDPOINT", "image.nghia.myds.me")
                .env("MINIO_BUCKET", "test")
                .env("MINIO_SSL", "true")
                .creation_flags(0x08000000) // 👈 Ẩn cửa sổ
                .spawn()
                .expect("Không thể khởi động Go backend");

            Ok(())
        })
        .on_window_event(move |_window, event| {
    if let tauri::WindowEvent::CloseRequested { .. } = event {
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "mongod.exe"])
            .spawn();

        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "backend.exe"])
            .spawn();
    }
})
        .run(tauri::generate_context!())
        .expect("error while running tauri app");
}
