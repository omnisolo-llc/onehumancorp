# Known Issues

- **Bazel dependency issue with `xds`:** There is a known issue where `bazelisk test //...` fails because the xds repository archive (`https://github.com/cncf/xds/archive/555b57ec207be86f811fb0c04752db6f85e3d7e2.tar.gz`) returns a 404 Not Found. This seems to be due to an upstream repository change or removal of the commit.
- **Workaround:** As a temporary workaround, development and testing should be focused on targeted packages (e.g., `bazelisk test //src/server/orchestration/...`) to bypass the global dependency resolution failure during development until the upstream dependency tree can be fully repaired.
- E2E tests are currently failing locally due to a Docker overlayfs extraction error for the `pgvector/pgvector:pg16` image: `failed to convert whiteout file ... operation not permitted`. This relates to local Docker sandbox limitations and not the Go code logic.
