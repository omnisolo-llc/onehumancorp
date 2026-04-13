#!/bin/bash
sed -i '/"sync_escalation_handler.go",/a \        "kairos_stream.go",' srcs/server/api/BUILD.bazel
sed -i '/srcs = \["sync_escalation_handler_test.go"\],/c \    srcs = ["sync_escalation_handler_test.go", "kairos_stream_test.go"],' srcs/server/api/BUILD.bazel
