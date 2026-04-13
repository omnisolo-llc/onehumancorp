1. **Fix `redis_pubsub.go` Panic**:
   - The `unsubscribe` function in `RedisPubSub` shouldn't close `ch` directly if the goroutine might write to it. Instead, we can stop the goroutine by closing the `pubsub`, which will close `redisCh`, which will break the `for msg := range redisCh` loop, and we can safely close `ch` after the loop exits.
2. **Fix `ws.go` Resource Leak**:
   - The read pump must signal the write pump to exit when the WebSocket connection is closed. We can use a `done` channel or `defer cancel()` on a context. Since `r.Context()` doesn't get canceled on WS disconnect, we should create a sub-context `ctx, cancel := context.WithCancel(r.Context())` and call `cancel()` when the read pump exits.
3. **Verify Fixes**:
   - Run the tests again (`bazelisk test //api/mesh/...`).
4. **Pre-Commit Step**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
