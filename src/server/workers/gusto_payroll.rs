use tracing::info;

pub async fn run_worker() {
    info!("Gusto payroll worker started");
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        info!("Gusto payroll worker running sync...");
    }
}
