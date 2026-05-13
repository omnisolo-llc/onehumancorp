package queue

import (
	"context"
	"database/sql"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	_ "github.com/mattn/go-sqlite3"
	"github.com/redis/rueidis"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func setupSQLiteDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	require.NoError(t, err)
	return db
}

func TestSQLiteTaskQueue_EnqueueDequeue(t *testing.T) {
	db := setupSQLiteDB(t)
	defer db.Close()

	q, err := NewSQLiteTaskQueue(db)
	require.NoError(t, err)

	ctx := context.Background()

	// 1. Enqueue
	job := &Job{
		ParentTaskID: "parent-1",
		AgentRole:    "sales",
		Payload:      `{"task":"sell"}`,
	}
	err = q.Enqueue(ctx, job)
	require.NoError(t, err)
	require.NotEmpty(t, job.ID)

	// 2. Dequeue with wrong role
	dequeued, err := q.Dequeue(ctx, []string{"marketing"})
	require.NoError(t, err)
	assert.Nil(t, dequeued)

	// 3. Dequeue with correct role
	dequeued, err = q.Dequeue(ctx, []string{"sales"})
	require.NoError(t, err)
	require.NotNil(t, dequeued)
	assert.Equal(t, job.ID, dequeued.ID)
	assert.Equal(t, job.AgentRole, dequeued.AgentRole)

	// 4. Try dequeue again (should be locked)
	dequeued2, err := q.Dequeue(ctx, []string{"sales"})
	require.NoError(t, err)
	assert.Nil(t, dequeued2)
}

func TestSQLiteTaskQueue_Complete(t *testing.T) {
	db := setupSQLiteDB(t)
	defer db.Close()

	q, err := NewSQLiteTaskQueue(db)
	require.NoError(t, err)

	ctx := context.Background()
	job := &Job{AgentRole: "sales", Payload: `{}`}
	_ = q.Enqueue(ctx, job)
	dequeued, _ := q.Dequeue(ctx, []string{"sales"})

	// Complete
	err = q.Complete(ctx, dequeued.ID)
	require.NoError(t, err)

	// Verify status in DB
	var status string
	err = db.QueryRow("SELECT status FROM sub_agent_jobs WHERE id = ?", dequeued.ID).Scan(&status)
	require.NoError(t, err)
	assert.Equal(t, "COMPLETED", status)
}

func TestSQLiteTaskQueue_FailRetryAndPoison(t *testing.T) {
	db := setupSQLiteDB(t)
	defer db.Close()

	q, err := NewSQLiteTaskQueue(db)
	require.NoError(t, err)

	ctx := context.Background()
	job := &Job{AgentRole: "sales", Payload: `{}`}
	_ = q.Enqueue(ctx, job)

	// 1. Dequeue and fail (attempt 1)
	dequeued, _ := q.Dequeue(ctx, []string{"sales"})
	err = q.Fail(ctx, dequeued.ID, "temporary error")
	require.NoError(t, err)

	// It should be QUEUED again but run_after is in the future.
	var status string
	var runAfter time.Time
	var attempts int
	err = db.QueryRow("SELECT status, run_after, attempts FROM sub_agent_jobs WHERE id = ?", dequeued.ID).Scan(&status, &runAfter, &attempts)
	require.NoError(t, err)
	assert.Equal(t, "QUEUED", status)
	assert.Equal(t, 1, attempts)
	assert.True(t, runAfter.After(time.Now())) // runAfter should be in the future

	// Force run_after to now for testing
	_, err = db.Exec("UPDATE sub_agent_jobs SET run_after = CURRENT_TIMESTAMP WHERE id = ?", dequeued.ID)
	require.NoError(t, err)

	// 2. Dequeue and fail (attempt 2)
	dequeued, _ = q.Dequeue(ctx, []string{"sales"})
	err = q.Fail(ctx, dequeued.ID, "temporary error 2")
	require.NoError(t, err)
	_, _ = db.Exec("UPDATE sub_agent_jobs SET run_after = CURRENT_TIMESTAMP WHERE id = ?", dequeued.ID)

	// 3. Dequeue and fail (attempt 3)
	dequeued, _ = q.Dequeue(ctx, []string{"sales"})
	err = q.Fail(ctx, dequeued.ID, "fatal error")
	require.NoError(t, err)

	// 4. Verify it's dead-lettered
	err = db.QueryRow("SELECT status, attempts FROM sub_agent_jobs WHERE id = ?", dequeued.ID).Scan(&status, &attempts)
	require.NoError(t, err)
	assert.Equal(t, "FAILED", status)
	assert.Equal(t, 3, attempts)
}

