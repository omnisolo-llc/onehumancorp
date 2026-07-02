issue_title: "Fix Local Docker Compose Environment: Valkey Container Fails to Start Due to OverlayFS Mount Error"
issue_description: |
  ## Problem Statement
  When attempting to launch the local development and testing stack via `docker compose -f deploy/docker-compose.yml up -d postgres valkey` (or the equivalent repository startup scripts), the stack fails to initialize correctly. Specifically, the `valkey` container (using the `redis:7-alpine` image) throws a fatal overlay filesystem mount error during container creation:

  `Error response from daemon: failed to mount /tmp/containerd-mount3735653149: mount source: "overlay", target: "/tmp/containerd-mount3735653149", fstype: overlay, flags: 0... err: invalid argument`

  This prevents any further work on the platform. The "Startup Exception Only" rule mandates that if the product cannot launch, the first gap is the startup blocker. We cannot perform dogfooding or testing via Playwright until this base infrastructure issue is resolved, as the backend `server` relies on a healthy `valkey` service (Redis replacement) to manage distributed locks and agent queues.

  ## Research Report
  ### Context & Auditing
  - **Live Service Check:** Attempted to start the real OHC stack using the required docker-compose commands. The `valkey` container failed to start completely, blocking the backend and UI from launching.
  - **Error Analysis:** The `invalid argument` error during the `overlay` mount usually indicates a corrupted Docker builder cache, a storage driver incompatibility in the CI/local environment, or an issue with the specific `redis:7-alpine` image layer extraction.
  - **Competitor/Industry Best Practices:** A robust local development environment should use reliable, easily-rebuildable images. If a specific alpine overlay is failing, switching to a more stable base image (e.g., `redis:7` or `valkey/valkey`) or clearing the local builder cache (`docker builder prune`) typically resolves this. Additionally, OHC should consider transitioning to the official `valkey/valkey` image since the service is named `valkey` but is pulling `redis:7-alpine`.

  ## Design Doc
  ### System Architecture
  ```mermaid
  graph TD
      subgraph Local Development Stack
          Docker[Docker Daemon]
          Valkey[Valkey Container]
          Server[OHC API Server]
      end

      Docker -->|Creates Container| Valkey
      Valkey -.->|OverlayFS Mount Fails| Docker
      Server -.->|Depends On| Valkey
  ```

  ### Mobile UX Flow
  - N/A (Infrastructure Fix). This fix is required to restore the local UI environment for testing.

  ### AI Agent Integration Points
  - The `valkey` service is critical for the `OHCJobQueue` and Agent Hub (Redis pub/sub). Without it, no agent coordination can happen.

  ### Key Decisions
  - Update `deploy/docker-compose.yml` to use a more stable image for the `valkey` service, such as the official `valkey/valkey` image or `redis:7`, to bypass the alpine overlay filesystem issue currently blocking the stack.

  ## Implementation Prompt
  **To the Implementer:**
  Your task is to fix the local Docker Compose startup blocker so that developers and the testing environment can successfully launch the stack.
  1. Investigate the `valkey` service definition in `deploy/docker-compose.yml`.
  2. Change the `image:` from `redis:7-alpine` to a stable alternative (e.g., `redis:7` or `valkey/valkey:latest`) that does not suffer from the overlayFS mount error in the current environment.
  3. Verify the fix by successfully running `docker compose -f deploy/docker-compose.yml up -d valkey postgres server` and ensuring all containers report as healthy.
  4. Ensure no existing E2E tests are broken by this underlying image change.

  ## Estimated Scope
  Small

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []