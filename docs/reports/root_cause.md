## Root Cause Analysis: Bazel Test Timeouts & Failures

During the initial triage, two primary issues blocked `bazelisk test //...` execution:

1. **`xds+` Network Fetch Failure**:
   The `xds` module resolution relies on downloading `https://github.com/cncf/xds/archive/555b57ec207be86f811fb0c04752db6f85e3d7e2.tar.gz`. The local sandbox/environment fails to resolve `codeload.github.com` (which GitHub redirects to for archive downloads), causing a fast-fail during the `http_archive` fetch.

2. **Bazel Test Timeouts**:
   When the fetch occasionally succeeds or hits the local cache, the full test suite (`//...`) times out after ~400 seconds. This is caused by massive action sizes when compiling heavy dependencies (e.g., `tokio` (547 files), `regex_automata` (98 files), `libsqlite3-sys` taking >30s alone), even when `--local_test_jobs=1` is configured. The CPU/memory constraints of the sandbox are overwhelmed by a full cold build + test of the workspace.

**Resolution / Pivot:**
As requested, we will not block on making the full Bazel build complete locally due to sandbox timeouts and network restrictions. Instead, we pivot immediately to Phase 2, focusing on K8s manifests, resource management (HPA/VPA), and observability optimization inside the `deploy/helm` chart.
