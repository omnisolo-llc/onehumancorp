package growth

import (
	"crypto/sha256"
	"encoding/binary"
	"sync"
)

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
		return "control"
	}

	hash := sha256.Sum256([]byte(id + userID))
	val := float64(binary.BigEndian.Uint64(hash[:8])) / float64(1<<64-1)

	if val < exp.TrafficSplit {
		return "treatment"
	}
	return "control"
}
