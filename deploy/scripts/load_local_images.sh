#!/bin/bash
set -e

echo "Building and loading onehumancorp/server:latest locally..."
npx @bazel/bazelisk run //deploy:server_load

echo "Building and loading onehumancorp/agent:bazel locally..."
npx @bazel/bazelisk run //deploy:agent_load

echo "Building and loading onehumancorp/internal-default-agent:bazel locally..."
npx @bazel/bazelisk run //deploy:default_agent_load

echo "All required local images loaded successfully."
