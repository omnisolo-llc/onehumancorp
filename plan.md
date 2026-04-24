1. **Refactor `memoryLock` in `src/server/interop/lock.go` to use atomic `os.Mkdir` to avoid TOCTOU**:
   - Change `path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)` to use `os.Mkdir` instead of `os.OpenFile`.
   - The lock is acquired if `os.Mkdir(path, 0700)` succeeds. The lock details (expiry and token) are written to a file inside the directory: `filepath.Join(path, "lock.data")`.
   - To handle expired locks: If `os.IsExist(err)`, read the `lock.data`. If it has expired, atomically attempt to rename the directory to a temporary name (e.g., `tempPath = path + "_" + uuid.New().String() + ".tmp"`). If `os.Rename` succeeds, the lock was successfully "stolen" from the expired holder. Delete the renamed directory (`os.RemoveAll`) and retry `os.Mkdir`. If `os.Rename` fails, another process already stole it; retry or return false.
   - To handle unlocking: Atomically rename the directory to a temporary name. Read `lock.data` in the temporary directory. If the token matches, delete the temporary directory. If it doesn't match, put it back with `os.Rename(tempPath, path)`.

2. **Verify changes in `src/server/interop/lock.go`**:
   - Read `src/server/interop/lock.go` and confirm the new `os.Mkdir` logic has been written correctly without using `os.Remove(path)` directly on the locked path.

3. **Update tests in `src/server/interop/lock_test.go`**:
   - Run `sed -n 's/os.WriteFile(path/os.MkdirAll(path, 0700); os.WriteFile(filepath.Join(path, "lock.data")/g'` style changes or rewrite the mock cases in `lock_test.go` to simulate invalid files/directories properly based on the new `os.Mkdir` structure.
   - For example, in `TestMemoryLock_ExpiredLockOverwrite`, the code creates an artificially expired lock file: `os.WriteFile(path, []byte(expiry+",old_token"), 0666)`. It should instead do: `os.Mkdir(path, 0700)` and `os.WriteFile(filepath.Join(path, "lock.data"), ... )`. Same for `TestMemoryLock_FailuresAndPaths` and `TestMemoryLock_CoveragePaths`.
   - In `TestMemoryLock_CoveragePaths` we force `os.OpenFile` to fail by doing `os.Mkdir(path, 0755)`. To force `os.Mkdir` to fail with a different error, we can create a file with the directory path. `os.WriteFile(path, []byte(""), 0666)`.

4. **Verify tests pass**:
   - Run `bazelisk test //...` to verify everything works end-to-end.

5. **Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.**
