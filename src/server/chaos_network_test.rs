#[cfg(test)]
mod chaos_network_tests {
    use crate::db::{DB, DbStore};
    use std::sync::Arc;
    use tokio::net::TcpListener;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    #[tokio::test]
    async fn test_network_packet_drops_with_retry() {
        // Start a mock local echo server that drops some connections
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let _server_handle = tokio::spawn(async move {
            let mut i = 0;
            while let Ok((mut socket, _)) = listener.accept().await {
                i += 1;
                // Drop every 2nd connection
                if i % 2 == 0 {
                    continue; // connection dropped / closed
                }

                let mut buf = [0; 1024];
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if let Err(e) = socket.write_all(&buf[0..n]).await {
                            tracing::error!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                    Err(_) => {
                        return;
                    }
                }
            }
        });

        // Initialize a dummy DB wrapper just to test execute_with_retry
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        // Perform the execute_with_retry, which should eventually fail or succeed
        use std::sync::atomic::{AtomicUsize, Ordering};
        let attempt_counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = attempt_counter.clone();

        let res: Result<(), String> = db.execute_with_retry("network_test", move || {
            let counter = counter_clone.clone();
            async move {
                counter.fetch_add(1, Ordering::SeqCst);

                match tokio::net::TcpStream::connect(addr).await {
                    Ok(mut stream) => {
                        let msg = b"hello";
                        if let Err(_) = stream.write_all(msg).await {
                             return Err("network drop on write".to_string());
                        }

                        let mut buf = vec![0; 5];
                        if let Err(_) = stream.read_exact(&mut buf).await {
                             return Err("network drop on read".to_string());
                        }

                        if &buf == msg {
                            Ok(())
                        } else {
                            Err("data mismatch".to_string())
                        }
                    }
                    Err(_) => Err("network drop on connect".to_string()),
                }
            }
        }).await;

        assert!(res.is_ok(), "Operation should eventually succeed because execute_with_retry retries on failure. Result: {:?}", res);
        assert!(attempt_counter.load(Ordering::SeqCst) > 1, "It should take more than one attempt since 50% are dropped");

        // Now test explicitly simulating the 60 second ML-Resilience fallback timeout
        let res_timeout: Result<(), String> = db.execute_with_retry("network_timeout_test", || async {
            // Sleep longer than the 60s timeout
            tokio::time::sleep(std::time::Duration::from_secs(65)).await;
            Err("should not get here".to_string())
        }).await;

        assert!(res_timeout.is_err());
        assert!(res_timeout.unwrap_err().contains("timed out"), "Must fail with the ML-Resilience fallback timeout error");
    }
}
