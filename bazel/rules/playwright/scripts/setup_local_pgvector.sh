#!/bin/bash
set -euo pipefail
mkdir -p /tmp/docker-build
cd /tmp/docker-build
cat << 'DOCKERFILE' > Dockerfile
FROM public.ecr.aws/docker/library/postgres:16-alpine
USER root
RUN apk add --no-cache git build-base clang19 llvm19
RUN git clone --branch v0.6.2 https://github.com/pgvector/pgvector.git \
    && cd pgvector \
    && make \
    && make install \
    && cd .. \
    && rm -rf pgvector
USER postgres
DOCKERFILE

if ! docker image inspect local-pgvector:16-alpine >/dev/null 2>&1; then
  docker build -t local-pgvector:16-alpine .
fi
