import re

with open("srcs/server/api/sync_handler.go", "r") as f:
    content = f.read()

# I want to add `telemetry.RecordSyncConflictResolved(ctx)` and `telemetry.RecordOmniContextBytes(ctx, int64(len(p.Payload)))` inside the loop where UpsertMission succeeds.
# Wait, let's see sync_handler.go UpsertMission part.
