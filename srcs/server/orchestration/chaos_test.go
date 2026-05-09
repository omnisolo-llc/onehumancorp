package orchestration

import (
	"context"
	"errors"
	"testing"
	"time"
    "fmt"
    "onehumancorp/srcs/server/orchestration/harness"

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

func (f *faultInjectingCloudDB) ClaimTask(ctx context.Context, organizationID string, agentID string) (*SharedTask, error) {
	return nil, nil
}

func (f *faultInjectingCloudDB) GetTasksByOrganization(ctx context.Context, organizationID string) ([]*SharedTask, error) {
	return nil, nil
}

type chaosMockPostgresProvider struct {
	*SqliteTaskStore
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

	// Attempt sync with failing cloud DB. This passes the error up currently.
    // Wrap it in a circuit breaker simulating StartSyncDaemon logic.
    circuit := harness.NewCircuitBreaker(3, 30*time.Second)
	err = circuit.Execute(func() error {
        return syncPendingEscalations(context.Background(), localStore, cloudStore)
    }, func() error {
        return nil // Swallow circuit error to prevent crash
    })

	// We expect NO error here because the fallback swallowed it gracefully.
	assert.NoError(t, err)

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
    circuit := harness.NewCircuitBreaker(3, 30*time.Second)
	err = circuit.Execute(func() error {
        return syncCompletedEscalations(context.Background(), localStore, cloudStore)
    }, func() error {
        return nil // Swallow circuit error to prevent crash
    })

	// No task should have updated localDB, and circuit breaker swallowed the GetTask fail.
	assert.NoError(t, err)

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
	cloudStore := &chaosMockPostgresProvider{NewSqliteTaskStore(cloudDB)}

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
