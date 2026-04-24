1. Modify `src/server/interop/lock.go`'s `memoryLock` to conform to the memory rules:
    - Hash the lock key via SHA-256 and hex encoding for cross-platform valid path.
    - Implement `os.Mkdir` instead of `os.OpenFile` + `os.O_EXCL` in local file-based lock.
    - Store the ownership token in a dynamically named file (`info_<token>.json`) inside the lock directory. The payload should store the expiry.
    - Fix the TOCTOU race conditions when unlocking by securely dealing with `info_<token>.json` rather than `os.RemoveAll` or `os.Rename`. Actually, for `Unlock`, we should just remove `info_<token>.json`. Wait, to unlock, we can read the directory contents, find our token file `info_<token>.json`, delete it, and then safely attempt to delete the directory (which will only succeed if it's empty, i.e., no other token file is there).
2. Modify `src/server/lib/resilience/mesh_fallback_chaos_test.go` to use `os.Mkdir` instead of `os.OpenFile` + `os.O_EXCL` and `os.Remove`. Since it's a test for lock contention, we must make it atomic with `os.Mkdir(lockDir, 0700)` and `os.Remove(lockDir)`.
3. Update `src/server/interop/lock_test.go` to adapt to the new locking scheme (SHA-256 hashed paths and `info_<token>.json` token file layout).
4. Run `bazelisk test //src/server/interop:interop_test` and `bazelisk test //src/server/lib/resilience/...` to ensure locking behavior is intact and race conditions are mitigated.
5. Run full test suite (`bazelisk test //... > test_output.log 2>&1 &`) to verify no regressions across the repository.
6. Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
7. Submit the changes with a clear description of the performance/latency improvement and latency chaos test fixes.
