package growth

import (
	"crypto/sha256"
	"encoding/binary"
	"sync"

	"github.com/prometheus/client_golang/prometheus"
)


var (
	experimentsCounter = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_growth_experiments_total",
			Help: "Total number of experiments evaluated",
		},
		[]string{"experiment_id", "variant"},
	)
)

func init() {
	prometheus.MustRegister(experimentsCounter)
}

type Experiment struct {
	ID           string
	Title        string
	TrafficSplit float64
}

type ExperimentManager struct {
	mu          sync.RWMutex
	experiments map[string]Experiment
}

func NewExperimentManager() *ExperimentManager {
	return &ExperimentManager{
		experiments: make(map[string]Experiment),
	}
}

func (em *ExperimentManager) AddExperiment(id, title string, split float64) {
	em.mu.Lock()
	defer em.mu.Unlock()
	em.experiments[id] = Experiment{
		ID:           id,
		Title:        title,
		TrafficSplit: split,
	}
}

func (em *ExperimentManager) GetVariant(id, userID string) string {
	em.mu.RLock()
	exp, ok := em.experiments[id]
	em.mu.RUnlock()

	if !ok {
		experimentsCounter.WithLabelValues(id, "control").Inc()
		return "control"
	}

	hash := sha256.Sum256([]byte(id + userID))
	val := float64(binary.BigEndian.Uint64(hash[:8])) / float64(1<<64-1)

	if val < exp.TrafficSplit {
		experimentsCounter.WithLabelValues(id, "treatment").Inc()
		return "treatment"
	}
	experimentsCounter.WithLabelValues(id, "control").Inc()
	return "control"
}
