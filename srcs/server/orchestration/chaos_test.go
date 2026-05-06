package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"
    "fmt"

	"github.com/stretchr/testify/assert"
)

type faultInjectingCloudDB struct {
	PostgresTaskStore
	fails bool
}

func (f *faultInjectingCloudDB) CreateTask(ctx context.Context, task *SharedTask) error {
	if f.fails {
		return errors.New("simulated network failure")
	}
	return nil
}

func (f *faultInjectingCloudDB) GetTask(ctx context.Context, id string) (*SharedTask, error) {
	if f.fails {
		return nil, errors.New("simulated network failure")
	}
	return &SharedTask{ID: id, Status: "DONE"}, nil
}

func (f *faultInjectingCloudDB) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	if f.fails {
		return errors.New("simulated network failure")
	}
	return nil
}

func TestChaosSyncDaemonNetworkFailure(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &faultInjectingCloudDB{fails: true}

	task := &SharedTask{
		ID:             "task-chaos-1",
		OrganizationID: "org-chaos",
		Title:          "Chaos Task",
		Status:         "CLOUD_ESCALATION",
	}

	err := localStore.CreateTask(context.Background(), task)
	assert.NoError(t, err)

	err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
	assert.NoError(t, err)

	// Attempt sync with failing cloud DB
	err = syncPendingEscalations(context.Background(), localStore, cloudStore)
	assert.NoError(t, err) // Should not cascade failure, just log and continue

	// Verify local task status hasn't changed to CLOUD_PROCESSING because the cloud push failed
	localTask, err := localStore.GetTask(context.Background(), "task-chaos-1")
	assert.NoError(t, err)
	assert.Equal(t, "CLOUD_ESCALATION", localTask.Status)
}

func TestChaosSyncDaemonDegradation(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &faultInjectingCloudDB{fails: true}

	task := &SharedTask{
		ID:             "task-chaos-2",
		OrganizationID: "org-chaos",
		Title:          "Chaos Task 2",
		Status:         "CLOUD_PROCESSING",
	}

	err := localStore.CreateTask(context.Background(), task)
	assert.NoError(t, err)

	err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_PROCESSING")
	assert.NoError(t, err)

	// Attempt to pull completed escalations with failing cloud DB
	err = syncCompletedEscalations(context.Background(), localStore, cloudStore)
	assert.NoError(t, err) // circuit breaking prevents error bubble up

	// Verify local task status hasn't changed
	localTask, err := localStore.GetTask(context.Background(), "task-chaos-2")
	assert.NoError(t, err)
	assert.Equal(t, "CLOUD_PROCESSING", localTask.Status)
}

func TestChaosStressVerification(t *testing.T) {
	localDB := setupSyncTestDB(t)
	defer localDB.Close()

	cloudDB := setupSyncTestDB(t)
	defer cloudDB.Close()

	localStore := NewSqliteTaskStore(localDB)
	cloudStore := &mockPostgresProvider{NewSqliteTaskStore(cloudDB)}

    // Simulate concurrent load
    ctx, cancel := context.WithCancel(context.Background())
    defer cancel()

    // Create tasks BEFORE starting daemon to avoid race condition where db closes before it processes
    for i := 0; i < 100; i++ {
        task := &SharedTask{
            ID:             fmt.Sprintf("task-stress-%d", i),
            OrganizationID: "org-stress",
            Title:          "Stress Task",
            Status:         "CLOUD_ESCALATION",
        }
        err := localStore.CreateTask(context.Background(), task)
        assert.NoError(t, err)
        err = localStore.UpdateTaskStatus(context.Background(), task.ID, "CLOUD_ESCALATION")
        assert.NoError(t, err)
    }

    go StartSyncDaemon(ctx, localStore, cloudStore)

    time.Sleep(1 * time.Second)

    // Cancel the context so StartSyncDaemon stops its loop
    cancel()
    time.Sleep(100 * time.Millisecond) // brief wait to let it exit

    // Assert tasks successfully synced
    for i := 0; i < 100; i++ {
        task, err := localStore.GetTask(context.Background(), fmt.Sprintf("task-stress-%d", i))
        assert.NoError(t, err)
        if task != nil {
             assert.Equal(t, "CLOUD_PROCESSING", task.Status) // Assuming sync daemon successfully processed and marked them
        }

        // Ensure Cloud Store also received them
        cloudTask, err := cloudStore.GetTask(context.Background(), fmt.Sprintf("task-stress-%d", i))
        assert.NoError(t, err)
        if cloudTask != nil {
            assert.Equal(t, "PENDING", cloudTask.Status)
        }
    }
}
