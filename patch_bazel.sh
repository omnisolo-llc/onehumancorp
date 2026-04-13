#!/bin/bash
git checkout main -- srcs/server/dashboard/BUILD.bazel
sed -i '/"server.go",/a \        "kairos_stream.go",' srcs/server/dashboard/BUILD.bazel
sed -i '/"server_test.go",/a \        "kairos_stream_test.go",' srcs/server/dashboard/BUILD.bazel
sed -i '/"@io_opentelemetry_go_otel\/\/:\otel",/a \        "@com_github_gorilla_websocket\/\/:\websocket",' srcs/server/dashboard/BUILD.bazel
