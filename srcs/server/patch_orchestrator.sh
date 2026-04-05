#!/bin/bash
cat << 'PATCH' > orch.patch
--- srcs/server/orchestration/task_orchestrator.go
+++ srcs/server/orchestration/task_orchestrator.go
@@ -12,6 +12,7 @@
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/models"
+	"github.com/onehumancorp/mono/srcs/server/interop"
	"github.com/redis/rueidis"
 )

@@ -27,6 +28,7 @@
	hub         *CentrifugeNode
	mesh        TeammateMesh
	mu          sync.Mutex // For standalone mode coordination
+	distLock    interop.DistributedLock
	workerCtx   context.Context
	workerCancel context.CancelFunc
	workerWg    sync.WaitGroup
@@ -34,11 +36,16 @@

 func NewTaskOrchestrator(provider db.Provider, redisClient rueidis.Client, hub *CentrifugeNode, mesh TeammateMesh) TaskOrchestrator {
	ctx, cancel := context.WithCancel(context.Background())
+	dl, err := interop.NewDistributedLock()
+	if err != nil {
+		slog.Error("Failed to initialize DistributedLock", "error", err)
+	}
	to := &DefaultTaskOrchestrator{
		db:          provider,
		redisClient: redisClient,
		hub:         hub,
		mesh:        mesh,
+		distLock:    dl,
		workerCtx:   ctx,
		workerCancel: cancel,
	}
@@ -169,7 +176,14 @@
 }

 func (to *DefaultTaskOrchestrator) AcquireReadyTask(ctx context.Context, agentID string, capabilities []string) (*models.Task, error) {
-	if to.redisClient == nil {
+	if to.distLock != nil {
+		lockKey := "task_acquire_lock"
+		err := to.distLock.TryLock(ctx, lockKey, agentID, 10*time.Second)
+		if err != nil {
+			return nil, fmt.Errorf("could not acquire distributed lock: %w", err)
+		}
+		defer to.distLock.Unlock(ctx, lockKey, agentID)
+	} else if to.redisClient == nil {
		to.mu.Lock()
		defer to.mu.Unlock()
	}
PATCH
patch srcs/server/orchestration/task_orchestrator.go orch.patch
