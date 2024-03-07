// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

#[tauri::command]
fn get_hardware_id() -> String {
    #[cfg(target_os = "windows")]
    {
        // Windows: Attempt to get the CPU ID
        let output = Command::new("wmic")
            .args(["CPU", "get", "ProcessorId"])
            .output()
            .expect("Failed to execute command");

        if let Ok(str_output) = String::from_utf8(output.stdout) {
            return str_output.trim().to_string();
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux: Attempt to get the system serial number via dmidecode
        let output = Command::new("sudo")
            .args(["dmidecode", "-s", "system-serial-number"])
            .output()
            .expect("Failed to execute command");

        if let Ok(str_output) = String::from_utf8(output.stdout) {
            return str_output.trim().to_string();
        }
    }

    #[cfg(target_os = "macos")]
    {
        // macOS: Attempt to get the hardware UUID
        let output = Command::new("system_profiler")
            .args(["SPHardwareDataType"])
            .output()
            .expect("Failed to execute command");

        if let Ok(str_output) = String::from_utf8(output.stdout) {
            for line in str_output.lines() {
                if line.contains("Hardware UUID") {
                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() > 1 {
                        return parts[1].trim().to_string();
                    }
                }
            }
        }
    }

    // Fallback if specific commands fail or on unsupported OS
    "unknown".to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_hardware_id])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
