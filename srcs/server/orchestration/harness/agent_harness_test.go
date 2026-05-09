package harness

import (
	"context"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestAgentHarness_Success(t *testing.T) {
	harness := NewAgentHarness()

	err := harness.ExecuteJob(context.Background(), func(ctx context.Context) error {
		return nil
	})

	assert.NoError(t, err)
}

func TestAgentHarness_Timeout(t *testing.T) {
	harness := &AgentHarness{
		timeout:    10 * time.Millisecond,
		maxRetries: 1,
	}

	err := harness.ExecuteJob(context.Background(), func(ctx context.Context) error {
		time.Sleep(50 * time.Millisecond)
		return nil
	})

	assert.Error(t, err)
	assert.Equal(t, "job timed out", err.Error())
}

func TestAgentHarness_Retry(t *testing.T) {
	harness := &AgentHarness{
		timeout:    100 * time.Millisecond,
		maxRetries: 3,
	}

	attempts := 0
	err := harness.ExecuteJob(context.Background(), func(ctx context.Context) error {
		attempts++
		if attempts < 3 {
			return errors.New("temporary error")
		}
		return nil
	})

	assert.NoError(t, err)
	assert.Equal(t, 3, attempts)
}
