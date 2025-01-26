use anyhow::Result;
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use serde_json::Value;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    let compute_url_str = std::env::var("COMPUTE_URL")
        .unwrap_or_else(|_| "http://localhost:8080/api/compute".to_string());

    let concurrency = std::env::var("CONCURRENCY")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    let requests_per_task = std::env::var("REQUESTS_PER_TASK")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(100);

    println!("\n=== LOADTEST CONFIG ===");
    println!(" Target URL:       {}", compute_url_str);
    println!(" Concurrency:      {}", concurrency);
    println!(" Requests/Task:    {}", requests_per_task);

    // Arc<String> for concurrency
    let compute_url = Arc::new(compute_url_str);

    let success_count = Arc::new(AtomicUsize::new(0));
    let fail_count = Arc::new(AtomicUsize::new(0));
    let total_latency_ms = Arc::new(AtomicUsize::new(0));

    let client = Arc::new(Client::new());
    let mut handles = Vec::new();
    let start_time = Instant::now();

    for _ in 0..concurrency {
        let client_cloned = Arc::clone(&client);
        let url_cloned = Arc::clone(&compute_url);
        let success_cloned = Arc::clone(&success_count);
        let fail_cloned = Arc::clone(&fail_count);
        let lat_cloned = Arc::clone(&total_latency_ms);

        let handle = tokio::spawn(async move {
            for _ in 0..requests_per_task {
                let now = Instant::now();

                // Build some random JSON for realism
                let body = generate_random_record();

                // .post expects &str or IntoUrl, so deref the Arc:
                let endpoint = &*url_cloned;

                let response = client_cloned
                    .post(endpoint)
                    .json(&body)
                    .send()
                    .await;

                let elapsed_ms = now.elapsed().as_millis() as usize;
                match response {
                    Ok(resp) => {
                        if resp.status().is_success() {
                            success_cloned.fetch_add(1, Ordering::Relaxed);
                        } else {
                            fail_cloned.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                    Err(_) => {
                        fail_cloned.fetch_add(1, Ordering::Relaxed);
                    }
                }
                lat_cloned.fetch_add(elapsed_ms, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        let _ = h.await;
    }

    let duration = start_time.elapsed();
    let total_succ = success_count.load(Ordering::Relaxed);
    let total_fail = fail_count.load(Ordering::Relaxed);
    let total_req = total_succ + total_fail;
    let accumulated_latency = total_latency_ms.load(Ordering::Relaxed);

    let avg_latency = if total_req > 0 {
        (accumulated_latency as f64) / (total_req as f64)
    } else {
        0.0
    };

    println!("\n=== LOADTEST RESULTS ===");
    println!(" Elapsed:          {:.2?}", duration);
    println!(" Total Requests:   {}", total_req);
    println!(" Success:          {}", total_succ);
    println!(" Fail:             {}", total_fail);
    println!(" Avg Latency (ms): {:.2}", avg_latency);

    Ok(())
}

/// Example: random property-like JSON
fn generate_random_record() -> Value {
    let random_id: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect();

    let property_id = format!("RIV-{}", random_id);

    serde_json::json!({
        "property_id": property_id,
        "owner_name": "John Doe",
        "address": {
            "street": "123 Main St",
            "city": "Riverside",
            "state": "CA",
            "zip": "92501"
        },
        "assessed_value": 550000.0,
        "location_code": "RIV-CA",
        "is_overdue": false,
        "last_payment_unix": 1684224000
    })
}
