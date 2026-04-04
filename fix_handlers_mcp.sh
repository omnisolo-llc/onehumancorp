sed -i '/time.Since(time.Now())/c\				telemetry.RecordSyncLatency(ctx, time.Since(startSync))' srcs/server/dashboard/handlers_mcp.go
sed -i '/startSync := time.Now()/d' srcs/server/dashboard/handlers_mcp.go
sed -i 's/forceLocal := r.Header.Get("X-OHC-Conflict-Resolution") == "force-local"/\n\t\tforceLocal := r.Header.Get("X-OHC-Conflict-Resolution") == "force-local"\n\t\tstartSync := time.Now()/' srcs/server/dashboard/handlers_mcp.go