func setupRedisQueue(t *testing.T) (*miniredis.Miniredis, *RedisTaskQueue) {
	s, err := miniredis.Run()
	require.NoError(t, err)

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{s.Addr()},
		DisableCache: true,
	})
	require.NoError(t, err)

	q := NewRedisTaskQueue(client, "test:")
	return s, q
}

func TestRedisTaskQueue_EnqueueDequeue(t *testing.T) {
	s, q := setupRedisQueue(t)
	defer s.Close()
	defer q.client.Close()

	ctx := context.Background()

	// 1. Enqueue
	job := &Job{
		ParentTaskID: "parent-1",
		AgentRole:    "sales",
		Payload:      `{"task":"sell"}`,
	}
	err := q.Enqueue(ctx, job)
	require.NoError(t, err)
	require.NotEmpty(t, job.ID)

	// 2. Dequeue with wrong role
	dequeued, err := q.Dequeue(ctx, []string{"marketing"})
	require.NoError(t, err)
	assert.Nil(t, dequeued)

	// 3. Dequeue with correct role
	dequeued, err = q.Dequeue(ctx, []string{"sales"})
	require.NoError(t, err)
	require.NotNil(t, dequeued)
	assert.Equal(t, job.ID, dequeued.ID)
	assert.Equal(t, job.AgentRole, dequeued.AgentRole)

	// 4. Try dequeue again (should be empty/locked)
	dequeued2, err := q.Dequeue(ctx, []string{"sales"})
	require.NoError(t, err)
	assert.Nil(t, dequeued2)
}

func TestRedisTaskQueue_Complete(t *testing.T) {
	s, q := setupRedisQueue(t)
	defer s.Close()
	defer q.client.Close()

	ctx := context.Background()
	job := &Job{AgentRole: "sales", Payload: `{}`}
	_ = q.Enqueue(ctx, job)
	dequeued, _ := q.Dequeue(ctx, []string{"sales"})

	// Complete
	err := q.Complete(ctx, dequeued.ID)
	require.NoError(t, err)

	// Verify not in running
	exists := s.Exists("test:running")
	// If it doesn't exist, it means either set is deleted or member deleted
	if exists {
		score, _ := s.ZScore("test:running", dequeued.ID)
		assert.Equal(t, float64(0), score, "member should be removed or score is 0 if removed")
	}
}

func TestRedisTaskQueue_FailRetryAndPoison(t *testing.T) {
	s, q := setupRedisQueue(t)
	defer s.Close()
	defer q.client.Close()

	ctx := context.Background()
	job := &Job{AgentRole: "sales", Payload: `{}`}
	_ = q.Enqueue(ctx, job)

	// 1. Dequeue and fail (attempt 1)
	dequeued, _ := q.Dequeue(ctx, []string{"sales"})
	err := q.Fail(ctx, dequeued.ID, "temporary error")
	require.NoError(t, err)

	// Because FastForward might be flaky with Redis ZRANGEBYSCORE in miniredis, we explicitly
	// retry dequeuing a few times in our test while advancing time

	// attempt 2
	for i := 0; i < 5; i++ {
		s.FastForward(20 * time.Second)
		dequeued, _ = q.Dequeue(ctx, []string{"sales"})
		if dequeued != nil {
			break
		}
	}
	require.NotNil(t, dequeued)
	err = q.Fail(ctx, dequeued.ID, "temporary error 2")
	require.NoError(t, err)

	// attempt 3
	for i := 0; i < 10; i++ {
		s.FastForward(50 * time.Second)
		dequeued, _ = q.Dequeue(ctx, []string{"sales"})
		if dequeued != nil {
			break
		}
	}
	require.NotNil(t, dequeued)
	err = q.Fail(ctx, dequeued.ID, "fatal error")
	require.NoError(t, err)

	// 4. Verify it's dead-lettered
	dequeuedDead, _ := q.Dequeue(ctx, []string{"sales"})
	assert.Nil(t, dequeuedDead)

	// Should be in dead set
	score, _ := s.ZScore("test:dead", dequeued.ID)
	assert.NotEqual(t, float64(0), score, "job should be in dead letter queue")
}
