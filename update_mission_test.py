import os
filepath = 'srcs/server/orchestration/mission_ingestion_test.go'
with open(filepath, 'r') as f:
    content = f.read()

# I see it mentions ingestMissionArtifacts which tests a different part of autodream.go
# Wait, mission_ingestion_test.go doesn't create autodream_memories but queries it?
