package profiling

import (
	"runtime"
	"time"
)

// Profile runs a function and returns the duration and memory allocation stats.
func Profile(f func()) (time.Duration, runtime.MemStats) {
	var m1, m2 runtime.MemStats
	runtime.ReadMemStats(&m1)
	start := time.Now()
	f()
	duration := time.Since(start)
	runtime.ReadMemStats(&m2)

	return duration, m2
}
