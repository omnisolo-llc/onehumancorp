package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestTokenBurnRateEngine(t *testing.T) {
	currentUsage := int64(100)
	trackerFunc := func() int64 {
		return currentUsage
	}

	engine := newTokenBurnRateEngine(time.Minute, trackerFunc)

	// Test calculation over a few polls
	// First poll, usage goes from 100 to 200 (100 diff per min)
	currentUsage = 200
	engine.calculateAndEmit(context.Background())
	assert.Equal(t, 1, len(engine.usageHistory))
	assert.Equal(t, float64(100), engine.usageHistory[0])

	// Second poll, usage goes from 200 to 400 (200 diff per min)
	currentUsage = 400
	engine.calculateAndEmit(context.Background())
	assert.Equal(t, 2, len(engine.usageHistory))
	assert.Equal(t, float64(200), engine.usageHistory[1])
}
