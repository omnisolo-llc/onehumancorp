#!/bin/bash
cd srcs/server/dashboard
# We fallback to simple tests in Go if Bazel encounters protobuf resolution issues with pure go test
# Let's run bazelisk test with --nobuild_tests_only to see if we can just test the server_test.go
