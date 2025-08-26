/// Commands that can be sent to the shared state actor.
/// This enum represents the different operations that can be performed on the actor's internal state.
pub enum SharedStateCommand {
    /// Increments the internal counter by 1.
    Increment,
    /// Retrieves the current value of the counter.
    /// The response is sent back through the provided oneshot channel.
    Get(tokio::sync::oneshot::Sender<u64>),
}

/// Starts a new shared state actor and returns a sender to communicate with it.
/// 
/// This function spawns a background task that maintains a counter state and processes
/// commands sent through the returned sender. The actor processes commands sequentially,
/// ensuring thread-safe access to the shared state.
///
/// Returns a `Sender` that can be used to send `SharedStateCommand`s to the actor.
pub async fn start() -> tokio::sync::mpsc::Sender<SharedStateCommand> {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let mut counter: u64 = 0;

    tokio::spawn(async move {
        while let Some(cmd) = rx.recv().await {
            match cmd {
                SharedStateCommand::Increment => {
                    counter += 1;
                }
                SharedStateCommand::Get(resp_tx) => {
                    let _ = resp_tx.send(counter);
                }
            }
        }
    });

    tx
}

/// Retrieves the current counter value from the shared state actor.
///
/// This function sends a `Get` command to the actor and waits for the response.
/// If the actor is no longer running or the response channel is closed, returns 0.
///
/// # Arguments
/// * `sender` - A reference to the sender channel connected to the actor
///
/// # Returns
/// The current value of the counter, or 0 if the operation fails.
pub async fn get_counter(sender: &tokio::sync::mpsc::Sender<SharedStateCommand>) -> u64 {
    let (resp_tx, resp_rx) = tokio::sync::oneshot::channel();
    let cmd = SharedStateCommand::Get(resp_tx);
    if sender.send(cmd).await.is_err() {
        return 0;
    }
    resp_rx.await.unwrap_or(0)
}

/// Increments the counter in the shared state actor by 1.
///
/// This function sends an `Increment` command to the actor. The operation is fire-and-forget,
/// meaning it doesn't wait for confirmation that the increment was processed.
///
/// # Arguments
/// * `sender` - A reference to the sender channel connected to the actor
pub async fn increment_counter(sender: &tokio::sync::mpsc::Sender<SharedStateCommand>) {
    let cmd = SharedStateCommand::Increment;
    let _ = sender.send(cmd).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_actor_starts_with_zero() {
        let sender = start().await;
        let count = get_counter(&sender).await;
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn test_increment_counter() {
        let sender = start().await;
        
        increment_counter(&sender).await;
        let count = get_counter(&sender).await;
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_multiple_increments() {
        let sender = start().await;
        
        for _ in 0..5 {
            increment_counter(&sender).await;
        }
        
        let count = get_counter(&sender).await;
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_concurrent_increments() {
        let sender = start().await;
        
        let mut handles = Vec::new();
        for _ in 0..10 {
            let sender_clone = sender.clone();
            let handle = tokio::spawn(async move {
                increment_counter(&sender_clone).await;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.await.unwrap();
        }
        
        let count = get_counter(&sender).await;
        assert_eq!(count, 10);
    }

    #[tokio::test]
    async fn test_concurrent_gets() {
        let sender = start().await;
        
        increment_counter(&sender).await;
        increment_counter(&sender).await;
        increment_counter(&sender).await;
        
        let mut handles = Vec::new();
        for _ in 0..5 {
            let sender_clone = sender.clone();
            let handle = tokio::spawn(async move {
                get_counter(&sender_clone).await
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let count = handle.await.unwrap();
            assert_eq!(count, 3);
        }
    }

    #[tokio::test]
    async fn test_mixed_operations() {
        let sender = start().await;
        
        increment_counter(&sender).await;
        let count1 = get_counter(&sender).await;
        assert_eq!(count1, 1);
        
        increment_counter(&sender).await;
        increment_counter(&sender).await;
        let count2 = get_counter(&sender).await;
        assert_eq!(count2, 3);
        
        increment_counter(&sender).await;
        let count3 = get_counter(&sender).await;
        assert_eq!(count3, 4);
    }

    #[tokio::test]
    async fn test_actor_survives_after_function_returns() {
        let sender = {
            let temp_sender = start().await;
            increment_counter(&temp_sender).await;
            temp_sender
        };
        
        sleep(Duration::from_millis(10)).await;
        
        let count = get_counter(&sender).await;
        assert_eq!(count, 1);
        
        increment_counter(&sender).await;
        let count = get_counter(&sender).await;
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_get_counter_with_closed_actor() {
        let sender = start().await;
        increment_counter(&sender).await;
        
        drop(sender);
        sleep(Duration::from_millis(10)).await;
        
        let sender2 = start().await;
        let count = get_counter(&sender2).await;
        assert_eq!(count, 0);
    }
}