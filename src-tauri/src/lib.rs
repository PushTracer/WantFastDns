mod core;
use core::test_dns_speed;

#[tauri::command]
async fn dns_speed_test() -> Vec<core::dns::DnsResult> {
    let dns_servers = vec!["8.8.8.8:53", "1.1.1.1:53", "114.114.114.114:53"];
    let domain = "www.google.com";

    let mut handles = vec![];

    for dns in dns_servers {
        let dns = dns.to_string();
        let domain = domain.to_string();
        // spawn 异步任务
        handles.push(tokio::spawn(
            async move { test_dns_speed(&dns, &domain).await },
        ));
    }

    let mut results = vec![];
    for handle in handles {
        if let Ok(res) = handle.await {
            results.push(res);
        }
    }

    results
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![dns_speed_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
