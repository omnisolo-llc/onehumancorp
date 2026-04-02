package orchestration

import (
	"context"
	"os"
	"sync"
	"sync/atomic"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/stretchr/testify/assert"
	_ "modernc.org/sqlite"
)

type slowMockSqliteProvider struct {
	db.Provider
	execCount int32
}

func (m *slowMockSqliteProvider) IsSQLite() bool { return true }
func (m *slowMockSqliteProvider) Exec(ctx context.Context, sql string, arguments ...interface{}) (int64, error) {
	atomic.AddInt32(&m.execCount, 1)
	time.Sleep(50 * time.Millisecond) // simulate slow write
	return 1, nil
}

func TestStandaloneSQLiteConcurrencyThrottling(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	mockProvider := &slowMockSqliteProvider{}
	sipdb := &SIPDB{db: mockProvider}

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	var wg sync.WaitGroup
	start := time.Now()

	// Launch 5 concurrent DelegateMission calls
	// Since throttle is 1 and it takes 50ms, the minimum time is 250ms
	// If they ran purely concurrently without throttle, it would take ~50ms
	numTasks := 5
	for i := 0; i < numTasks; i++ {
		wg.Add(1)
		go func(id int) {
			defer wg.Done()
			err := sipdb.DelegateMission(ctx, "mission", "role", Message{})
			assert.NoError(t, err)
		}(i)
	}

	wg.Wait()
	duration := time.Since(start)

	// Ensure that the minimum expected serialized time is roughly met
	assert.True(t, duration >= time.Duration(numTasks)*40*time.Millisecond, "Writes were not throttled, took %v", duration)
	assert.Equal(t, int32(numTasks), atomic.LoadInt32(&mockProvider.execCount))
}
