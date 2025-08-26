use crate::{LAYER1_PERFORMANCE, LAYER2_PERFORMANCE, PRODUCER_PERFORMANCE};

pub enum Report {
    Producer(usize, f32),
    Layer1(usize, f32),
    Layer2(usize, f32),
}

pub async fn reporter_task(report_rx: flume::Receiver<Report>) {
    loop {
        match report_rx.recv_async().await {
            Ok(Report::Producer(id, messages_per_second)) => {
                //println!("Producer {} is sending {:.2} messages per second", id, messages_per_second);
                PRODUCER_PERFORMANCE[id].store(messages_per_second as u32, std::sync::atomic::Ordering::Relaxed);
            }
            Ok(Report::Layer1(id, messages_per_second)) => {
                //println!("Layer 1 processor {} is processing {:.2} messages per second", id, messages_per_second);
                LAYER1_PERFORMANCE[id].store(messages_per_second as u32, std::sync::atomic::Ordering::Relaxed);
            }
            Ok(Report::Layer2(id, messages_per_second)) => {
                //println!("Layer 2 processor {} is processing {:.2} messages per second", id, messages_per_second);
                LAYER2_PERFORMANCE[id].store(messages_per_second as u32, std::sync::atomic::Ordering::Relaxed);
            }
            Err(_) => {
                break;
            }
        }
    }
}