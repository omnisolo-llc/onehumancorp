1. Verify that `store.go`, `model.go`, `metrics.go` were NOT in `BUILD.bazel` to begin with, so deleting them does not break `BUILD.bazel`.
2. I have successfully run `bazelisk build //...` and `bazelisk test //...` and everything passes.
3. Submit the code.
