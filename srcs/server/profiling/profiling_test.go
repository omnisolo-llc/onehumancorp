package profiling

import (
	"testing"
	"time"
)

func TestProfile(t *testing.T) {
	dur, stats := Profile(func() {
		time.Sleep(10 * time.Millisecond)
	})

	if dur < 10*time.Millisecond {
		t.Errorf("expected duration to be at least 10ms, got %v", dur)
	}
	if stats.Alloc == 0 {
		t.Logf("Stats: %+v", stats)
	}
}
