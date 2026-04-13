1. **Fix Concurrency Panic in `LocalMesh.Publish`**
   - In `srcs/server/orchestration/mesh/local_mesh.go`, modify `Publish` to copy the `subs` slice while holding the read lock.
   - Example:
     ```go
     m.mu.RLock()
     subs, ok := m.topics[topic]
     var subsCopy []chan []byte
     if ok {
         subsCopy = make([]chan []byte, len(subs))
         copy(subsCopy, subs)
     }
     m.mu.RUnlock()
     // iterate over subsCopy
     ```
   - Also, need to handle "send on closed channel" properly if a channel is closed concurrently, or rely on the `subsCopy` and gracefully handle panic with recover or sync it. Actually, `localSubscription.Close()` closes the channel. We should just not close the channel in `Close()` or handle sending better. Wait, if `Close` removes the sub and closes the channel, `Publish` holding a stale reference might write to it. It's safer to have `Publish` just do a non-blocking send and `Close` to just remove the channel, letting garbage collection reap the channel, or ensure `Publish` uses `select` and a `closed` boolean to avoid sending to closed channels. Best practice: never close receiver channels if there are active senders. Let's just remove the `close(s.ch)` from `Close()`.

2. **Fix Resource Leaks on Context Cancellation**
   - In `LocalMesh.Subscribe`: Monitor `ctx.Done()` in the goroutine, and call `sub.Close()` when it fires to clean up the subscription.
   - In `RedisMesh.Subscribe`: The `go func()` loop should exit if `ctx.Done()` fires. Also, `pubsub.Close()` should be called when `ctx.Done()` fires to prevent connection leaks.

3. **Fix Memory Leak in `LocalMesh` Presence**
   - In `LocalMesh.GetActiveAgents`, while filtering active agents, explicitly `delete(m.presence, p.AgentID)` for stale entries to avoid an unbounded map size. To do this safely, `GetActiveAgents` will need to acquire `m.mu.Lock()` instead of `m.mu.RLock()` since it will modify the map.

4. **Verify Tests**
   - Run `go test -v .` locally and `~/go/bin/bazelisk test //...` on the modified code.

5. **Complete Pre-Commit Steps**
   - Run memory recording and submit.
