# Node runtime pins

The active harness worker and web image Dockerfiles use the same Node patch as
`.node-version` and `scripts/node-distributions.json`. The compatibility image
at `src/ui/next/Dockerfile` retains that pin too. `scripts/native-platform.test.mjs`
checks all five Node image stages. Package engine minimums describe compatibility
and need not equal the selected runtime patch.

On 2026-10-05, the [official Node image metadata](https://github.com/docker-library/official-images/blob/master/library/node)
listed `22.23.3-bookworm-slim` for Linux amd64, arm/v7, arm64/v8 and ppc64le.
The Docker registry manifest for `library/node:22.23.3-bookworm-slim` resolved to
`sha256:43ac6c60b8f89723f746e8a92ce91abd5017e627ce1ddfe4238355d3a30b772c`.
This records the inspected manifest; the Dockerfiles retain version tags rather
than immutable digest pins. Metadata verification does not replace image builds
and runtime acceptance on supported platforms.

Global npm tool installs inside the harness images have their own dependency
boundary. Updating Node does not change or audit those tool versions or their
transitive dependencies.
