# Local pgvector test workaround

In cases where `pgvector/pgvector:pg16` suffers from an `overlayfs` pull error on the Docker daemon (due to Docker Hub limits or an overlayfs bug on the sandbox runner), the test stack configuration has been modified to build and use `ghcr.io/tensorchord/pgvecto-rs:pg16-v0.2.0`. This image uses the public AWS ECR proxy for the base postgres image and compiles the vector extension directly via `make install`.

This bypasses both Docker Hub rate limiting and the overlayfs bug.
