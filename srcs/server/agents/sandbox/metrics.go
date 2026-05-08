package sandbox

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	SandboxViolationsTotal = promauto.NewCounter(
		prometheus.CounterOpts{
			Name: "ohc_sandbox_violations_total",
			Help: "Total number of sandbox violations detected",
		},
	)
)
