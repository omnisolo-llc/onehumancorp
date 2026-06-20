issue_title: "Fix Startup Blocker: Docker Registry Rate Limits & Data Limit Exceeded Errors"
issue_description: |
  ## Issue Title
  Fix Startup Blocker: Docker Registry Rate Limits & Data Limit Exceeded Errors

  ## Problem Statement
  When new developers or automated environments attempt to launch the OneHumanCorp stack locally using `docker compose -f deploy/docker-compose.yml up -d` or `bazelisk run //deploy:load_all_images`, the startup process is blocked by external container registry failures. Specifically, attempting to pull images like `public.ecr.aws/docker/library/redis:7-alpine` results in an `error from registry: Data limit exceeded` failure. This prevents the primary business logic components from initializing, blocking all subsequent local development, E2E testing, and UI gap analysis. For a non-technical owner or operator attempting to run a local instance or an engineer joining the project, this represents a hard failure state with no clear path forward.

  ## Research Report
  - **The Gap**: The system depends on external public container registries (e.g., AWS Public ECR, Docker Hub) for base images like Redis and Postgres. These registries enforce strict data transfer and pull rate limits. In CI or shared network environments, these limits are quickly exhausted, leading to unrecoverable `Data limit exceeded` errors.
  - **Competitor Analysis**: Enterprise platforms and robust open-source projects mitigate this by utilizing authenticated private registries (e.g., Google Artifact Registry, private ECR) or by implementing aggressive local caching mechanisms for base images.
  - **Observed Behavior**: Running `docker compose up --build` or `bazelisk run //deploy:load_all_images` locally fails consistently during the image pulling phase. The `redis:7-alpine` image fails to pull, preventing the Redis container from starting, which in turn causes the dependent Rust API server and background workers to crash or fail to connect.
  - **Proposed Fix**: The deployment architecture must be decoupled from rate-limited public registries. We need to mirror required base images to an authenticated, high-limit private registry or update the Bazel workspace and Docker Compose files to pull from an alternative, stable mirror.

  ## Design Doc
  ### Architecture Adjustments
  1.  **Registry Mirroring**: Set up a private or high-limit container registry mirror (e.g., Google Artifact Registry or an authenticated Docker Hub account) for all critical base images (`postgres:15-alpine`, `redis:7-alpine`, `alpine:3.19`).
  2.  **Configuration Update**: Update the `deploy/docker-compose.yml` and related Bazel `WORKSPACE`/`MODULE.bazel` files to reference these mirrored images instead of the public ECR/Docker Hub tags.
  3.  **Local Cache Fallback**: Implement a local tarball fallback mechanism within the Bazel build pipeline to load images directly from a cached artifact if the remote registry is unreachable or rate-limited.

  ### Mobile UX Flow
  - N/A for this infrastructure issue. This fixes a backend startup blocker.

  ### AI Agent Integration Points
  - N/A for this infrastructure issue.

  ## Implementation Prompt
  **Target Persona**: Developer / Infrastructure Engineer
  **Outcome**: The OHC stack can be consistently started locally using `docker compose up` or Bazel without encountering "Data limit exceeded" or rate-limiting errors from public container registries.

  **Acceptance Criteria**:
  1.  Update the Docker Compose and Bazel configurations to pull the Redis, Postgres, and Alpine base images from a reliable, rate-limit-free registry or mirror.
  2.  Verify that running `docker compose -f deploy/docker-compose.yml up -d` successfully starts all containers (DB, Redis, API, UI) on a fresh machine.
  3.  Verify that `bazelisk run //deploy:load_all_images` completes successfully without external registry download errors.

  ## Priority
  P0

  ## Estimated Scope
  Small
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
