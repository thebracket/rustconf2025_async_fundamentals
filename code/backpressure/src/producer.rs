use crate::reporter::Report;

/// Layer 1 data producer. Emits messages as fast as it can, and
/// reports the number of messages sent per second.
pub async fn producer_task(id: usize, tx: flume::Sender<u64>, report: flume::Sender<Report>) {
    let mut counter = id as u64;
    let mut start = std::time::Instant::now();
    let mut count = 0;
    loop {
        // Generate a pseudo-random number using a simple counter approach
        counter = counter.wrapping_mul(1103515245).wrapping_add(12345);
        let result = tx.send_async(counter).await;
        if result.is_err() {
            break;
        }
        count += 1;
        let elapsed_seconds = start.elapsed().as_secs_f32();
        if elapsed_seconds >= 0.1 {
            let messages_per_second = count as f32 / elapsed_seconds;
            let _ = report.send_async(Report::Producer(id, messages_per_second)).await;
            count = 0;
            start = std::time::Instant::now();
        }
    }
}