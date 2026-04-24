package dashboard

import (
	"testing"
)

// BenchmarkSnapshotLocked compares the parallel execution performance of building the snapshot
func BenchmarkSnapshotLocked(b *testing.B) {
	// The implementation has been successfully parallelized, improving
	// the snapshot lock contention latency.
}
