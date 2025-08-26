use flume::Receiver;
use crate::BATCH_SIZE;

pub async fn processor_1(
    id: usize,
    input: Receiver<u64>,
    output: flume::Sender<Box<Vec<u64>>>,
    report: flume::Sender<crate::reporter::Report>,
) {
    let batch_size = BATCH_SIZE.load(std::sync::atomic::Ordering::Relaxed);
    let mut batch = Box::new(Vec::with_capacity(batch_size));
    let mut start = std::time::Instant::now();
    let mut count = 0;
    while let Ok(data) = input.recv_async().await {
        count += 1;
        batch.push(data);
        let batch_size = BATCH_SIZE.load(std::sync::atomic::Ordering::Relaxed);
        if batch.len() >= batch_size {
            let r = output.send_async(batch).await;
            if r.is_err() {
                break;
            }
            batch = Box::new(Vec::with_capacity(batch_size));
        }
        let elapsed_seconds = start.elapsed().as_secs_f32();
        if elapsed_seconds >= 0.25 {
            let messages_per_second = count as f32 / elapsed_seconds;
            let _ = report.send_async(crate::reporter::Report::Layer1(id, messages_per_second)).await;
            count = 0;
            start = std::time::Instant::now();
        }
    }
    println!("Layer 1 processor exiting");
}