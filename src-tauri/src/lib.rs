mod core;
use core::test_dns_speed;

#[tauri::command]
async fn dns_speed_test(dns_servers: Vec<String>, domain: String) -> Vec<core::dns::DnsResult> {
    let mut handles = vec![];
    for dns in dns_servers {
        let dns_clone = dns.clone();
        let domain_clone = domain.clone();
        handles.push(tokio::spawn(async move {
            test_dns_speed(&dns_clone, &domain_clone).await
        }));
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
