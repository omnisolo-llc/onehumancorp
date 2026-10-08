e.

Rust dependency writes remain restricted to successful, positively allowed main/tag push or manual runs. npm downloads are saved only after successful locked installation on a main push/manual run; subsequent application tests can independently fail without invalidating those content-addressed downloads. Next compiler writes require a successful build on an allowed main push/manual run. PRs restore but do not write these caches; `pull_request_target` is not authorized by a negative-only event check. Automatic setup-node caching is disabled so there is no second writer outside this policy. The existing `cold_cache=true` input bypasses cache restores and saves, including the harness path. The first `npm-v2` / `next-v2` run is expected to miss; measure actual restoration/build times rather than claiming a speedup from configuration alone.

Docker keeps its existing Buildx layer cache and source-verified same-run image artifacts. Signing material, provider keys, `.env`, test sessions and live databases must never enter caches or artifacts. Cache keys need version/role changes when the build contract changes; do not save failed compiled builds or entire shared host directories.

## Disk, memory and iteration speed

This migration review found a 16 GB `target/debug/incremental` directory on a full 244 GB host disk. Removing only that generated cache recovered space without deleting source or compiled dependencies. Never use a broad `git clean` or remove a whole worktree to address a build cache issue.

Development incremental compilation stays available. Dev/test profiles explicitly share 256 codegen units and line-table debugging; test debug assertions and overflow checks remain enabled. Disabling incremental must not silently change a large crate to the non-incremental default codegen-unit count. The test profile disables incremental object graphs; the CI setup explicitly exports `CARGO_INCREMENTAL=0` because these graphs are not persisted. `CARGO_BUILD_JOBS=1` is u