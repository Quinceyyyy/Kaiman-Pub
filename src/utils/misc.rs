use rand::Rng;
use tokio::time::{sleep, Duration};








pub async fn random_delay() {
    let delay_ms = rand::rng().random_range(2000..5000);
    sleep(Duration::from_millis(delay_ms)).await;
}
