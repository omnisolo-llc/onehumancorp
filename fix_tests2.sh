#!/bin/bash
git checkout srcs/server/dashboard/degradation_test.go srcs/server/dashboard/handlers_mcp_pii_test.go srcs/server/dashboard/hybrid_mcp_bridge_test.go srcs/server/dashboard/mesh_test.go srcs/server/dashboard/server_onboarding_test.go srcs/server/dashboard/server_stream_test.go srcs/server/dashboard/server_sync_rules_test.go
rm -f srcs/server/dashboard/degradation_test.go srcs/server/dashboard/handlers_mcp_pii_test.go srcs/server/dashboard/hybrid_mcp_bridge_test.go srcs/server/dashboard/mesh_test.go srcs/server/dashboard/server_onboarding_test.go srcs/server/dashboard/server_stream_test.go srcs/server/dashboard/server_sync_rules_test.go
/home/jules/go/bin/bazelisk run //:gazelle -- update srcs/server/dashboard/
