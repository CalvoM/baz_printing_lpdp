use printing::lpd::client::LPDPClient;
use printing::lpd::errors::LPDPClientError;

#[tauri::command]
fn hello_world(name: String) -> String {
    format!("Hello there {name}")
}

#[tauri::command(rename_all = "snake_case")]
fn lpd_print_file(
    host: String,
    port: u32,
    queue: String,
    file_path: String,
) -> Result<(), LPDPClientError> {
    let mut lpd_client = LPDPClient::try_new(&queue, &host)?;
    lpd_client.send_printer_job(std::path::Path::new(&file_path))?;
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
fn lpd_query_queue(
    host: String,
    port: u32,
    queue: String,
    username: Option<String>,
    job_number: Option<String>,
) -> Result<String, LPDPClientError> {
    let mut lpd_client = LPDPClient::try_new(&queue, &host)?;
    Ok(lpd_client.request_queue_start_long(username, job_number)?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            hello_world,
            lpd_print_file,
            lpd_query_queue
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
