#!/bin/bash
# Fixing the tests that are broken from previously on master
sed -i 's/app.Mux/app/g' srcs/server/dashboard/degradation_test.go
sed -i 's/app.authStore.GenerateToken/app.authStore.IssueToken/g' srcs/server/dashboard/degradation_test.go
sed -i 's/integrations.IntegrationStatus/domain.IntegrationStatus/g' srcs/server/dashboard/hybrid_mcp_bridge_test.go
sed -i 's/srv.handleMissionsSync/srv.handleHybridSyncMissions/g' srcs/server/dashboard/handlers_mcp_pii_test.go
