1. **Fix Path Traversal in LocalFSProvider**
   - Use `filepath.Clean` and check `filepath.Rel` to safely ensure path boundaries in `srcs/server/tools/hybridfsmcp/local_provider.go`.
   - Ensure trailing slash checks or cleaner boundary logic are used.
2. **Re-run tests**
   - Run `bazelisk test //srcs/server/tools/hybridfsmcp/...`
