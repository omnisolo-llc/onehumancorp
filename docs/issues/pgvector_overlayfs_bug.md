# Title: pgvector container pull fails in bazel playwright tests on sandbox runners

## Problem
When running the `bazel test //...` suite, the `playwright_test` component that relies on the docker infrastructure fails to properly extract the `pgvector/pgvector:pg16` layer due to an overlayfs operation permissions issue during layer unpacking.

Error logged during the container start:
```
docker: failed to extract layer (application/vnd.oci.image.layer.v1.tar+gzip) to overlayfs as "extract-...": failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted
```

As a result, tests in the `//src/e2e` package cannot be run natively through the bazel layer on local instances or sandbox runners affected by this overlayfs restriction. Note that running tests manually outside the docker stack via `npx playwright test src/e2e/*.spec.ts` succeeds because it bypasses the need to rebuild the pg container, assuming an external pg environment or mocked layers are running locally.
