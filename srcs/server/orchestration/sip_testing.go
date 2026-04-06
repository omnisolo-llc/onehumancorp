package orchestration

// ClearSemaphore clears the standalone throttle semaphore. Useful for tests to avoid deadlocks.
func ClearSemaphore() {
	select {
	case <-standaloneThrottle:
	default:
	}
}
