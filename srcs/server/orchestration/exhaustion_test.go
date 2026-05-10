package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

// A slow mock spawner that will simulate exhaustion
type slowMockSpawner struct {
	delay time.Duration
}

func (s *slowMockSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	return nil
}

func (s *slowMockSpawner) Monitor(ctx context.Context) error {
	return nil
}

func (s *slowMockSpawner) SpawnIsolated(ctx context.Context, job *Job) error {
	select {
	case <-time.After(s.delay):
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

type overridingSpawner struct {
	SubAgentSpawner
	fn func(context.Context, *Job) error
}

func (o *overridingSpawner) SpawnIsolated(ctx context.Context, job *Job) error {
	return o.fn(ctx, job)
}
func (o *overridingSpawner) Spawn(ctx context.Context, task *SharedTask) error { return nil }
func (o *overridingSpawner) Monitor(ctx context.Context) error { return nil }

func TestChaosHostResourceExhaustion_TimeoutAndRetryFallback(t *testing.T) {
	// Temporarily override the durations for faster testing
	oldTimeout := RetryTimeoutDuration
	oldBackoff := RetryBackoffDuration
	RetryTimeoutDuration = 10 * time.Millisecond
	RetryBackoffDuration = 10 * time.Millisecond
	defer func() {
		RetryTimeoutDuration = oldTimeout
		RetryBackoffDuration = oldBackoff
	}()

	job := &Job{ID: "exhaustion-job", TaskID: "exhaustion-task"}

	// Test timeout failure after 3 attempts
	err := RunWithTimeoutAndRetry(context.Background(), job, &slowMockSpawner{delay: 100 * time.Millisecond})
	assert.Error(t, err)
	assert.Equal(t, context.DeadlineExceeded, err)

	// Test success on the 2nd attempt if the host recovers
	attempts := 0
	os := &overridingSpawner{
		fn: func(ctx context.Context, j *Job) error {
			attempts++
			if attempts < 2 {
				select {
				case <-time.After(50 * time.Millisecond):
					return nil
				case <-ctx.Done():
					return ctx.Err()
				}
			}
			return nil
		},
	}

	// We need a slightly longer timeout so attempt 2 doesn't immediately fail.
	RetryTimeoutDuration = 30 * time.Millisecond
	err = RunWithTimeoutAndRetry(context.Background(), job, os)
	assert.NoError(t, err) // Successful on retry
	assert.Equal(t, 2, attempts)
}
