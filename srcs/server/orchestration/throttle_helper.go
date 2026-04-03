package orchestration

// ClearThrottleSemaphore drains the package-level throttleSemaphore to prevent deadlocks between test runs.
func ClearThrottleSemaphore() {
	select {
	case <-throttleSemaphore:
	default:
	}
}
