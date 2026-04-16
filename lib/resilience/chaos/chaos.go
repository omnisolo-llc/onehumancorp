package chaos

import (
	"context"
	"math/rand"
	"sync"
	"time"

	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	chaosInjections = promauto.NewCounterVec(
		prometheus.CounterOpts{
			Name: "chaos_injections_total",
			Help: "Total number of chaos injections applied",
		},
		[]string{"mode"},
	)
)

// ChaosMode defines the type of chaos to inject.
type ChaosMode int

const (
	// NoChaos means no disruption.
	NoChaos ChaosMode = iota
	// LatencySpike injects random delays.
	LatencySpike
	// ConnectionDrop returns errors simulating network drops.
	ConnectionDrop
	// ResourceExhaustion returns errors simulating CPU/Memory limits.
	ResourceExhaustion
)

// String returns the string representation of ChaosMode.
func (c ChaosMode) String() string {
	switch c {
	case NoChaos:
		return "no_chaos"
	case LatencySpike:
		return "latency_spike"
	case ConnectionDrop:
		return "connection_drop"
	case ResourceExhaustion:
		return "resource_exhaustion"
	default:
		return "unknown"
	}
}

// Injector is responsible for injecting chaos into operations.
type Injector struct {
	mode ChaosMode
	mu   sync.Mutex
	rand *rand.Rand
}

// NewInjector creates a new Chaos Injector.
func NewInjector(mode ChaosMode, seed int64) *Injector {
	return &Injector{
		mode: mode,
		rand: rand.New(rand.NewSource(seed)),
	}
}

// ChaosError is a custom error for chaos-induced failures.
type ChaosError struct {
	Message string
}

func (e *ChaosError) Error() string {
	return e.Message
}

// Inject applies the chaos mode to the current context.
func (i *Injector) Inject(ctx context.Context) error {
	chaosInjections.WithLabelValues(i.mode.String()).Inc()

	switch i.mode {
	case LatencySpike:
		i.mu.Lock()
		delay := time.Duration(i.rand.Intn(90)+10) * time.Millisecond
		i.mu.Unlock()
		select {
		case <-time.After(delay):
			return nil
		case <-ctx.Done():
			return ctx.Err()
		}
	case ConnectionDrop:
		i.mu.Lock()
		drop := i.rand.Float32() < 0.1
		i.mu.Unlock()
		if drop {
			return &ChaosError{Message: "chaos: simulated connection drop"}
		}
	case ResourceExhaustion:
		i.mu.Lock()
		exhaust := i.rand.Float32() < 0.05
		i.mu.Unlock()
		if exhaust {
			return &ChaosError{Message: "chaos: simulated resource exhaustion"}
		}
	}
	return nil
}
