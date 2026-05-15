pub struct InMemJobQueue {
    topics: DashMap<String, (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>)>,
}

impl InMemJobQueue {
    pub fn new() -> Self {
        InMemJobQueue {
            topics: DashMap::new(),
        }
    }

    fn get_or_create_topic(&self, topic: &str) -> (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>) {
        if let Some(t) = self.topics.get(topic) {
            return t.value().clone();
        }

        let (tx, rx) = mpsc::channel(10000);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let t = (tx, rx);
        self.topics.insert(topic.to_string(), t.clone());
        t
    }
}

#[async_trait]
impl JobQueue for InMemJobQueue {
    async fn push(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let (tx, _) = self.get_or_create_topic(topic);
        tx.send(payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pop(&self, topic: &str) -> Result<Vec<u8>, String> {
        let (_, rx) = self.get_or_create_topic(topic);
        let mut rx = rx.lock().await;
        rx.recv().await.ok_or_else(|| "channel closed".to_string())
    }
}
