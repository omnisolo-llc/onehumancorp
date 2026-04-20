package orchestration

import (
	"context"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"strings"
	"sync"
	"time"
)

// ResourceMonitor tracks local system resources.
type ResourceMonitor struct {
	mu             sync.Mutex
	lastSampleTime time.Time
	lastCPUTotal   uint64
	lastCPUIdle    uint64
}

// NewResourceMonitor initializes a new ResourceMonitor.
func NewResourceMonitor() *ResourceMonitor {
	rm := &ResourceMonitor{}
	rm.lastCPUTotal, rm.lastCPUIdle, _ = rm.getCPUStats()
	rm.lastSampleTime = time.Now()
	return rm
}

// GetCPUUsage returns the estimated CPU usage percentage.
func (rm *ResourceMonitor) GetCPUUsage(ctx context.Context) (float64, error) {
	if runtime.GOOS != "linux" {
		return 0, nil // Fallback for non-linux
	}

	total, idle, err := rm.getCPUStats()
	if err != nil {
		return 0, err
	}

	rm.mu.Lock()
	defer rm.mu.Unlock()

	diffTotal := total - rm.lastCPUTotal
	diffIdle := idle - rm.lastCPUIdle

	if diffTotal == 0 {
		return 0, nil
	}

	usage := float64(diffTotal-diffIdle) / float64(diffTotal) * 100

	rm.lastCPUTotal = total
	rm.lastCPUIdle = idle
	rm.lastSampleTime = time.Now()

	return usage, nil
}

// GetMemoryUsage returns the estimated memory usage percentage.
func (rm *ResourceMonitor) GetMemoryUsage(ctx context.Context) (float64, error) {
	if runtime.GOOS != "linux" {
		return 0, nil
	}

	contents, err := os.ReadFile("/proc/meminfo")
	if err != nil {
		return 0, err
	}

	var memTotal, memAvailable uint64
	lines := strings.Split(string(contents), "\n")
	for _, line := range lines {
		if strings.HasPrefix(line, "MemTotal:") {
			parts := strings.Fields(line)
			if len(parts) >= 2 {
				memTotal, _ = strconv.ParseUint(parts[1], 10, 64)
			}
		}
		if strings.HasPrefix(line, "MemAvailable:") {
			parts := strings.Fields(line)
			if len(parts) >= 2 {
				memAvailable, _ = strconv.ParseUint(parts[1], 10, 64)
			}
		}
	}

	if memTotal == 0 {
		return 0, fmt.Errorf("could not determine total memory")
	}

	usage := float64(memTotal-memAvailable) / float64(memTotal) * 100
	return usage, nil
}

func (rm *ResourceMonitor) getCPUStats() (uint64, uint64, error) {
	contents, err := os.ReadFile("/proc/stat")
	if err != nil {
		return 0, 0, err
	}

	lines := strings.Split(string(contents), "\n")
	for _, line := range lines {
		if strings.HasPrefix(line, "cpu ") {
			fields := strings.Fields(line)
			if len(fields) < 5 {
				return 0, 0, fmt.Errorf("invalid /proc/stat format")
			}

			var total uint64
			for i := 1; i < len(fields); i++ {
				val, _ := strconv.ParseUint(fields[i], 10, 64)
				total += val
			}
			idle, _ := strconv.ParseUint(fields[4], 10, 64)
			return total, idle, nil
		}
	}

	return 0, 0, fmt.Errorf("cpu info not found in /proc/stat")
}
