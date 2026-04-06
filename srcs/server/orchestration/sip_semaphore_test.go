package orchestration

// ClearSemaphore drains the standalone throttle channel to prevent deadlocks during test execution.
func ClearSemaphore() {
	select {
	case <-standaloneThrottle:
	default:
	}
}
